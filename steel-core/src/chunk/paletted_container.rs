//! A paletted container is a container that can be either homogeneous or heterogeneous.
use std::{
    fmt::Debug,
    hash::Hash,
    io::{Result, Write},
};

use steel_utils::{BlockStateId, codec::VarInt, serial::WriteTo};

/// A trait for converting a value to a global ID.
pub trait ToGlobalId {
    /// Converts the value to a global ID.
    fn to_global_id(&self) -> u32;
}

impl ToGlobalId for BlockStateId {
    fn to_global_id(&self) -> u32 {
        u32::from(self.0)
    }
}

impl ToGlobalId for u8 {
    fn to_global_id(&self) -> u32 {
        u32::from(*self)
    }
}

/// 3d array indexed by y,z,x
type Cube<T, const DIM: usize> = [[[T; DIM]; DIM]; DIM];

/// A heterogeneous palette container.
#[derive(Debug, Clone)]
pub struct HeterogeneousPalette<V: Hash + Eq + Copy, const DIM: usize> {
    cube: Box<Cube<V, DIM>>,
    // Keeps track of how many different times each value appears in the cube. (value, count)
    pub(crate) palette: Vec<(V, u16)>,
}

impl<V: Hash + Eq + Copy, const DIM: usize> HeterogeneousPalette<V, DIM> {
    fn get(&self, x: usize, y: usize, z: usize) -> V {
        debug_assert!(x < DIM);
        debug_assert!(y < DIM);
        debug_assert!(z < DIM);

        self.cube[y][z][x]
    }

    fn set(&mut self, x: usize, y: usize, z: usize, value: V) -> V {
        debug_assert!(x < DIM);
        debug_assert!(y < DIM);
        debug_assert!(z < DIM);

        let old_value = self.cube[y][z][x];

        if let Some((_, count)) = self.palette.iter_mut().find(|(v, _)| *v == value) {
            *count += 1;
        } else {
            self.palette.push((value, 1));
        }

        if let Some((index, (_, count))) = self
            .palette
            .iter_mut()
            .enumerate()
            .find(|(_, (v, _))| *v == old_value)
        {
            *count -= 1;
            if *count == 0 {
                self.palette.swap_remove(index);
            }
        }

        self.cube[y][z][x] = value;

        old_value
    }
}

/// A paletted container.
#[derive(Debug, Clone)]
pub enum PalettedContainer<V: Hash + Eq + Copy + Default, const DIM: usize> {
    /// A homogeneous container, where all values are the same.
    Homogeneous(V),
    /// A heterogeneous container, where values can be different.
    Heterogeneous(HeterogeneousPalette<V, DIM>),
}

enum PaletteMode {
    Linear,
    Hash,
    Global,
}

impl<V: Hash + Eq + Copy + Default + Debug, const DIM: usize> PalettedContainer<V, DIM> {
    /// The size of the container in one dimension.
    pub const SIZE: usize = DIM;
    /// The volume of the container.
    pub const VOLUME: usize = DIM * DIM * DIM;

    fn from_cube(cube: Box<Cube<V, DIM>>) -> Self {
        let mut palette: Vec<(V, u16)> = Vec::new();
        cube.iter().flatten().flatten().for_each(|v| {
            if let Some((_, count)) = palette.iter_mut().find(|(value, _)| value == v) {
                *count += 1;
            } else {
                palette.push((*v, 1));
            }
        });

        if palette.len() == 1 {
            Self::Homogeneous(palette[0].0)
        } else {
            Self::Heterogeneous(HeterogeneousPalette { cube, palette })
        }
    }

    /// Gets the value at the given coordinates.
    pub fn get(&self, x: usize, y: usize, z: usize) -> V {
        match self {
            Self::Homogeneous(value) => *value,
            Self::Heterogeneous(data) => data.get(x, y, z),
        }
    }

    /// Sets the value at the given coordinates.
    pub fn set(&mut self, x: usize, y: usize, z: usize, value: V) -> V {
        debug_assert!(x < Self::SIZE);
        debug_assert!(y < Self::SIZE);
        debug_assert!(z < Self::SIZE);

        match self {
            Self::Homogeneous(original) => {
                let original = *original;
                if value != original {
                    let mut cube = Box::new([[[original; DIM]; DIM]; DIM]);
                    cube[y][z][x] = value;
                    *self = Self::from_cube(cube);
                }
                original
            }
            Self::Heterogeneous(data) => {
                let original = data.set(x, y, z, value);
                if data.palette.len() == 1 {
                    *self = Self::Homogeneous(data.palette[0].0);
                }
                original
            }
        }
    }

    /// Writes the container to the given writer.
    ///
    /// # Errors
    /// - If the writer fails to write.
    pub fn write(&self, writer: &mut impl Write) -> Result<()>
    where
        V: ToGlobalId,
    {
        match self {
            Self::Homogeneous(value) => {
                0u8.write(writer)?;
                VarInt(value.to_global_id() as i32).write(writer)?;
                VarInt(0).write(writer)?;
            }
            Self::Heterogeneous(data) => {
                let (bits, mode) = Self::calculate_strategy(data.palette.len());

                // Write bits per entry
                bits.write(writer)?;

                // Write Palette
                match mode {
                    PaletteMode::Linear | PaletteMode::Hash => {
                        VarInt(data.palette.len() as i32).write(writer)?;
                        for (val, _) in &data.palette {
                            VarInt(val.to_global_id() as i32).write(writer)?;
                        }
                    }
                    PaletteMode::Global => {}
                }

                // Pack data
                let indices: Vec<u32> = data
                    .cube
                    .iter()
                    .flatten()
                    .flatten()
                    .map(|val| {
                        if matches!(mode, PaletteMode::Global) {
                            val.to_global_id()
                        } else {
                            data.palette.iter().position(|(v, _)| v == val).unwrap() as u32
                        }
                    })
                    .collect();

                let packed = pack_bits(&indices, bits as usize);

                // Write data
                for long in packed {
                    long.write(writer)?;
                }
            }
        }
        Ok(())
    }

    fn calculate_strategy(count: usize) -> (u8, PaletteMode) {
        if DIM == 16 {
            // Block states
            match count {
                0..=1 => unreachable!("Homogeneous handled separately"),
                2..=16 => (4, PaletteMode::Linear),
                17..=32 => (5, PaletteMode::Hash),
                33..=64 => (6, PaletteMode::Hash),
                65..=128 => (7, PaletteMode::Hash),
                129..=256 => (8, PaletteMode::Hash),
                _ => (15, PaletteMode::Global), // ceil(log2(max_block_state_id)) approx 15
            }
        } else {
            // Biomes
            match count {
                0..=1 => unreachable!("Homogeneous handled separately"),
                2 => (1, PaletteMode::Linear),
                3..=4 => (2, PaletteMode::Linear),
                5..=8 => (3, PaletteMode::Hash),
                _ => (6, PaletteMode::Global), // ceil(log2(max_biome_id)) approx 6
            }
        }
    }
}

fn pack_bits(indices: &[u32], bits: usize) -> Vec<u64> {
    let values_per_long = 64 / bits;
    let len = (indices.len() + values_per_long - 1) / values_per_long;
    let mut data = vec![0u64; len];

    for (i, &index) in indices.iter().enumerate() {
        let array_index = i / values_per_long;
        let offset = (i % values_per_long) * bits;

        // In `SimpleBitStorage`, values are packed from right to left within the long?
        // Checking SimpleBitStorage.java:
        // packedValue |= values[inputOffset + indexInLong] & this.mask;
        // where indexInLong goes from valuesPerLong-1 down to 0.
        // Wait, the loop is:
        // for (int indexInLong = this.valuesPerLong - 1; indexInLong >= 0; indexInLong--) {
        //    packedValue <<= bits;
        //    packedValue |= values[inputOffset + indexInLong] & this.mask;
        // }
        // This means the LAST value in the chunk (high index) is in the LEAST significant bits of the long?
        // NO.
        // `packedValue` is shifted left.
        // Initial packedValue = 0.
        // indexInLong = 3 (say).
        // packedValue = (packedValue << bits) | val[3]
        // packedValue = val[3]
        // indexInLong = 2.
        // packedValue = (val[3] << bits) | val[2]
        // indexInLong = 0.
        // packedValue = (... | val[1] << bits) | val[0]
        //
        // So val[0] (the FIRST value in the block) ends up in the LEAST significant bits.
        // val[valuesPerLong-1] ends up in the MOST significant bits.
        //
        // So: data[array_index] |= (index as u64) << offset;
        // where offset = (i % values_per_long) * bits;
        //
        // Let's trace:
        // i = 0. array_index = 0. offset = 0. data[0] |= index << 0. Correct.
        // i = 1. array_index = 0. offset = bits. data[0] |= index << bits. Correct.

        data[array_index] |= (index as u64) << offset;
    }

    data
}

/// A palette container for blocks.
pub type BlockPalette = PalettedContainer<BlockStateId, 16>;
/// A palette container for biomes.
pub type BiomePalette = PalettedContainer<u8, 4>;

impl BlockPalette {
    /// Gets the number of non-empty blocks in the container.
    #[must_use]
    pub fn non_empty_block_count(&self) -> u16 {
        match self {
            Self::Homogeneous(v) => {
                if v.0 == 0 {
                    0
                } else {
                    #[allow(clippy::cast_possible_truncation)]
                    {
                        Self::VOLUME as u16
                    }
                }
            }
            Self::Heterogeneous(data) => {
                let mut count = 0;
                for (v, c) in &data.palette {
                    if v.0 != 0 {
                        count += c;
                    }
                }
                count
            }
        }
    }
}
