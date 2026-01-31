pub mod themes;
pub mod digits;
pub mod digit_fonts;

pub use digit_fonts::DigitFont;

use std::time::{Duration, Instant};

use crate::timer::TimerState;
use themes::ThemeType;

/// Theme rotation interval: 2.5 minutes
const THEME_ROTATION_SECS: u64 = 150;

pub struct AnimationEngine {
    pub frame_index: usize,
    pub current_theme: ThemeType,
    pub current_font: DigitFont,
    last_frame_time: Instant,
    last_theme_change: Instant,
    fps: u8,
}

impl AnimationEngine {
    pub fn new() -> Self {
        Self {
            frame_index: 0,
            current_theme: ThemeType::random(),
            current_font: DigitFont::Block3D, // Start with the fancier font
            last_frame_time: Instant::now(),
            last_theme_change: Instant::now(),
            fps: 10,
        }
    }

    pub fn reset(&mut self) {
        self.frame_index = 0;
        self.last_frame_time = Instant::now();
        // Keep the current theme on reset
    }

    pub fn tick(&mut self, state: &TimerState, auto_rotate: bool) {
        let frame_duration = Duration::from_millis(1000 / self.fps as u64);

        if self.last_frame_time.elapsed() >= frame_duration {
            self.frame_index = self.frame_index.wrapping_add(1);
            self.last_frame_time = Instant::now();

            // Slower animation for breaks
            if matches!(state, TimerState::ShortBreak { .. }) {
                self.fps = 5;
            } else {
                self.fps = 10;
            }
        }

        // Check for automatic theme rotation (only if enabled)
        if auto_rotate && self.should_rotate_theme() {
            self.rotate_theme();
        }
    }

    /// Check if 2.5 minutes have elapsed since last theme change
    pub fn should_rotate_theme(&self) -> bool {
        self.last_theme_change.elapsed() >= Duration::from_secs(THEME_ROTATION_SECS)
    }

    /// Switch to a random different theme
    pub fn rotate_theme(&mut self) {
        self.current_theme = ThemeType::random_except(self.current_theme);
        self.last_theme_change = Instant::now();
    }

    /// Force a specific theme (useful for menu preview)
    pub fn set_theme(&mut self, theme: ThemeType) {
        self.current_theme = theme;
        self.last_theme_change = Instant::now();
    }

    /// Cycle to the next font style
    pub fn next_font(&mut self) {
        self.current_font = self.current_font.next();
    }

    /// Set a specific font style
    pub fn set_font(&mut self, font: DigitFont) {
        self.current_font = font;
    }
}
