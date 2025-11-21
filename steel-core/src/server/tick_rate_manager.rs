use std::time::{Duration, Instant};

/// Manages the server tick rate, including freezing, stepping, and sprinting.
pub struct TickRateManager {
    /// The current tick rate in ticks per second.
    pub tick_rate: f32,
    /// The number of nanoseconds per tick based on the tick rate.
    pub nanoseconds_per_tick: u64,
    /// The current tick count.
    pub tick_count: u64,
    /// Whether the server is currently frozen.
    pub is_frozen: bool,
    /// The number of ticks to run while frozen (stepping).
    pub frozen_ticks_to_run: u64,
    /// Whether game elements should run this tick.
    pub run_game_elements: bool,

    // Sprinting
    /// The number of ticks remaining to sprint.
    pub remaining_sprint_ticks: u64,
    /// The total number of ticks scheduled for the current sprint.
    pub scheduled_current_sprint_ticks: u64,
    /// The start time of the current tick sprint.
    pub sprint_tick_start_time: Option<Instant>,
    /// The total time spent sprinting.
    pub sprint_time_spent: Duration,
    /// Whether the server was frozen before sprinting started.
    pub previous_is_frozen: bool,
}

impl TickRateManager {
    /// Creates a new `TickRateManager` with the default tick rate (20.0 TPS).
    #[must_use]
    pub fn new() -> Self {
        Self {
            tick_rate: 20.0,
            nanoseconds_per_tick: 50_000_000,
            tick_count: 0,
            is_frozen: false,
            frozen_ticks_to_run: 0,
            run_game_elements: true,
            remaining_sprint_ticks: 0,
            scheduled_current_sprint_ticks: 0,
            sprint_tick_start_time: None,
            sprint_time_spent: Duration::ZERO,
            previous_is_frozen: false,
        }
    }

    /// Sets the tick rate.
    pub fn set_tick_rate(&mut self, rate: f32) {
        self.tick_rate = rate.max(1.0);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        {
            self.nanoseconds_per_tick = (1_000_000_000.0 / f64::from(self.tick_rate)) as u64;
        }
    }

    /// Sets the frozen state of the server.
    pub fn set_frozen(&mut self, frozen: bool) {
        self.is_frozen = frozen;
    }

    /// Returns whether the server is frozen.
    #[must_use]
    pub fn is_frozen(&self) -> bool {
        self.is_frozen
    }

    /// Returns whether the server is currently stepping forward.
    #[must_use]
    pub fn is_stepping_forward(&self) -> bool {
        self.frozen_ticks_to_run > 0
    }

    /// Sets the number of ticks to step forward.
    pub fn set_frozen_ticks_to_run(&mut self, ticks: u64) {
        self.frozen_ticks_to_run = ticks;
    }

    /// Returns whether game elements should run this tick.
    #[must_use]
    pub fn runs_normally(&self) -> bool {
        self.run_game_elements
    }

    /// Updates the state for the current tick.
    pub fn tick(&mut self) {
        self.run_game_elements = !self.is_frozen || self.frozen_ticks_to_run > 0;
        if self.frozen_ticks_to_run > 0 {
            self.frozen_ticks_to_run -= 1;
        }
        if self.run_game_elements {
            self.tick_count += 1;
        }
    }

    // Sprinting logic

    /// Requests the game to sprint for a given number of ticks.
    /// Returns true if an existing sprint was interrupted.
    pub fn request_game_to_sprint(&mut self, ticks: u64) -> bool {
        let interrupted = self.remaining_sprint_ticks > 0;
        self.sprint_time_spent = Duration::ZERO;
        self.scheduled_current_sprint_ticks = ticks;
        self.remaining_sprint_ticks = ticks;
        self.previous_is_frozen = self.is_frozen;
        self.set_frozen(false);
        interrupted
    }

    /// Stops the current sprint.
    /// Returns true if a sprint was stopped.
    pub fn stop_sprinting(&mut self) -> bool {
        if self.remaining_sprint_ticks > 0 {
            self.finish_tick_sprint();
            true
        } else {
            false
        }
    }

    /// Checks if the server should sprint this tick.
    pub fn check_should_sprint_this_tick(&mut self) -> bool {
        if !self.run_game_elements {
            return false;
        }
        if self.remaining_sprint_ticks > 0 {
            self.sprint_tick_start_time = Some(Instant::now());
            self.remaining_sprint_ticks -= 1;
            true
        } else {
            self.finish_tick_sprint();
            false
        }
    }

    /// Ends the work for the current tick sprint.
    pub fn end_tick_work(&mut self) {
        if let Some(start) = self.sprint_tick_start_time {
            self.sprint_time_spent += start.elapsed();
            self.sprint_tick_start_time = None;
        }
    }

    /// Finishes the current tick sprint.
    fn finish_tick_sprint(&mut self) {
        // Here we would normally log the sprint report
        self.scheduled_current_sprint_ticks = 0;
        self.sprint_time_spent = Duration::ZERO;
        self.remaining_sprint_ticks = 0;
        self.set_frozen(self.previous_is_frozen);
    }

    /// Returns whether the server is currently sprinting.
    #[must_use]
    pub fn is_sprinting(&self) -> bool {
        self.scheduled_current_sprint_ticks > 0
    }
}

impl Default for TickRateManager {
    fn default() -> Self {
        Self::new()
    }
}
