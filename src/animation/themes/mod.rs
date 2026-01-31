pub mod matrix;
pub mod fire;
pub mod starfield;
pub mod plasma;
pub mod rain;
pub mod waves;
pub mod shapes;
pub mod fireworks;

use ratatui::prelude::*;

/// All available animation themes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeType {
    Matrix,
    Fire,
    Starfield,
    Plasma,
    RainDrops,
    RadioWaves,
    SpinningShapes,
    Fireworks,
}

impl ThemeType {
    /// Get all theme variants
    pub fn all() -> &'static [ThemeType] {
        &[
            ThemeType::Matrix,
            ThemeType::Fire,
            ThemeType::Starfield,
            ThemeType::Plasma,
            ThemeType::RainDrops,
            ThemeType::RadioWaves,
            ThemeType::SpinningShapes,
            ThemeType::Fireworks,
        ]
    }

    /// Pick a random theme (different from current)
    pub fn random_except(current: ThemeType) -> ThemeType {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as usize;

        let themes = Self::all();
        let mut idx = seed % themes.len();

        // Make sure we get a different theme
        while themes[idx] == current {
            idx = (idx + 1) % themes.len();
        }
        themes[idx]
    }

    /// Pick a random theme
    pub fn random() -> ThemeType {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as usize;

        let themes = Self::all();
        themes[seed % themes.len()]
    }

    /// Theme display name
    pub fn name(&self) -> &'static str {
        match self {
            ThemeType::Matrix => "Matrix Rain",
            ThemeType::Fire => "Fire",
            ThemeType::Starfield => "Starfield",
            ThemeType::Plasma => "Plasma",
            ThemeType::RainDrops => "Rain Drops",
            ThemeType::RadioWaves => "Radio Waves",
            ThemeType::SpinningShapes => "Spinning Shapes",
            ThemeType::Fireworks => "Fireworks",
        }
    }

    /// Render the animation background for this theme
    pub fn render_background(&self, frame: &mut Frame, area: Rect, frame_index: usize) {
        match self {
            ThemeType::Matrix => matrix::render_background(frame, area, frame_index),
            ThemeType::Fire => fire::render_background(frame, area, frame_index),
            ThemeType::Starfield => starfield::render_background(frame, area, frame_index),
            ThemeType::Plasma => plasma::render_background(frame, area, frame_index),
            ThemeType::RainDrops => rain::render_background(frame, area, frame_index),
            ThemeType::RadioWaves => waves::render_background(frame, area, frame_index),
            ThemeType::SpinningShapes => shapes::render_background(frame, area, frame_index),
            ThemeType::Fireworks => fireworks::render_background(frame, area, frame_index),
        }
    }

    /// Get the primary color for this theme (used for digits)
    pub fn primary_color(&self) -> Color {
        match self {
            ThemeType::Matrix => Color::Rgb(0, 255, 65),       // Bright green
            ThemeType::Fire => Color::Rgb(255, 200, 50),       // Yellow-orange
            ThemeType::Starfield => Color::Rgb(200, 200, 255), // Pale blue-white
            ThemeType::Plasma => Color::Rgb(255, 100, 255),    // Magenta
            ThemeType::RainDrops => Color::Rgb(100, 200, 255), // Cyan
            ThemeType::RadioWaves => Color::Rgb(0, 255, 255),  // Neon cyan
            ThemeType::SpinningShapes => Color::Rgb(255, 255, 100), // Yellow
            ThemeType::Fireworks => Color::Rgb(255, 220, 100), // Gold
        }
    }

    /// Get the secondary color for this theme (used for digit shadows/outlines)
    pub fn secondary_color(&self) -> Color {
        match self {
            ThemeType::Matrix => Color::Rgb(0, 100, 30),
            ThemeType::Fire => Color::Rgb(200, 50, 0),
            ThemeType::Starfield => Color::Rgb(50, 50, 100),
            ThemeType::Plasma => Color::Rgb(100, 0, 150),
            ThemeType::RainDrops => Color::Rgb(0, 50, 100),
            ThemeType::RadioWaves => Color::Rgb(100, 0, 150),
            ThemeType::SpinningShapes => Color::Rgb(100, 100, 0),
            ThemeType::Fireworks => Color::Rgb(150, 100, 0),
        }
    }

    /// Get the background color for this theme
    pub fn background_color(&self) -> Color {
        match self {
            ThemeType::Matrix => Color::Rgb(0, 10, 0),
            ThemeType::Fire => Color::Rgb(20, 5, 0),
            ThemeType::Starfield => Color::Rgb(0, 0, 15),
            ThemeType::Plasma => Color::Rgb(10, 0, 20),
            ThemeType::RainDrops => Color::Rgb(5, 10, 20),
            ThemeType::RadioWaves => Color::Rgb(10, 0, 20),
            ThemeType::SpinningShapes => Color::Rgb(10, 10, 20),
            ThemeType::Fireworks => Color::Rgb(5, 5, 15),
        }
    }
}
