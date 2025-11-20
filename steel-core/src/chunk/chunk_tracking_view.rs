use steel_utils::ChunkPos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkTrackingView {
    pub center: ChunkPos,
    pub view_distance: i32,
}

impl ChunkTrackingView {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            center: ChunkPos::new(0, 0),
            view_distance: 0,
        }
    }

    #[must_use]
    pub fn new(center: ChunkPos, view_distance: i32) -> Self {
        Self {
            center,
            view_distance,
        }
    }

    #[must_use]
    pub fn contains(&self, pos: ChunkPos) -> bool {
        let dx = (pos.0.x - self.center.0.x).abs();
        let dy = (pos.0.y - self.center.0.y).abs();
        dx <= self.view_distance && dy <= self.view_distance
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(ChunkPos),
    {
        for x in -self.view_distance..=self.view_distance {
            for z in -self.view_distance..=self.view_distance {
                f(ChunkPos::new(self.center.0.x + x, self.center.0.y + z));
            }
        }
    }

    pub fn difference(
        old: &Self,
        new: &Self,
        mut on_added: impl FnMut(ChunkPos),
        mut on_removed: impl FnMut(ChunkPos),
    ) {
        // Optimize: if disjoint, just remove all old and add all new
        // If overlapping, only process differences

        // For simplicity in first iteration, we can just iterate.
        // But O(N^2) is bad if we naive check.
        // Better: iterate old, if not in new -> removed.
        // Iterate new, if not in old -> added.
        // Since N is small (view distance <= 32), N^2 is ~4096.
        // A simple loop is fine.

        old.for_each(|pos| {
            if !new.contains(pos) {
                on_removed(pos);
            }
        });

        new.for_each(|pos| {
            if !old.contains(pos) {
                on_added(pos);
            }
        });
    }
}
