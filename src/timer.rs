use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

const WORK_DURATION: Duration = Duration::from_secs(25 * 60);
const SHORT_BREAK_DURATION: Duration = Duration::from_secs(5 * 60);
const LONG_BREAK_DURATION: Duration = Duration::from_secs(15 * 60);

const WORK_LAPS: u8 = 10;
const SHORT_BREAK_LAPS: u8 = 3;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimerState {
    Idle,
    Work { lap: u8 },
    ShortBreak { lap: u8 },
    LongBreak,
    Paused(Box<TimerState>),
}

pub struct PomodoroTimer {
    pub state: TimerState,
    pub remaining: Duration,
    pub cycle_position: u8, // 0-4 for the 5-phase cycle
    last_tick: Option<Instant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerSnapshot {
    pub state: TimerState,
    pub remaining_secs: u64,
    pub session_name: String,
    pub session_progress: f64,
    pub is_paused: bool,
    pub cycle_position: u8,
}

impl PomodoroTimer {
    pub fn new() -> Self {
        Self {
            state: TimerState::Idle,
            remaining: Duration::ZERO,
            cycle_position: 0,
            last_tick: None,
        }
    }

    pub fn start(&mut self) {
        self.state = TimerState::Work { lap: 1 };
        self.remaining = WORK_DURATION;
        self.cycle_position = 0;
        self.last_tick = Some(Instant::now());
    }

    pub fn toggle_pause(&mut self) {
        match &self.state {
            TimerState::Paused(inner) => {
                self.state = *inner.clone();
                self.last_tick = Some(Instant::now());
            }
            TimerState::Idle => {}
            state => {
                self.state = TimerState::Paused(Box::new(state.clone()));
                self.last_tick = None;
            }
        }
    }

    pub fn reset_current_session(&mut self) {
        let inner_state = match &self.state {
            TimerState::Paused(inner) => inner.as_ref(),
            other => other,
        };

        let (new_state, duration) = match inner_state {
            TimerState::Work { .. } => (TimerState::Work { lap: 1 }, WORK_DURATION),
            TimerState::ShortBreak { .. } => (TimerState::ShortBreak { lap: 1 }, SHORT_BREAK_DURATION),
            TimerState::LongBreak => (TimerState::LongBreak, LONG_BREAK_DURATION),
            TimerState::Idle | TimerState::Paused(_) => return,
        };
        self.state = new_state;
        self.remaining = duration;
        self.last_tick = Some(Instant::now());
    }

    pub fn tick(&mut self) {
        if matches!(self.state, TimerState::Idle | TimerState::Paused(_)) {
            return;
        }

        if let Some(last) = self.last_tick {
            let elapsed = last.elapsed();
            self.last_tick = Some(Instant::now());

            if elapsed >= self.remaining {
                self.remaining = Duration::ZERO;
                self.advance_state();
            } else {
                self.remaining -= elapsed;
            }
        }
    }

    /// Advance to the next session (used when timer completes or Tab is pressed)
    /// Cycle: Work -> Short Break -> Work -> Short Break -> Work -> Short Break -> Work -> Long Break -> repeat
    pub fn advance_state(&mut self) {
        // Handle paused state - advance the inner state
        let current_state = match &self.state {
            TimerState::Paused(inner) => inner.as_ref().clone(),
            other => other.clone(),
        };

        match current_state {
            TimerState::Work { .. } => {
                // Work complete, move to break
                self.cycle_position += 1;
                if self.cycle_position >= 4 {
                    // After 4 work sessions, long break
                    self.state = TimerState::LongBreak;
                    self.remaining = LONG_BREAK_DURATION;
                } else {
                    self.state = TimerState::ShortBreak { lap: 1 };
                    self.remaining = SHORT_BREAK_DURATION;
                }
            }
            TimerState::ShortBreak { .. } => {
                // Short break complete, back to work
                self.state = TimerState::Work { lap: 1 };
                self.remaining = WORK_DURATION;
            }
            TimerState::LongBreak => {
                // Long break complete, reset cycle
                self.cycle_position = 0;
                self.state = TimerState::Work { lap: 1 };
                self.remaining = WORK_DURATION;
            }
            TimerState::Idle => {
                // Start fresh
                self.start();
                return;
            }
            TimerState::Paused(_) => unreachable!(),
        }
        self.last_tick = Some(Instant::now());
    }

    pub fn current_lap(&self) -> u8 {
        match &self.state {
            TimerState::Work { lap } => *lap,
            TimerState::ShortBreak { lap } => *lap,
            TimerState::Paused(inner) => match inner.as_ref() {
                TimerState::Work { lap } => *lap,
                TimerState::ShortBreak { lap } => *lap,
                _ => 0,
            },
            _ => 0,
        }
    }

    pub fn total_laps(&self) -> u8 {
        match &self.state {
            TimerState::Work { .. } => WORK_LAPS,
            TimerState::ShortBreak { .. } => SHORT_BREAK_LAPS,
            TimerState::Paused(inner) => match inner.as_ref() {
                TimerState::Work { .. } => WORK_LAPS,
                TimerState::ShortBreak { .. } => SHORT_BREAK_LAPS,
                _ => 0,
            },
            _ => 0,
        }
    }

    pub fn is_paused(&self) -> bool {
        matches!(self.state, TimerState::Paused(_))
    }

    pub fn session_name(&self) -> &'static str {
        match &self.state {
            TimerState::Idle => "Idle",
            TimerState::Work { .. } => "Work",
            TimerState::ShortBreak { .. } => "Short Break",
            TimerState::LongBreak => "Long Break",
            TimerState::Paused(inner) => match inner.as_ref() {
                TimerState::Work { .. } => "Work (Paused)",
                TimerState::ShortBreak { .. } => "Short Break (Paused)",
                TimerState::LongBreak => "Long Break (Paused)",
                _ => "Paused",
            },
        }
    }

    /// Progress within current session (0.0 to 1.0)
    pub fn session_progress(&self) -> f64 {
        let total = match &self.state {
            TimerState::Work { .. } => WORK_DURATION,
            TimerState::ShortBreak { .. } => SHORT_BREAK_DURATION,
            TimerState::LongBreak => LONG_BREAK_DURATION,
            TimerState::Paused(inner) => match inner.as_ref() {
                TimerState::Work { .. } => WORK_DURATION,
                TimerState::ShortBreak { .. } => SHORT_BREAK_DURATION,
                TimerState::LongBreak => LONG_BREAK_DURATION,
                _ => return 0.0,
            },
            TimerState::Idle => return 0.0,
        };

        1.0 - (self.remaining.as_secs_f64() / total.as_secs_f64())
    }

    pub fn snapshot(&self) -> TimerSnapshot {
        TimerSnapshot {
            state: self.state.clone(),
            remaining_secs: self.remaining.as_secs(),
            session_name: self.session_name().to_string(),
            session_progress: self.session_progress(),
            is_paused: self.is_paused(),
            cycle_position: self.cycle_position,
        }
    }
}
