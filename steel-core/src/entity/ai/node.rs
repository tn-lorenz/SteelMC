//! Vanilla pathfinding node primitives.

use glam::DVec3;
use rustc_hash::FxHashMap;
use steel_utils::BlockPos;

use crate::entity::ai::path::PathType;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    hash: i32,
    pub heap_idx: i32,
    pub g: f32,
    pub h: f32,
    pub f: f32,
    pub came_from: Option<i32>,
    pub closed: bool,
    pub walked_distance: f32,
    pub cost_malus: f32,
    pub path_type: PathType,
}

impl Node {
    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x,
            y,
            z,
            hash: Self::create_hash(x, y, z),
            heap_idx: -1,
            g: 0.0,
            h: 0.0,
            f: 0.0,
            came_from: None,
            closed: false,
            walked_distance: 0.0,
            cost_malus: 0.0,
            path_type: PathType::Blocked,
        }
    }

    #[must_use]
    pub const fn clone_and_move(&self, x: i32, y: i32, z: i32) -> Self {
        let mut node = Self::new(x, y, z);
        node.heap_idx = self.heap_idx;
        node.g = self.g;
        node.h = self.h;
        node.f = self.f;
        node.came_from = self.came_from;
        node.closed = self.closed;
        node.walked_distance = self.walked_distance;
        node.cost_malus = self.cost_malus;
        node.path_type = self.path_type;
        node
    }

    #[must_use]
    pub const fn create_hash(x: i32, y: i32, z: i32) -> i32 {
        let mut hash =
            ((y as u32) & 0xff) | (((x as u32) & 32_767) << 8) | (((z as u32) & 32_767) << 24);
        if x < 0 {
            hash |= 0x8000_0000;
        }
        if z < 0 {
            hash |= 32_768;
        }
        hash as i32
    }

    #[must_use]
    pub const fn hash(&self) -> i32 {
        self.hash
    }

    #[must_use]
    pub fn distance_to(&self, to: &Self) -> f32 {
        let xd = (to.x - self.x) as f32;
        let yd = (to.y - self.y) as f32;
        let zd = (to.z - self.z) as f32;
        xd.mul_add(xd, yd.mul_add(yd, zd * zd)).sqrt()
    }

    #[must_use]
    pub fn distance_to_pos(&self, pos: BlockPos) -> f32 {
        let xd = (pos.x() - self.x) as f32;
        let yd = (pos.y() - self.y) as f32;
        let zd = (pos.z() - self.z) as f32;
        xd.mul_add(xd, yd.mul_add(yd, zd * zd)).sqrt()
    }

    #[must_use]
    pub fn distance_to_sqr(&self, to: &Self) -> f32 {
        let xd = (to.x - self.x) as f32;
        let yd = (to.y - self.y) as f32;
        let zd = (to.z - self.z) as f32;
        xd.mul_add(xd, yd.mul_add(yd, zd * zd))
    }

    #[must_use]
    pub fn distance_manhattan(&self, to: &Self) -> f32 {
        (to.x - self.x).abs() as f32 + (to.y - self.y).abs() as f32 + (to.z - self.z).abs() as f32
    }

    #[must_use]
    pub const fn as_block_pos(&self) -> BlockPos {
        BlockPos::new(self.x, self.y, self.z)
    }

    #[must_use]
    pub fn as_vec3(&self) -> DVec3 {
        DVec3::new(f64::from(self.x), f64::from(self.y), f64::from(self.z))
    }

    #[must_use]
    pub const fn in_open_set(&self) -> bool {
        self.heap_idx >= 0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Target {
    node: Node,
    best_heuristic: f32,
    best_node: Option<i32>,
    reached: bool,
}

impl Target {
    #[must_use]
    pub const fn new(node: Node) -> Self {
        Self {
            node,
            best_heuristic: f32::MAX,
            best_node: None,
            reached: false,
        }
    }

    #[must_use]
    pub const fn node(&self) -> &Node {
        &self.node
    }

    pub fn update_best(&mut self, heuristic: f32, node: &Node) {
        if heuristic < self.best_heuristic {
            self.best_heuristic = heuristic;
            self.best_node = Some(node.hash());
        }
    }

    #[must_use]
    pub const fn best_node(&self) -> Option<i32> {
        self.best_node
    }

    pub const fn set_reached(&mut self) {
        self.reached = true;
    }

    #[must_use]
    pub const fn is_reached(&self) -> bool {
        self.reached
    }
}

#[derive(Debug, Default, Clone)]
pub struct NodeStore {
    nodes: FxHashMap<i32, Node>,
}

impl NodeStore {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn get_node(&mut self, x: i32, y: i32, z: i32) -> &mut Node {
        let hash = Node::create_hash(x, y, z);
        self.nodes.entry(hash).or_insert_with(|| Node::new(x, y, z))
    }

    #[must_use]
    pub fn get(&self, hash: i32) -> Option<&Node> {
        self.nodes.get(&hash)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, NodeStore, Target};

    #[test]
    fn node_hash_matches_vanilla_packing_shape() {
        assert_eq!(Node::create_hash(0, 0, 0), 0);
        assert_eq!(Node::create_hash(1, 64, 2), 0x0200_0140);
        assert_eq!(Node::create_hash(-1, 0, 0), 0x807f_ff00_u32 as i32);
        assert_eq!(Node::create_hash(0, 0, -1), 0xff00_8000_u32 as i32);
    }

    #[test]
    fn node_store_reuses_nodes_by_vanilla_hash() {
        let mut store = NodeStore::new();
        let hash = store.get_node(1, 64, 2).hash();
        store.get_node(1, 64, 2).cost_malus = 4.0;

        assert_eq!(store.len(), 1);
        assert_eq!(store.get(hash).map(|node| node.cost_malus), Some(4.0));
    }

    #[test]
    fn target_tracks_best_node_hash() {
        let from = Node::new(0, 64, 0);
        let better = Node::new(1, 64, 0);
        let worse = Node::new(4, 64, 0);
        let mut target = Target::new(Node::new(2, 64, 0));

        target.update_best(10.0, &worse);
        target.update_best(1.0, &better);
        target.update_best(5.0, &from);

        assert_eq!(target.best_node(), Some(better.hash()));
        assert!(!target.is_reached());
        target.set_reached();
        assert!(target.is_reached());
    }
}
