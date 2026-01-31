use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Plasma effect using sine wave interference patterns
pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Dark purple background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(10, 0, 20)));
    frame.render_widget(bg, area);

    let t = frame_index as f32 * 0.05; // Time factor

    for y in 0..area.height {
        for x in 0..area.width {
            let fx = x as f32 / area.width as f32;
            let fy = y as f32 / area.height as f32;

            // Multiple sine waves combined
            let v1 = fast_sin(fx * 10.0 + t);
            let v2 = fast_sin(fy * 10.0 + t * 0.7);
            let v3 = fast_sin((fx + fy) * 7.0 + t * 1.3);
            let v4 = fast_sin(((fx - 0.5).powi(2) + (fy - 0.5).powi(2)).sqrt() * 12.0 - t);

            // Combine waves
            let value = (v1 + v2 + v3 + v4) / 4.0; // -1 to 1
            let normalized = (value + 1.0) / 2.0;   // 0 to 1

            let (color, ch) = plasma_color_char(normalized, frame_index);

            frame.render_widget(
                Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
}

/// Fast approximation of sine
fn fast_sin(x: f32) -> f32 {
    // Normalize to 0..2π range then approximate
    let x = x % (2.0 * std::f32::consts::PI);
    let x = if x < 0.0 {
        x + 2.0 * std::f32::consts::PI
    } else {
        x
    };

    // Parabola approximation
    if x < std::f32::consts::PI {
        let t = x / std::f32::consts::PI;
        4.0 * t * (1.0 - t) - 1.0 + 1.0 // Shift to -1..1
    } else {
        let t = (x - std::f32::consts::PI) / std::f32::consts::PI;
        -(4.0 * t * (1.0 - t) - 1.0 + 1.0)
    }
}

/// Get color and character based on plasma value
fn plasma_color_char(value: f32, frame_index: usize) -> (Color, char) {
    // Cycle through rainbow based on value + time offset
    let hue = (value + (frame_index as f32 * 0.01)) % 1.0;

    let color = hsv_to_rgb(hue, 0.8, 0.9);

    // Character based on intensity bands
    let ch = if value < 0.2 {
        '░'
    } else if value < 0.4 {
        '▒'
    } else if value < 0.6 {
        '▓'
    } else if value < 0.8 {
        '█'
    } else {
        '▓'
    };

    (color, ch)
}

/// Convert HSV to RGB color
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let h = h * 6.0;
    let i = h.floor() as i32;
    let f = h - i as f32;

    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));

    let (r, g, b) = match i % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };

    Color::Rgb(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
    )
}
