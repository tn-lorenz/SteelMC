//! Path navigation state shell.

use glam::DVec3;
use steel_utils::BlockPos;

#[derive(Debug, Clone, PartialEq)]
pub struct PathNavigation {
    target_pos: Option<BlockPos>,
    speed_modifier: f64,
    tick: i32,
    done: bool,
}

impl PathNavigation {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            target_pos: None,
            speed_modifier: 0.0,
            tick: 0,
            done: true,
        }
    }

    #[must_use]
    pub const fn target_pos(&self) -> Option<BlockPos> {
        self.target_pos
    }

    #[must_use]
    pub const fn speed_modifier(&self) -> f64 {
        self.speed_modifier
    }

    #[must_use]
    pub const fn is_done(&self) -> bool {
        self.done
    }

    pub const fn tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    pub const fn stop(&mut self) {
        self.target_pos = None;
        self.speed_modifier = 0.0;
        self.done = true;
    }

    pub const fn set_direct_target(&mut self, target: DVec3, speed_modifier: f64) {
        self.target_pos = Some(BlockPos::new(
            target.x.floor() as i32,
            target.y.floor() as i32,
            target.z.floor() as i32,
        ));
        self.speed_modifier = speed_modifier;
        self.done = false;
    }
}

impl Default for PathNavigation {
    fn default() -> Self {
        Self::new()
    }
}
