/// Digit font styles for countdown timer display
/// Each font provides digits 0-9 and a colon with consistent dimensions

/// Font style enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DigitFont {
    /// Original simple block font (5x5)
    #[default]
    Classic,
    /// 3D block style with depth effect (7x9)
    Block3D,
    /// Large outlined style with double lines (7x11)
    Outlined,
    /// Isometric 3D perspective (8x10)
    Isometric,
    /// Retro LCD style with segments (6x9)
    LCD,
}

impl DigitFont {
    pub fn all() -> &'static [DigitFont] {
        &[
            DigitFont::Classic,
            DigitFont::Block3D,
            DigitFont::Outlined,
            DigitFont::Isometric,
            DigitFont::LCD,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            DigitFont::Classic => "Classic",
            DigitFont::Block3D => "3D Blocks",
            DigitFont::Outlined => "Outlined",
            DigitFont::Isometric => "Isometric",
            DigitFont::LCD => "LCD",
        }
    }

    pub fn height(&self) -> u16 {
        match self {
            DigitFont::Classic => 5,
            DigitFont::Block3D => 9,
            DigitFont::Outlined => 11,
            DigitFont::Isometric => 10,
            DigitFont::LCD => 9,
        }
    }

    pub fn width(&self) -> u16 {
        match self {
            DigitFont::Classic => 5,
            DigitFont::Block3D => 7,
            DigitFont::Outlined => 7,
            DigitFont::Isometric => 8,
            DigitFont::LCD => 6,
        }
    }

    pub fn colon_width(&self) -> u16 {
        match self {
            DigitFont::Classic => 2,
            DigitFont::Block3D => 3,
            DigitFont::Outlined => 3,
            DigitFont::Isometric => 3,
            DigitFont::LCD => 2,
        }
    }

    pub fn get_digit(&self, digit: usize) -> &'static [&'static str] {
        let digit = digit.min(9);
        match self {
            DigitFont::Classic => &CLASSIC_DIGITS[digit],
            DigitFont::Block3D => &BLOCK3D_DIGITS[digit],
            DigitFont::Outlined => &OUTLINED_DIGITS[digit],
            DigitFont::Isometric => &ISOMETRIC_DIGITS[digit],
            DigitFont::LCD => &LCD_DIGITS[digit],
        }
    }

    pub fn get_colon(&self) -> &'static [&'static str] {
        match self {
            DigitFont::Classic => &CLASSIC_COLON,
            DigitFont::Block3D => &BLOCK3D_COLON,
            DigitFont::Outlined => &OUTLINED_COLON,
            DigitFont::Isometric => &ISOMETRIC_COLON,
            DigitFont::LCD => &LCD_COLON,
        }
    }

    /// Characters that should be styled as primary (foreground)
    pub fn primary_chars(&self) -> &'static [char] {
        match self {
            DigitFont::Classic => &['█'],
            DigitFont::Block3D => &['█', '▀', '▄', '▌', '▐', '▓', '▒'],
            DigitFont::Outlined => &['█', '▀', '▄', '║', '═', '╔', '╗', '╚', '╝', '│', '─', '┌', '┐', '└', '┘', '╠', '╣', '╬'],
            DigitFont::Isometric => &['/', '\\', '_', '|', '▓', '▒', '░'],
            DigitFont::LCD => &['█', '▀', '▄', '▐', '▌', '│', '─'],
        }
    }

    /// Characters that should be styled as secondary (shadow/depth)
    pub fn secondary_chars(&self) -> &'static [char] {
        match self {
            DigitFont::Classic => &[],
            DigitFont::Block3D => &['░', '▁', '▏'],
            DigitFont::Outlined => &['░', '▒'],
            DigitFont::Isometric => &['·', '.'],
            DigitFont::LCD => &['░'],
        }
    }

    pub fn next(&self) -> DigitFont {
        let all = Self::all();
        let idx = all.iter().position(|f| f == self).unwrap_or(0);
        all[(idx + 1) % all.len()]
    }
}

// ============================================================================
// CLASSIC FONT (5x5) - Original simple block style
// ============================================================================

const CLASSIC_DIGITS: [[&str; 5]; 10] = [
    // 0
    [" ███ ", "█   █", "█   █", "█   █", " ███ "],
    // 1
    ["  █  ", " ██  ", "  █  ", "  █  ", " ███ "],
    // 2
    [" ███ ", "    █", " ███ ", "█    ", "█████"],
    // 3
    ["█████", "    █", " ███ ", "    █", "█████"],
    // 4
    ["█   █", "█   █", "█████", "    █", "    █"],
    // 5
    ["█████", "█    ", "████ ", "    █", "████ "],
    // 6
    [" ███ ", "█    ", "████ ", "█   █", " ███ "],
    // 7
    ["█████", "    █", "   █ ", "  █  ", "  █  "],
    // 8
    [" ███ ", "█   █", " ███ ", "█   █", " ███ "],
    // 9
    [" ███ ", "█   █", " ████", "    █", " ███ "],
];

const CLASSIC_COLON: [&str; 5] = ["  ", "██", "  ", "██", "  "];

// ============================================================================
// BLOCK 3D FONT (7x9) - 3D block effect with depth
// ============================================================================

const BLOCK3D_DIGITS: [[&str; 9]; 10] = [
    // 0
    [
        " ▄███▄ ",
        "██▀▀▀██",
        "██   ██",
        "██   ██",
        "██   ██",
        "██   ██",
        "██▄▄▄██",
        " ▀███▀ ",
        "       ",
    ],
    // 1
    [
        "  ▄██  ",
        " ▀███  ",
        "  ███  ",
        "  ███  ",
        "  ███  ",
        "  ███  ",
        "  ███  ",
        " ▄███▄ ",
        "       ",
    ],
    // 2
    [
        " ▄███▄ ",
        "██▀▀▀██",
        "    ▐██",
        "   ▄██▀",
        "  ▄██▀ ",
        " ▄██▀  ",
        "███████",
        "▀▀▀▀▀▀▀",
        "       ",
    ],
    // 3
    [
        "▄█████▄",
        "▀▀▀▀▀██",
        "    ▐██",
        " ▄████▀",
        "    ▀██",
        "     ██",
        "▄▄▄▄▄██",
        "▀█████▀",
        "       ",
    ],
    // 4
    [
        "██   ██",
        "██   ██",
        "██   ██",
        "███████",
        "▀▀▀▀▀██",
        "     ██",
        "     ██",
        "     ▀▀",
        "       ",
    ],
    // 5
    [
        "███████",
        "██▀▀▀▀▀",
        "██     ",
        "██████▄",
        "▀▀▀▀▀██",
        "     ██",
        "▄▄▄▄▄██",
        "▀█████▀",
        "       ",
    ],
    // 6
    [
        " ▄███▄ ",
        "██▀▀▀▀ ",
        "██     ",
        "██████▄",
        "██▀▀▀██",
        "██   ██",
        "██▄▄▄██",
        " ▀███▀ ",
        "       ",
    ],
    // 7
    [
        "███████",
        "▀▀▀▀▀██",
        "    ▐██",
        "   ▄██▀",
        "  ▐██▀ ",
        "  ██▀  ",
        "  ██   ",
        "  ▀▀   ",
        "       ",
    ],
    // 8
    [
        " ▄███▄ ",
        "██▀▀▀██",
        "██   ██",
        " ▀███▀ ",
        "██▀▀▀██",
        "██   ██",
        "██▄▄▄██",
        " ▀███▀ ",
        "       ",
    ],
    // 9
    [
        " ▄███▄ ",
        "██▀▀▀██",
        "██   ██",
        " ▀████▀",
        "     ██",
        "    ▐██",
        "▄▄▄▄██▀",
        "▀████▀ ",
        "       ",
    ],
];

const BLOCK3D_COLON: [&str; 9] = [
    "   ",
    "   ",
    "▐█▌",
    "   ",
    "   ",
    "▐█▌",
    "   ",
    "   ",
    "   ",
];

// ============================================================================
// OUTLINED FONT (7x11) - Double-line outlined style
// ============================================================================

const OUTLINED_DIGITS: [[&str; 11]; 10] = [
    // 0
    [
        "╔═════╗",
        "║█████║",
        "║██ ██║",
        "║██ ██║",
        "║██ ██║",
        "║██ ██║",
        "║██ ██║",
        "║██ ██║",
        "║█████║",
        "╚═════╝",
        "       ",
    ],
    // 1
    [
        "  ╔══╗ ",
        "  ║██║ ",
        " ╔╝██║ ",
        " ║ ██║ ",
        " ║ ██║ ",
        " ║ ██║ ",
        " ║ ██║ ",
        " ║ ██║ ",
        "╔╩═══╩╗",
        "╚═════╝",
        "       ",
    ],
    // 2
    [
        "╔═════╗",
        "║█████║",
        "╚══╗██║",
        "   ║██║",
        "╔══╩██║",
        "║█████║",
        "║██╔══╝",
        "║██║   ",
        "║█████╗",
        "╚═════╝",
        "       ",
    ],
    // 3
    [
        "╔═════╗",
        "║█████║",
        "╚══╗██║",
        "   ║██║",
        "╔══╩██║",
        "╚══╗██║",
        "   ║██║",
        "╔══╝██║",
        "║█████║",
        "╚═════╝",
        "       ",
    ],
    // 4
    [
        "╔══╗╔══",
        "║██║║██",
        "║██║║██",
        "║██║║██",
        "║█████║",
        "╚══╗║██",
        "   ║║██",
        "   ║║██",
        "   ╚╩══",
        "       ",
        "       ",
    ],
    // 5
    [
        "╔═════╗",
        "║█████║",
        "║██╔══╝",
        "║██║   ",
        "║█████╗",
        "╚══╗██║",
        "   ║██║",
        "╔══╝██║",
        "║█████║",
        "╚═════╝",
        "       ",
    ],
    // 6
    [
        "╔═════╗",
        "║█████║",
        "║██╔══╝",
        "║██║   ",
        "║█████╗",
        "║██╔██║",
        "║██║██║",
        "║██╚██║",
        "║█████║",
        "╚═════╝",
        "       ",
    ],
    // 7
    [
        "╔═════╗",
        "║█████║",
        "╚══╗██║",
        "   ║██║",
        "  ╔╝██║",
        "  ║██╔╝",
        "  ║██║ ",
        "  ║██║ ",
        "  ╚══╝ ",
        "       ",
        "       ",
    ],
    // 8
    [
        "╔═════╗",
        "║█████║",
        "║██╔██║",
        "║██║██║",
        "║█████║",
        "║██╔██║",
        "║██║██║",
        "║██╚██║",
        "║█████║",
        "╚═════╝",
        "       ",
    ],
    // 9
    [
        "╔═════╗",
        "║█████║",
        "║██╔██║",
        "║██║██║",
        "║█████║",
        "╚══╗██║",
        "   ║██║",
        "╔══╝██║",
        "║█████║",
        "╚═════╝",
        "       ",
    ],
];

const OUTLINED_COLON: [&str; 11] = [
    "   ",
    "   ",
    "╔═╗",
    "║█║",
    "╚═╝",
    "   ",
    "╔═╗",
    "║█║",
    "╚═╝",
    "   ",
    "   ",
];

// ============================================================================
// ISOMETRIC FONT (8x10) - 3D isometric perspective
// ============================================================================

const ISOMETRIC_DIGITS: [[&str; 10]; 10] = [
    // 0
    [
        "  ____  ",
        " /\\   \\ ",
        "/  \\   \\",
        "\\   \\  /",
        " \\   \\/ ",
        " /\\   \\ ",
        "/  \\   \\",
        "\\   \\  /",
        " \\___\\/ ",
        "        ",
    ],
    // 1
    [
        "   __   ",
        "  /\\ \\  ",
        " /  \\ \\ ",
        " \\   \\ \\",
        "  \\   \\ ",
        "  /\\   \\",
        " /  \\   \\",
        " \\___\\__\\",
        "        ",
        "        ",
    ],
    // 2
    [
        "  ____  ",
        " /\\   \\ ",
        " \\ \\   \\",
        "  \\ \\  /",
        "  /\\  / ",
        " /  \\   ",
        "/____\\  ",
        "\\______\\",
        "        ",
        "        ",
    ],
    // 3
    [
        " ______ ",
        " \\     \\",
        "  \\    /",
        "  /___/ ",
        "  \\    \\",
        "   \\   /",
        " __\\  / ",
        " \\____/ ",
        "        ",
        "        ",
    ],
    // 4
    [
        " __  __ ",
        "/\\ \\/\\ \\",
        "\\ \\ \\_\\ \\",
        " \\ \\____\\",
        "  \\/___/\\",
        "      \\ \\",
        "       \\ \\",
        "        \\/",
        "        ",
        "        ",
    ],
    // 5
    [
        " ______ ",
        "/\\     \\",
        "\\ \\     ",
        " \\ \\____",
        "  \\/   /",
        "   \\  / ",
        " __/  / ",
        " \\____/ ",
        "        ",
        "        ",
    ],
    // 6
    [
        "  ____  ",
        " /\\   \\ ",
        "/  \\    ",
        "\\   \\___",
        " \\  /  /",
        " /\\ \\  \\",
        "/  \\ \\  \\",
        "\\___\\_\\/ ",
        "        ",
        "        ",
    ],
    // 7
    [
        " ______ ",
        " \\     \\",
        "  \\    /",
        "   \\  / ",
        "   /  \\ ",
        "  /  / \\",
        " /  /   ",
        " \\/     ",
        "        ",
        "        ",
    ],
    // 8
    [
        "  ____  ",
        " /\\   \\ ",
        "/  \\   \\",
        "\\  /\\  /",
        " \\/  \\/ ",
        " /\\  /\\ ",
        "/  \\/  \\",
        "\\______/",
        "        ",
        "        ",
    ],
    // 9
    [
        "  ____  ",
        " /\\   \\ ",
        "/  \\   \\",
        "\\   \\__/",
        " \\___/\\ ",
        "     \\ \\",
        "  ___\\ \\",
        "  \\____\\",
        "        ",
        "        ",
    ],
];

const ISOMETRIC_COLON: [&str; 10] = [
    "   ",
    "   ",
    " /\\",
    " \\/",
    "   ",
    "   ",
    " /\\",
    " \\/",
    "   ",
    "   ",
];

// ============================================================================
// LCD FONT (6x9) - Retro LCD segment display
// ============================================================================

const LCD_DIGITS: [[&str; 9]; 10] = [
    // 0
    [
        " ▄▄▄▄ ",
        "▐█▀▀█▌",
        "▐█  █▌",
        "▐█  █▌",
        "▐█  █▌",
        "▐█  █▌",
        "▐█▄▄█▌",
        " ▀▀▀▀ ",
        "      ",
    ],
    // 1
    [
        "  ▄█  ",
        " ▀██  ",
        "  ██  ",
        "  ██  ",
        "  ██  ",
        "  ██  ",
        " ▄██▄ ",
        "      ",
        "      ",
    ],
    // 2
    [
        " ▄▄▄▄ ",
        "▀▀▀▀█▌",
        "    █▌",
        " ▄▄▄█▌",
        "▐█▀▀▀ ",
        "▐█    ",
        "▐█▄▄▄▄",
        " ▀▀▀▀▀",
        "      ",
    ],
    // 3
    [
        "▄▄▄▄▄ ",
        "▀▀▀▀█▌",
        "    █▌",
        " ▄▄▄█▌",
        "    █▌",
        "    █▌",
        "▄▄▄▄█▌",
        "▀▀▀▀▀ ",
        "      ",
    ],
    // 4
    [
        "▐█  █▌",
        "▐█  █▌",
        "▐█  █▌",
        "▐█▄▄█▌",
        " ▀▀▀█▌",
        "    █▌",
        "    █▌",
        "      ",
        "      ",
    ],
    // 5
    [
        "▐█▄▄▄▄",
        "▐█▀▀▀▀",
        "▐█    ",
        "▐█▄▄▄ ",
        " ▀▀▀█▌",
        "    █▌",
        "▄▄▄▄█▌",
        "▀▀▀▀▀ ",
        "      ",
    ],
    // 6
    [
        " ▄▄▄▄ ",
        "▐█▀▀▀ ",
        "▐█    ",
        "▐█▄▄▄ ",
        "▐█▀▀█▌",
        "▐█  █▌",
        "▐█▄▄█▌",
        " ▀▀▀▀ ",
        "      ",
    ],
    // 7
    [
        "▄▄▄▄▄▄",
        "▀▀▀▀█▌",
        "   ▐█ ",
        "   █▌ ",
        "  ▐█  ",
        "  █▌  ",
        "  █▌  ",
        "      ",
        "      ",
    ],
    // 8
    [
        " ▄▄▄▄ ",
        "▐█▀▀█▌",
        "▐█  █▌",
        " ▀▄▄▀ ",
        "▐█▀▀█▌",
        "▐█  █▌",
        "▐█▄▄█▌",
        " ▀▀▀▀ ",
        "      ",
    ],
    // 9
    [
        " ▄▄▄▄ ",
        "▐█▀▀█▌",
        "▐█  █▌",
        " ▀▄▄█▌",
        "    █▌",
        "   ▐█ ",
        "▄▄▄█▌ ",
        "▀▀▀▀  ",
        "      ",
    ],
];

const LCD_COLON: [&str; 9] = [
    "  ",
    "  ",
    "▐▌",
    "  ",
    "  ",
    "▐▌",
    "  ",
    "  ",
    "  ",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_dimensions() {
        for font in DigitFont::all() {
            for digit in 0..10 {
                let lines = font.get_digit(digit);
                assert_eq!(
                    lines.len() as u16,
                    font.height(),
                    "Font {:?} digit {} height mismatch",
                    font,
                    digit
                );
            }
            let colon = font.get_colon();
            assert_eq!(
                colon.len() as u16,
                font.height(),
                "Font {:?} colon height mismatch",
                font
            );
        }
    }

    #[test]
    fn test_font_cycle() {
        let mut font = DigitFont::Classic;
        let start = font;
        for _ in 0..5 {
            font = font.next();
        }
        assert_eq!(font, start, "Font should cycle back to start");
    }
}
