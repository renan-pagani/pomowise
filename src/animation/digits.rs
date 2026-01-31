use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use super::digit_fonts::DigitFont;

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
    render_time_with_font(
        frame,
        area,
        minutes,
        seconds,
        primary_color,
        secondary_color,
        DigitFont::default(),
    );
}

/// Render big digits with a specific font style
pub fn render_time_with_font(
    frame: &mut Frame,
    area: Rect,
    minutes: u8,
    seconds: u8,
    primary_color: Color,
    secondary_color: Color,
    font: DigitFont,
) {
    let m1 = (minutes / 10) as usize;
    let m2 = (minutes % 10) as usize;
    let s1 = (seconds / 10) as usize;
    let s2 = (seconds % 10) as usize;

    let digit_width = font.width();
    let digit_height = font.height();
    let colon_width = font.colon_width();

    // Total width: 4 digits + colon + spacing (4 spaces between)
    let total_width = digit_width * 4 + colon_width + 4;
    let start_x = area.x + area.width.saturating_sub(total_width) / 2;
    let start_y = area.y + area.height.saturating_sub(digit_height) / 2;

    // Render each digit and colon
    let mut x_offset = start_x;

    // First minute digit
    render_digit_with_font(
        frame,
        x_offset,
        start_y,
        m1,
        primary_color,
        secondary_color,
        font,
    );
    x_offset += digit_width + 1;

    // Second minute digit
    render_digit_with_font(
        frame,
        x_offset,
        start_y,
        m2,
        primary_color,
        secondary_color,
        font,
    );
    x_offset += digit_width + 1;

    // Colon
    render_colon_with_font(frame, x_offset, start_y, primary_color, secondary_color, font);
    x_offset += colon_width + 1;

    // First second digit
    render_digit_with_font(
        frame,
        x_offset,
        start_y,
        s1,
        primary_color,
        secondary_color,
        font,
    );
    x_offset += digit_width + 1;

    // Second second digit
    render_digit_with_font(
        frame,
        x_offset,
        start_y,
        s2,
        primary_color,
        secondary_color,
        font,
    );
}

fn render_digit_with_font(
    frame: &mut Frame,
    x: u16,
    y: u16,
    digit: usize,
    primary: Color,
    secondary: Color,
    font: DigitFont,
) {
    let digit = digit.min(9);
    let pattern = font.get_digit(digit);
    let frame_area = frame.area();
    let primary_chars = font.primary_chars();
    let secondary_chars = font.secondary_chars();

    for (i, line) in pattern.iter().enumerate() {
        let line_y = y + i as u16;
        if line_y >= frame_area.height || x >= frame_area.width {
            continue;
        }

        let styled_line = style_line(line, primary, secondary, primary_chars, secondary_chars);
        let width = font.width().min(frame_area.width.saturating_sub(x));
        frame.render_widget(
            Paragraph::new(styled_line),
            Rect::new(x, line_y, width, 1),
        );
    }
}

fn render_colon_with_font(
    frame: &mut Frame,
    x: u16,
    y: u16,
    primary: Color,
    secondary: Color,
    font: DigitFont,
) {
    let frame_area = frame.area();
    let pattern = font.get_colon();
    let primary_chars = font.primary_chars();
    let secondary_chars = font.secondary_chars();

    for (i, line) in pattern.iter().enumerate() {
        let line_y = y + i as u16;
        if line_y >= frame_area.height || x >= frame_area.width {
            continue;
        }

        let styled_line = style_line(line, primary, secondary, primary_chars, secondary_chars);
        let width = font.colon_width().min(frame_area.width.saturating_sub(x));
        frame.render_widget(
            Paragraph::new(styled_line),
            Rect::new(x, line_y, width, 1),
        );
    }
}

fn style_line(
    line: &str,
    primary: Color,
    secondary: Color,
    primary_chars: &[char],
    secondary_chars: &[char],
) -> Line<'static> {
    let spans: Vec<Span> = line
        .chars()
        .map(|ch| {
            let style = if primary_chars.contains(&ch) {
                Style::default().fg(primary)
            } else if secondary_chars.contains(&ch) {
                Style::default().fg(secondary)
            } else {
                Style::default()
            };
            Span::styled(ch.to_string(), style)
        })
        .collect();
    Line::from(spans)
}

/// Get the dimensions needed for the timer display with default font
pub fn timer_dimensions() -> (u16, u16) {
    timer_dimensions_for_font(DigitFont::default())
}

/// Get the dimensions needed for the timer display with a specific font
pub fn timer_dimensions_for_font(font: DigitFont) -> (u16, u16) {
    // Width: 4 digits + colon + spacing
    let width = font.width() * 4 + font.colon_width() + 4;
    let height = font.height();
    (width, height)
}
