/// Axis-Aligned Bounding Box used for block collision and outline shapes.
///
/// Coordinates are in block-local space (0.0 to 1.0 for a standard block).
/// Values can extend beyond 0.0-1.0 for blocks like fences (collision height 1.5).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}

impl AABB {
    /// Creates a new AABB from min and max coordinates.
    #[must_use]
    pub const fn new(
        min_x: f32,
        min_y: f32,
        min_z: f32,
        max_x: f32,
        max_y: f32,
        max_z: f32,
    ) -> Self {
        Self {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        }
    }

    /// A full block (0,0,0) to (1,1,1).
    pub const FULL_BLOCK: AABB = AABB::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);

    /// An empty shape (no collision).
    pub const EMPTY: AABB = AABB::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    /// Returns true if this AABB has no volume.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.min_x >= self.max_x || self.min_y >= self.max_y || self.min_z >= self.max_z
    }

    /// Returns the width (X dimension) of this AABB.
    #[must_use]
    pub fn width(&self) -> f32 {
        self.max_x - self.min_x
    }

    /// Returns the height (Y dimension) of this AABB.
    #[must_use]
    pub fn height(&self) -> f32 {
        self.max_y - self.min_y
    }

    /// Returns the depth (Z dimension) of this AABB.
    #[must_use]
    pub fn depth(&self) -> f32 {
        self.max_z - self.min_z
    }
}

/// A VoxelShape is a collection of AABBs that define the shape of a block.
///
/// For simple blocks, this is typically a single AABB (full block or empty).
/// For complex blocks like stairs or fences, this is multiple AABBs combined.
pub type VoxelShape = &'static [AABB];

/// An ID referencing a registered VoxelShape in the ShapeRegistry.
///
/// Use this to refer to shapes in a compact way. The actual shape data
/// can be retrieved from the ShapeRegistry using this ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShapeId(pub u16);

impl ShapeId {
    /// The empty shape (no AABBs).
    pub const EMPTY: ShapeId = ShapeId(0);

    /// A full block shape.
    pub const FULL_BLOCK: ShapeId = ShapeId(1);
}

/// Registry for VoxelShapes.
///
/// Shapes are registered once and referenced by ShapeId. This allows
/// deduplication of shapes and compact storage of shape references.
///
/// Vanilla shapes are registered at startup. Plugins can register
/// additional shapes for custom blocks.
pub struct ShapeRegistry {
    shapes: Vec<&'static [AABB]>,
    allows_registering: bool,
}

impl Default for ShapeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ShapeRegistry {
    /// Creates a new shape registry with the standard empty and full block shapes.
    #[must_use]
    pub fn new() -> Self {
        let mut registry = Self {
            shapes: Vec::new(),
            allows_registering: true,
        };

        // Register the two standard shapes - IDs must match ShapeId::EMPTY and ShapeId::FULL_BLOCK
        let empty_id = registry.register(&[]);
        debug_assert_eq!(empty_id, ShapeId::EMPTY);

        let full_id = registry.register(FULL_BLOCK_SHAPE);
        debug_assert_eq!(full_id, ShapeId::FULL_BLOCK);

        registry
    }

    /// Registers a new shape and returns its ID.
    ///
    /// # Panics
    /// Panics if the registry has been frozen.
    pub fn register(&mut self, shape: &'static [AABB]) -> ShapeId {
        assert!(
            self.allows_registering,
            "Cannot register shapes after the registry has been frozen"
        );

        let id = ShapeId(self.shapes.len() as u16);
        self.shapes.push(shape);
        id
    }

    /// Gets the shape for a given ID.
    ///
    /// Returns an empty slice if the ID is invalid.
    #[must_use]
    pub fn get(&self, id: ShapeId) -> &'static [AABB] {
        self.shapes.get(id.0 as usize).copied().unwrap_or(&[])
    }

    /// Returns the number of registered shapes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.shapes.len()
    }

    /// Returns true if no shapes are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    /// Freezes the registry, preventing further registrations.
    pub fn freeze(&mut self) {
        self.allows_registering = false;
    }
}

// Static shape for full block - used during registry initialization
static FULL_BLOCK_SHAPE: &[AABB] = &[AABB::FULL_BLOCK];

/// Shape data for a block state, containing both collision and outline shapes.
#[derive(Debug, Clone, Copy)]
pub struct BlockShapes {
    pub collision: VoxelShape,
    pub outline: VoxelShape,
}

impl BlockShapes {
    /// Creates new block shapes.
    #[must_use]
    pub const fn new(collision: VoxelShape, outline: VoxelShape) -> Self {
        Self { collision, outline }
    }

    /// Full block collision and outline.
    pub const FULL_BLOCK: BlockShapes = BlockShapes::new(&[AABB::FULL_BLOCK], &[AABB::FULL_BLOCK]);

    /// Empty shapes (no collision, no outline).
    pub const EMPTY: BlockShapes = BlockShapes::new(&[], &[]);
}
