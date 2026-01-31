# Font Creation Template

This document explains how to create a new digit font for pomowise.

## File Location

Fonts are defined in: `src/animation/digit_fonts.rs`

## Font Structure

Each font needs:
1. An entry in `DigitFont` enum
2. Digit patterns (0-9) as array of string slices
3. Colon pattern
4. Dimensions and character classifications

## Step-by-Step

### 1. Add to Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DigitFont {
    #[default]
    Classic,
    // ... existing fonts
    YourFont,  // Add here
}
```

### 2. Define Digit Patterns

Each digit is an array of string slices, one per row.
All digits must have the same height. Width should be consistent.

```rust
// ============================================================================
// YOUR FONT NAME (WIDTHxHEIGHT) - Brief description
// ============================================================================

const YOURFONT_DIGITS: [[&str; HEIGHT]; 10] = [
    // 0
    [
        "row1",
        "row2",
        // ... HEIGHT rows total
    ],
    // 1
    [
        "row1",
        "row2",
        // ...
    ],
    // ... digits 2-9
];

const YOURFONT_COLON: [&str; HEIGHT] = [
    "row1",
    "row2",
    // ... HEIGHT rows total
];
```

### 3. Update Enum Methods

Add your font to each match block:

```rust
impl DigitFont {
    pub fn all() -> &'static [DigitFont] {
        &[
            // ... existing
            DigitFont::YourFont,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            // ... existing
            DigitFont::YourFont => "Your Font Name",
        }
    }

    pub fn height(&self) -> u16 {
        match self {
            // ... existing
            DigitFont::YourFont => HEIGHT,
        }
    }

    pub fn width(&self) -> u16 {
        match self {
            // ... existing
            DigitFont::YourFont => WIDTH,
        }
    }

    pub fn colon_width(&self) -> u16 {
        match self {
            // ... existing
            DigitFont::YourFont => COLON_WIDTH,
        }
    }

    pub fn get_digit(&self, digit: usize) -> &'static [&'static str] {
        match self {
            // ... existing
            DigitFont::YourFont => &YOURFONT_DIGITS[digit.min(9)],
        }
    }

    pub fn get_colon(&self) -> &'static [&'static str] {
        match self {
            // ... existing
            DigitFont::YourFont => &YOURFONT_COLON,
        }
    }

    pub fn primary_chars(&self) -> &'static [char] {
        match self {
            // ... existing
            DigitFont::YourFont => &['█', '▓', /* main visible chars */],
        }
    }

    pub fn secondary_chars(&self) -> &'static [char] {
        match self {
            // ... existing
            DigitFont::YourFont => &['░', /* shadow/depth chars */],
        }
    }
}
```

## Common Characters for Fonts

```
Solid blocks:     █ ▓ ▒ ░
Half blocks:      ▄ ▀ ▌ ▐
Box drawing:      ╔ ╗ ╚ ╝ ║ ═ ┌ ┐ └ ┘ │ ─
Isometric:        / \ _ | ╱ ╲
Rounded:          ╭ ╮ ╯ ╰
Dots:             · • ●
Special:          ▁ ▏ ▕ ▔
```

## Example: Minimal Font (4x5)

```rust
const MINIMAL_DIGITS: [[&str; 5]; 10] = [
    // 0
    ["┌──┐", "│  │", "│  │", "│  │", "└──┘"],
    // 1
    ["  ┐ ", "  │ ", "  │ ", "  │ ", "  ┴ "],
    // 2
    ["───┐", "   │", "┌──┘", "│   ", "└───"],
    // 3
    ["───┐", "   │", " ──┤", "   │", "───┘"],
    // 4
    ["│  │", "│  │", "└──┤", "   │", "   │"],
    // 5
    ["┌───", "│   ", "└──┐", "   │", "───┘"],
    // 6
    ["┌──┐", "│   ", "├──┐", "│  │", "└──┘"],
    // 7
    ["───┐", "   │", "  ╱ ", " ╱  ", "╱   "],
    // 8
    ["┌──┐", "│  │", "├──┤", "│  │", "└──┘"],
    // 9
    ["┌──┐", "│  │", "└──┤", "   │", "└──┘"],
];

const MINIMAL_COLON: [&str; 5] = [" ", "·", " ", "·", " "];
```

## Tips

- Keep all rows the same width (pad with spaces)
- Test with different themes - font should be readable
- Primary chars get theme's primary color
- Secondary chars get theme's secondary (shadow) color
- Colon is typically narrower than digits
- Run tests: `cargo test test_font_dimensions`
