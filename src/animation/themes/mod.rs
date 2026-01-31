pub mod matrix;
pub mod fire;
pub mod starfield;
pub mod plasma;
pub mod rain;
pub mod waves;
pub mod shapes;
pub mod fireworks;
pub mod aurora;
pub mod ocean;
pub mod dna;
pub mod bubbles;
pub mod electric;
pub mod snowfall;
pub mod nature;
pub mod geometric;
pub mod glitch;
pub mod minimal;
pub mod seasonal;
pub mod landscape;
pub mod claude;
pub mod github;
pub mod medieval;
pub mod synthwave;

use ratatui::prelude::*;
use crate::animation::digit_fonts::DigitFont;

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
    Aurora,
    Ocean,
    DNA,
    Bubbles,
    Electric,
    Snowfall,
    Nature,
    Geometric,
    Glitch,
    Minimal,
    Seasonal,
    Landscape,
    Claude,
    GitHub,
    Medieval,
    Synthwave,
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
            ThemeType::Aurora,
            ThemeType::Ocean,
            ThemeType::DNA,
            ThemeType::Bubbles,
            ThemeType::Electric,
            ThemeType::Snowfall,
            ThemeType::Nature,
            ThemeType::Geometric,
            ThemeType::Glitch,
            ThemeType::Minimal,
            ThemeType::Seasonal,
            ThemeType::Landscape,
            ThemeType::Claude,
            ThemeType::GitHub,
            ThemeType::Medieval,
            ThemeType::Synthwave,
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
            ThemeType::Aurora => "Aurora Borealis",
            ThemeType::Ocean => "Ocean Waves",
            ThemeType::DNA => "DNA Helix",
            ThemeType::Bubbles => "Bubbles",
            ThemeType::Electric => "Electric Storm",
            ThemeType::Snowfall => "Snowfall",
            ThemeType::Nature => "Forest Nature",
            ThemeType::Geometric => "Geometric Patterns",
            ThemeType::Glitch => "Glitch Cyberpunk",
            ThemeType::Minimal => "Minimal Zen",
            ThemeType::Seasonal => "Seasonal",
            ThemeType::Landscape => "Landscape",
            ThemeType::Claude => "Claude",
            ThemeType::GitHub => "GitHub",
            ThemeType::Medieval => "Medieval",
            ThemeType::Synthwave => "Synthwave",
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
            ThemeType::Aurora => aurora::render_background(frame, area, frame_index),
            ThemeType::Ocean => ocean::render_background(frame, area, frame_index),
            ThemeType::DNA => dna::render_background(frame, area, frame_index),
            ThemeType::Bubbles => bubbles::render_background(frame, area, frame_index),
            ThemeType::Electric => electric::render_background(frame, area, frame_index),
            ThemeType::Snowfall => snowfall::render_background(frame, area, frame_index),
            ThemeType::Nature => nature::render_background(frame, area, frame_index),
            ThemeType::Geometric => geometric::render_background(frame, area, frame_index),
            ThemeType::Glitch => glitch::render_background(frame, area, frame_index),
            ThemeType::Minimal => minimal::render_background(frame, area, frame_index),
            ThemeType::Seasonal => seasonal::render_background(frame, area, frame_index),
            ThemeType::Landscape => landscape::render_background(frame, area, frame_index),
            ThemeType::Claude => claude::render_background(frame, area, frame_index),
            ThemeType::GitHub => github::render_background(frame, area, frame_index),
            ThemeType::Medieval => medieval::render_background(frame, area, frame_index),
            ThemeType::Synthwave => synthwave::render_background(frame, area, frame_index),
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
            ThemeType::Aurora => Color::Rgb(100, 255, 150),    // Aurora green
            ThemeType::Ocean => Color::Rgb(100, 200, 255),     // Ocean blue
            ThemeType::DNA => Color::Rgb(150, 255, 200),       // Bio green
            ThemeType::Bubbles => Color::Rgb(180, 220, 255),   // Bubble blue
            ThemeType::Electric => Color::Rgb(150, 200, 255),  // Electric blue
            ThemeType::Snowfall => Color::Rgb(220, 230, 255),  // Snow white
            ThemeType::Nature => Color::Rgb(120, 200, 100),    // Forest green
            ThemeType::Geometric => Color::Rgb(200, 150, 255), // Violet
            ThemeType::Glitch => Color::Rgb(255, 50, 150),     // Hot pink
            ThemeType::Minimal => Color::Rgb(150, 160, 180),   // Calm grey-blue
            ThemeType::Seasonal => Color::Rgb(200, 180, 150),  // Warm neutral
            ThemeType::Landscape => Color::Rgb(150, 200, 100), // Pastoral green
            ThemeType::Claude => Color::Rgb(217, 119, 6),      // Anthropic orange
            ThemeType::GitHub => Color::Rgb(57, 211, 83),      // GitHub green
            ThemeType::Medieval => Color::Rgb(255, 180, 80),   // Torch orange
            ThemeType::Synthwave => Color::Rgb(255, 100, 200), // Neon pink
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
            ThemeType::Aurora => Color::Rgb(50, 100, 80),
            ThemeType::Ocean => Color::Rgb(30, 80, 120),
            ThemeType::DNA => Color::Rgb(60, 120, 100),
            ThemeType::Bubbles => Color::Rgb(80, 120, 150),
            ThemeType::Electric => Color::Rgb(50, 80, 150),
            ThemeType::Snowfall => Color::Rgb(100, 120, 150),
            ThemeType::Nature => Color::Rgb(60, 100, 50),
            ThemeType::Geometric => Color::Rgb(80, 60, 120),
            ThemeType::Glitch => Color::Rgb(100, 0, 80),
            ThemeType::Minimal => Color::Rgb(60, 70, 80),
            ThemeType::Seasonal => Color::Rgb(100, 90, 80),
            ThemeType::Landscape => Color::Rgb(80, 120, 60),
            ThemeType::Claude => Color::Rgb(120, 70, 10),
            ThemeType::GitHub => Color::Rgb(30, 100, 40),
            ThemeType::Medieval => Color::Rgb(100, 60, 30),
            ThemeType::Synthwave => Color::Rgb(150, 50, 100),
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
            ThemeType::Aurora => Color::Rgb(5, 5, 15),
            ThemeType::Ocean => Color::Rgb(0, 20, 40),
            ThemeType::DNA => Color::Rgb(5, 10, 20),
            ThemeType::Bubbles => Color::Rgb(5, 15, 35),
            ThemeType::Electric => Color::Rgb(10, 10, 20),
            ThemeType::Snowfall => Color::Rgb(10, 15, 25),
            ThemeType::Nature => Color::Rgb(15, 30, 20),
            ThemeType::Geometric => Color::Rgb(8, 5, 15),
            ThemeType::Glitch => Color::Rgb(5, 5, 12),
            ThemeType::Minimal => Color::Rgb(12, 12, 15),
            ThemeType::Seasonal => Color::Rgb(20, 20, 25),
            ThemeType::Landscape => Color::Rgb(20, 30, 40),
            ThemeType::Claude => Color::Rgb(30, 20, 15),
            ThemeType::GitHub => Color::Rgb(13, 17, 23),
            ThemeType::Medieval => Color::Rgb(15, 12, 10),
            ThemeType::Synthwave => Color::Rgb(10, 5, 20),
        }
    }

    /// Get the preferred font for this theme
    pub fn font(&self) -> DigitFont {
        match self {
            ThemeType::Claude => DigitFont::ClaudeFont,
            ThemeType::GitHub => DigitFont::Terminal,
            ThemeType::Medieval => DigitFont::Gothic,
            ThemeType::Synthwave => DigitFont::Neon,
            ThemeType::Nature => DigitFont::Bamboo,
            ThemeType::Geometric => DigitFont::Angular,
            ThemeType::Glitch => DigitFont::Fragmented,
            ThemeType::Minimal => DigitFont::Hairline,
            ThemeType::Seasonal => DigitFont::SeasonalFont,
            ThemeType::Landscape => DigitFont::Savanna,
            // Default to Block3D for themes without a specific font
            _ => DigitFont::Block3D,
        }
    }
}
