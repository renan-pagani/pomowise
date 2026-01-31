use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Geometric - Rotating fractals, tessellations, expanding/contracting patterns, mathematical beauty

fn simple_hash(x: usize, seed: usize) -> usize {
    let mut h = x.wrapping_mul(2654435761);
    h ^= seed;
    h = h.wrapping_mul(2654435761);
    h ^ (h >> 16)
}

/// Fast sine approximation
fn fast_sin(x: f32) -> f32 {
    let x = x % (2.0 * std::f32::consts::PI);
    let x = if x < 0.0 { x + 2.0 * std::f32::consts::PI } else { x };

    if x < std::f32::consts::PI {
        let t = x / std::f32::consts::PI;
        4.0 * t * (1.0 - t) * 2.0 - 1.0
    } else {
        let t = (x - std::f32::consts::PI) / std::f32::consts::PI;
        -(4.0 * t * (1.0 - t) * 2.0 - 1.0)
    }
}

/// Fast cosine using sine
fn fast_cos(x: f32) -> f32 {
    fast_sin(x + std::f32::consts::PI / 2.0)
}

/// Calculate rotating spiral pattern intensity
fn spiral_intensity(x: u16, y: u16, cx: f32, cy: f32, frame_index: usize) -> f32 {
    let dx = x as f32 - cx;
    let dy = (y as f32 - cy) * 2.0; // Adjust for terminal aspect ratio

    let dist = (dx * dx + dy * dy).sqrt();
    let angle = dy.atan2(dx);

    let t = frame_index as f32 * 0.04;

    // Spiral arms
    let spiral = (angle * 3.0 + dist * 0.2 - t).sin();

    // Pulsing rings
    let ring = (dist * 0.3 - t * 2.0).sin();

    ((spiral + ring) * 0.5 + 0.5).clamp(0.0, 1.0)
}

/// Calculate tessellation pattern
fn tessellation_pattern(x: u16, y: u16, frame_index: usize) -> (f32, usize) {
    let t = frame_index as f32 * 0.02;

    // Hexagonal tessellation
    let scale = 6.0;
    let fx = x as f32 / scale;
    let fy = y as f32 / scale * 1.7; // Adjust for aspect ratio

    // Offset every other row
    let row = fy.floor() as i32;
    let offset = if row % 2 == 0 { 0.5 } else { 0.0 };
    let fx = fx + offset;

    // Find cell center
    let cell_x = fx.floor();
    let cell_y = fy.floor();

    // Distance from cell center
    let dx = fx - cell_x - 0.5;
    let dy = fy - cell_y - 0.5;
    let dist = (dx * dx + dy * dy).sqrt();

    // Pulsing effect
    let pulse = fast_sin(t + (cell_x + cell_y) * 0.5) * 0.3 + 0.7;
    let intensity = (1.0 - dist * 2.0).max(0.0) * pulse;

    // Pattern type based on cell
    let pattern_type = ((cell_x + cell_y).abs() as usize) % 4;

    (intensity, pattern_type)
}

/// Mandelbrot-inspired fractal edge detection
fn fractal_edge(x: u16, y: u16, width: u16, height: u16, frame_index: usize) -> f32 {
    let t = frame_index as f32 * 0.01;

    // Map to complex plane with animation
    let zoom = 2.5 + fast_sin(t * 0.5) * 0.5;
    let cx = -0.5 + fast_cos(t * 0.3) * 0.2;
    let cy = fast_sin(t * 0.4) * 0.2;

    let real = (x as f32 / width as f32 - 0.5) * zoom + cx;
    let imag = (y as f32 / height as f32 - 0.5) * zoom * 0.5 + cy; // Aspect ratio

    // Quick escape time calculation
    let mut zr = 0.0f32;
    let mut zi = 0.0f32;
    let max_iter = 15;
    let mut iter = 0;

    while zr * zr + zi * zi < 4.0 && iter < max_iter {
        let new_zr = zr * zr - zi * zi + real;
        zi = 2.0 * zr * zi + imag;
        zr = new_zr;
        iter += 1;
    }

    if iter == max_iter {
        0.0
    } else {
        (iter as f32 / max_iter as f32).powf(0.5)
    }
}

/// Get geometric character based on intensity and pattern
fn geo_char(intensity: f32, pattern: usize) -> char {
    let chars_by_pattern = [
        ['░', '▒', '▓', '█'],  // Blocks
        ['·', '•', '○', '●'],  // Circles
        ['╌', '─', '═', '▀'],  // Lines
        ['╲', '╳', '╱', '◆'],  // Diamonds
    ];

    let idx = ((intensity * 3.9) as usize).min(3);
    chars_by_pattern[pattern % 4][idx]
}

/// Get color based on position and frame
fn geo_color(x: u16, y: u16, intensity: f32, frame_index: usize) -> Color {
    let t = frame_index as f32 * 0.03;
    let phase = (x as f32 * 0.1 + y as f32 * 0.15 + t) % 3.0;

    let i = (intensity * 200.0) as u8 + 40;

    if phase < 1.0 {
        // Purple/violet
        Color::Rgb(i / 2, i / 4, i)
    } else if phase < 2.0 {
        // Cyan/teal
        Color::Rgb(i / 4, i * 3 / 4, i)
    } else {
        // Gold/amber
        Color::Rgb(i, i * 3 / 4, i / 3)
    }
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Deep dark background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(8, 5, 15)));
    frame.render_widget(bg, area);

    let cx = area.width as f32 / 2.0;
    let cy = area.height as f32 / 2.0;

    for y in 0..area.height {
        for x in 0..area.width {
            // Combine multiple patterns
            let spiral = spiral_intensity(x, y, cx, cy, frame_index);
            let (tess, pattern) = tessellation_pattern(x, y, frame_index);
            let fractal = fractal_edge(x, y, area.width, area.height, frame_index);

            // Layer the patterns
            let combined = (spiral * 0.4 + tess * 0.3 + fractal * 0.3).clamp(0.0, 1.0);

            if combined > 0.1 {
                let ch = geo_char(combined, pattern);
                let color = geo_color(x, y, combined, frame_index);

                frame.render_widget(
                    Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }

    // Add rotating center symbol
    let symbols = ['◇', '◈', '◆', '◇', '❖'];
    let symbol_idx = (frame_index / 10) % symbols.len();
    let center_x = area.width / 2;
    let center_y = area.height / 2;

    if center_x < area.width && center_y < area.height {
        frame.render_widget(
            Paragraph::new(symbols[symbol_idx].to_string())
                .style(Style::default().fg(Color::Rgb(255, 220, 100))),
            Rect::new(area.x + center_x, area.y + center_y, 1, 1),
        );
    }
}
