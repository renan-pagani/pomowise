use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

/// Big 7-segment style digit patterns (5 lines tall, variable width)
/// Each digit is represented as a 5-line array of strings
pub const DIGIT_HEIGHT: u16 = 5;
pub const DIGIT_WIDTH: u16 = 5;
pub const COLON_WIDTH: u16 = 2;

const DIGITS: [[&str; 5]; 10] = [
    // 0
    [
        " ███ ",
        "█   █",
        "█   █",
        "█   █",
        " ███ ",
    ],
    // 1
    [
        "  █  ",
        " ██  ",
        "  █  ",
        "  █  ",
        " ███ ",
    ],
    // 2
    [
        " ███ ",
        "    █",
        " ███ ",
        "█    ",
        "█████",
    ],
    // 3
    [
        "█████",
        "    █",
        " ███ ",
        "    █",
        "█████",
    ],
    // 4
    [
        "█   █",
        "█   █",
        "█████",
        "    █",
        "    █",
    ],
    // 5
    [
        "█████",
        "█    ",
        "████ ",
        "    █",
        "████ ",
    ],
    // 6
    [
        " ███ ",
        "█    ",
        "████ ",
        "█   █",
        " ███ ",
    ],
    // 7
    [
        "█████",
        "    █",
        "   █ ",
        "  █  ",
        "  █  ",
    ],
    // 8
    [
        " ███ ",
        "█   █",
        " ███ ",
        "█   █",
        " ███ ",
    ],
    // 9
    [
        " ███ ",
        "█   █",
        " ████",
        "    █",
        " ███ ",
    ],
];

const COLON: [&str; 5] = [
    "  ",
    "██",
    "  ",
    "██",
    "  ",
];

/// Render big digits for the timer display
/// Format: MM:SS centered in the given area
pub fn render_time(
    frame: &mut Frame,
    area: Rect,
    minutes: u8,
    seconds: u8,
    primary_color: Color,
    secondary_color: Color,
) {
    let m1 = (minutes / 10) as usize;
    let m2 = (minutes % 10) as usize;
    let s1 = (seconds / 10) as usize;
    let s2 = (seconds % 10) as usize;

    // Total width: 4 digits (5 each) + colon (2) + spacing (4 spaces between)
    // = 20 + 2 + 4 = 26 characters
    let total_width = DIGIT_WIDTH * 4 + COLON_WIDTH + 4;
    let start_x = area.x + area.width.saturating_sub(total_width) / 2;
    let start_y = area.y + area.height.saturating_sub(DIGIT_HEIGHT) / 2;

    // Render each digit and colon
    let mut x_offset = start_x;

    // First minute digit
    render_digit(frame, x_offset, start_y, m1, primary_color, secondary_color);
    x_offset += DIGIT_WIDTH + 1;

    // Second minute digit
    render_digit(frame, x_offset, start_y, m2, primary_color, secondary_color);
    x_offset += DIGIT_WIDTH + 1;

    // Colon
    render_colon(frame, x_offset, start_y, primary_color);
    x_offset += COLON_WIDTH + 1;

    // First second digit
    render_digit(frame, x_offset, start_y, s1, primary_color, secondary_color);
    x_offset += DIGIT_WIDTH + 1;

    // Second second digit
    render_digit(frame, x_offset, start_y, s2, primary_color, secondary_color);
}

fn render_digit(frame: &mut Frame, x: u16, y: u16, digit: usize, primary: Color, secondary: Color) {
    let digit = digit.min(9);
    let pattern = &DIGITS[digit];
    let frame_area = frame.area();

    for (i, line) in pattern.iter().enumerate() {
        let line_y = y + i as u16;
        if line_y >= frame_area.height || x >= frame_area.width {
            continue;
        }

        let styled_line = style_digit_line(line, primary, secondary);
        let width = (DIGIT_WIDTH).min(frame_area.width.saturating_sub(x));
        frame.render_widget(
            Paragraph::new(styled_line),
            Rect::new(x, line_y, width, 1),
        );
    }
}

fn render_colon(frame: &mut Frame, x: u16, y: u16, color: Color) {
    let frame_area = frame.area();

    for (i, line) in COLON.iter().enumerate() {
        let line_y = y + i as u16;
        if line_y >= frame_area.height || x >= frame_area.width {
            continue;
        }

        let styled = Line::from(Span::styled(
            line.to_string(),
            Style::default().fg(color),
        ));
        let width = COLON_WIDTH.min(frame_area.width.saturating_sub(x));
        frame.render_widget(Paragraph::new(styled), Rect::new(x, line_y, width, 1));
    }
}

fn style_digit_line(line: &str, primary: Color, _secondary: Color) -> Line<'static> {
    let spans: Vec<Span> = line
        .chars()
        .map(|ch| {
            let style = match ch {
                '█' => Style::default().fg(primary),
                _ => Style::default(),
            };
            Span::styled(ch.to_string(), style)
        })
        .collect();
    Line::from(spans)
}

/// Get the dimensions needed for the timer display
pub fn timer_dimensions() -> (u16, u16) {
    // Width: 4 digits + colon + spacing
    let width = DIGIT_WIDTH * 4 + COLON_WIDTH + 4;
    let height = DIGIT_HEIGHT;
    (width, height)
}
