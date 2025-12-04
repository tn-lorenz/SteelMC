//! Bit packing utilities for chunk persistence.
//!
//! Uses only power-of-2 bit widths (1, 2, 4, 8, 16) to avoid entries spanning
//! u64 boundaries, which simplifies encoding/decoding and improves performance.

/// Calculates the number of bits needed to represent indices into a palette.
/// Only returns power-of-2 values: 1, 2, 4, 8, or 16.
///
/// Returns `None` for homogeneous containers (palette length 0 or 1).
#[must_use]
pub const fn bits_for_palette_len(palette_len: usize) -> Option<u8> {
    match palette_len {
        0..=1 => None, // Homogeneous, no bit array needed
        2 => Some(1),
        3..=4 => Some(2),
        5..=16 => Some(4),
        17..=256 => Some(8),
        _ => Some(16),
    }
}

/// Packs indices into a compact bit array using power-of-2 bit widths.
///
/// # Arguments
/// * `indices` - The indices to pack (values must fit in `bits` bits)
/// * `bits` - Bits per entry (must be 1, 2, 4, 8, or 16)
///
/// # Panics
/// Panics if `bits` is not a power of 2 or is greater than 16.
#[must_use]
pub fn pack_indices(indices: &[u32], bits: u8) -> Box<[u64]> {
    debug_assert!(
        bits.is_power_of_two() && bits <= 16,
        "bits must be 1, 2, 4, 8, or 16"
    );

    if indices.is_empty() {
        return Box::new([]);
    }

    let bits = bits as usize;
    let values_per_u64 = 64 / bits;
    let num_u64s = indices.len().div_ceil(values_per_u64);
    let mut data = vec![0u64; num_u64s];

    for (i, &index) in indices.iter().enumerate() {
        let array_index = i / values_per_u64;
        let offset = (i % values_per_u64) * bits;
        data[array_index] |= u64::from(index) << offset;
    }

    data.into_boxed_slice()
}

/// Unpacks indices from a compact bit array.
///
/// # Arguments
/// * `data` - The packed bit array
/// * `bits` - Bits per entry (must be 1, 2, 4, 8, or 16)
/// * `count` - Number of indices to unpack
///
/// # Panics
/// Panics if `bits` is not a power of 2 or is greater than 16.
#[must_use]
pub fn unpack_indices(data: &[u64], bits: u8, count: usize) -> Vec<u32> {
    debug_assert!(
        bits.is_power_of_two() && bits <= 16,
        "bits must be 1, 2, 4, 8, or 16"
    );

    if count == 0 {
        return Vec::new();
    }

    let bits = bits as usize;
    let values_per_u64 = 64 / bits;
    let mask = (1u64 << bits) - 1;
    let mut indices = Vec::with_capacity(count);

    for i in 0..count {
        let array_index = i / values_per_u64;
        let offset = (i % values_per_u64) * bits;
        let value = (data[array_index] >> offset) & mask;
        indices.push(value as u32);
    }

    indices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_for_palette_len() {
        assert_eq!(bits_for_palette_len(0), None);
        assert_eq!(bits_for_palette_len(1), None);
        assert_eq!(bits_for_palette_len(2), Some(1));
        assert_eq!(bits_for_palette_len(3), Some(2));
        assert_eq!(bits_for_palette_len(4), Some(2));
        assert_eq!(bits_for_palette_len(5), Some(4));
        assert_eq!(bits_for_palette_len(16), Some(4));
        assert_eq!(bits_for_palette_len(17), Some(8));
        assert_eq!(bits_for_palette_len(256), Some(8));
        assert_eq!(bits_for_palette_len(257), Some(16));
    }

    #[test]
    fn test_pack_unpack_roundtrip() {
        for bits in [1, 2, 4, 8, 16] {
            let max_value = (1u32 << bits) - 1;
            let indices: Vec<u32> = (0..100).map(|i| i % (max_value + 1)).collect();

            let packed = pack_indices(&indices, bits);
            let unpacked = unpack_indices(&packed, bits, indices.len());

            assert_eq!(indices, unpacked, "Failed for bits={}", bits);
        }
    }

    #[test]
    fn test_pack_4096_entries() {
        // Simulate a chunk section with 4096 blocks
        let indices: Vec<u32> = (0..4096).map(|i| (i % 16) as u32).collect();

        let packed = pack_indices(&indices, 4);
        assert_eq!(packed.len(), 4096 / 16); // 16 values per u64 with 4 bits

        let unpacked = unpack_indices(&packed, 4, 4096);
        assert_eq!(indices, unpacked);
    }
}
