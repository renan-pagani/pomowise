//! Terminal scaling and adaptive layout system
//! Handles different terminal sizes gracefully with automatic font selection

use crate::animation::DigitFont;

/// Minimum terminal dimensions for the app to function
pub const MIN_WIDTH: u16 = 40;
pub const MIN_HEIGHT: u16 = 15;

/// Terminal size categories
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TerminalSize {
    TooSmall,    // Below minimum - show warning
    Compact,     // 40-60 width - use smallest fonts
    Medium,      // 60-100 width - use medium fonts
    Large,       // 100-150 width - use standard fonts
    ExtraLarge,  // 150+ width - use largest fonts
}

impl TerminalSize {
    /// Determine terminal size category from dimensions
    pub fn from_dimensions(width: u16, height: u16) -> Self {
        if width < MIN_WIDTH || height < MIN_HEIGHT {
            TerminalSize::TooSmall
        } else if width < 60 || height < 20 {
            TerminalSize::Compact
        } else if width < 100 || height < 30 {
            TerminalSize::Medium
        } else if width < 150 || height < 45 {
            TerminalSize::Large
        } else {
            TerminalSize::ExtraLarge
        }
    }
}

/// Scaling context with all calculated dimensions
#[derive(Debug, Clone)]
pub struct ScalingContext {
    pub width: u16,
    pub height: u16,
    pub size_category: TerminalSize,
    pub recommended_font: DigitFont,
    pub timer_area_height: u16,
    pub show_progress_bar: bool,
    pub show_hints: bool,
    pub show_session_info: bool,
    pub background_detail_level: u8, // 0-3, affects theme complexity
}

impl ScalingContext {
    /// Create scaling context from terminal dimensions
    pub fn new(width: u16, height: u16) -> Self {
        let size_category = TerminalSize::from_dimensions(width, height);

        let (recommended_font, background_detail_level, show_progress_bar, show_hints, show_session_info) =
            match size_category {
                TerminalSize::TooSmall => (
                    DigitFont::Classic, // Smallest font
                    0,    // Minimal background
                    false,
                    false,
                    false,
                ),
                TerminalSize::Compact => (
                    DigitFont::Classic, // 5x5 - smallest
                    1,    // Simple background
                    true,
                    false, // Hide hints in compact mode
                    false,
                ),
                TerminalSize::Medium => (
                    DigitFont::Terminal, // 5x7 - compact but readable
                    2,    // Medium detail
                    true,
                    true,
                    true,
                ),
                TerminalSize::Large => (
                    DigitFont::Block3D, // 7x9 - nice looking
                    3,    // Full detail
                    true,
                    true,
                    true,
                ),
                TerminalSize::ExtraLarge => (
                    DigitFont::Outlined, // 7x11 - largest
                    3,    // Full detail
                    true,
                    true,
                    true,
                ),
            };

        // Calculate timer area height based on font
        let timer_area_height = recommended_font.height() + 4; // Font height + padding

        Self {
            width,
            height,
            size_category,
            recommended_font,
            timer_area_height,
            show_progress_bar,
            show_hints,
            show_session_info,
            background_detail_level,
        }
    }

    /// Check if terminal is too small to render
    pub fn is_too_small(&self) -> bool {
        self.size_category == TerminalSize::TooSmall
    }

    /// Get the timer display width for current font
    pub fn timer_width(&self) -> u16 {
        // MM:SS format = 4 digits + colon
        self.recommended_font.width() * 4 + self.recommended_font.colon_width() + 4
    }

    /// Calculate centered X position for an element of given width
    pub fn center_x(&self, element_width: u16) -> u16 {
        if element_width >= self.width {
            0
        } else {
            (self.width - element_width) / 2
        }
    }

    /// Calculate centered Y position for an element of given height
    pub fn center_y(&self, element_height: u16) -> u16 {
        if element_height >= self.height {
            0
        } else {
            (self.height - element_height) / 2
        }
    }

    /// Get vertical position for the timer (slightly above center)
    pub fn timer_y(&self) -> u16 {
        let font_height = self.recommended_font.height();
        if font_height + 4 >= self.height {
            0
        } else {
            // Position at ~40% from top for visual balance
            (self.height as f32 * 0.35) as u16
        }
    }

    /// Get progress bar area (bottom of screen)
    pub fn progress_bar_y(&self) -> u16 {
        self.height.saturating_sub(3)
    }

    /// Get hint bar area (above progress bar)
    pub fn hints_y(&self) -> u16 {
        self.height.saturating_sub(5)
    }

    /// Scale a value proportionally to terminal width
    pub fn scale_width(&self, base_value: u16, reference_width: u16) -> u16 {
        ((base_value as f32 * self.width as f32) / reference_width as f32) as u16
    }

    /// Scale a value proportionally to terminal height
    pub fn scale_height(&self, base_value: u16, reference_height: u16) -> u16 {
        ((base_value as f32 * self.height as f32) / reference_height as f32) as u16
    }
}

/// Select the best font for given terminal dimensions
pub fn select_font_for_size(width: u16, height: u16) -> DigitFont {
    // Calculate available space for timer (assume ~60% of width, ~40% of height)
    let available_width = (width as f32 * 0.6) as u16;
    let available_height = (height as f32 * 0.4) as u16;

    // Timer needs: 4 * digit_width + colon_width for width
    // and: digit_height for height

    // Fonts sorted by size (smallest to largest)
    let fonts_by_size = [
        (DigitFont::Classic, 5, 5),      // Width: 5*4+2 = 22, Height: 5
        (DigitFont::Terminal, 5, 7),     // Width: 5*4+2 = 22, Height: 7
        (DigitFont::Hairline, 5, 7),     // Width: 5*4+2 = 22, Height: 7
        (DigitFont::Organic, 6, 7),      // Width: 6*4+3 = 27, Height: 7
        (DigitFont::ClaudeFont, 6, 8),   // Width: 6*4+3 = 27, Height: 8
        (DigitFont::Angular, 6, 8),      // Width: 6*4+3 = 27, Height: 8
        (DigitFont::Bamboo, 6, 8),       // Width: 6*4+3 = 27, Height: 8
        (DigitFont::SeasonalFont, 6, 8), // Width: 6*4+3 = 27, Height: 8
        (DigitFont::LCD, 6, 9),          // Width: 6*4+3 = 27, Height: 9
        (DigitFont::Block3D, 7, 9),      // Width: 7*4+3 = 31, Height: 9
        (DigitFont::Gothic, 7, 9),       // Width: 7*4+3 = 31, Height: 9
        (DigitFont::Neon, 7, 9),         // Width: 7*4+3 = 31, Height: 9
        (DigitFont::Fragmented, 7, 9),   // Width: 7*4+3 = 31, Height: 9
        (DigitFont::Savanna, 9, 9),      // Width: 9*4+3 = 39, Height: 9
        (DigitFont::Isometric, 8, 10),   // Width: 8*4+3 = 35, Height: 10
        (DigitFont::Outlined, 7, 11),    // Width: 7*4+3 = 31, Height: 11
    ];

    // Find the largest font that fits
    let mut best_font = DigitFont::Classic;

    for (font, digit_width, digit_height) in fonts_by_size.iter().rev() {
        let timer_width = *digit_width * 4 + 3; // 4 digits + colon

        if timer_width <= available_width && *digit_height <= available_height {
            best_font = *font;
            break;
        }
    }

    best_font
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_categories() {
        assert_eq!(TerminalSize::from_dimensions(30, 10), TerminalSize::TooSmall);
        assert_eq!(TerminalSize::from_dimensions(50, 18), TerminalSize::Compact);
        assert_eq!(TerminalSize::from_dimensions(80, 24), TerminalSize::Medium);
        assert_eq!(TerminalSize::from_dimensions(120, 40), TerminalSize::Large);
        assert_eq!(TerminalSize::from_dimensions(200, 50), TerminalSize::ExtraLarge);
    }

    #[test]
    fn test_font_selection() {
        // Small terminal should get small font
        let small = select_font_for_size(50, 20);
        assert!(small.height() <= 7);

        // Large terminal can use bigger font
        let large = select_font_for_size(150, 50);
        assert!(large.height() >= 9);
    }
}
