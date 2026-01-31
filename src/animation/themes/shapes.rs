use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Spinning ASCII shape patterns
pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Dark background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(10, 10, 20)));
    frame.render_widget(bg, area);

    let center_x = area.width as f32 / 2.0;
    let center_y = area.height as f32 / 2.0;

    // Rotation angle
    let angle = (frame_index as f32 * 0.03) % (2.0 * std::f32::consts::PI);

    // Draw multiple rotating shapes at different scales
    draw_rotating_shape(frame, area, center_x, center_y, angle, 8.0, 0, frame_index);
    draw_rotating_shape(
        frame,
        area,
        center_x,
        center_y,
        -angle * 0.7,
        15.0,
        1,
        frame_index,
    );
    draw_rotating_shape(
        frame,
        area,
        center_x,
        center_y,
        angle * 0.5,
        22.0,
        2,
        frame_index,
    );

    // Draw some floating particles
    for i in 0..20 {
        let particle_angle = (i as f32 / 20.0) * 2.0 * std::f32::consts::PI + angle * 2.0;
        let particle_dist = 5.0 + (i as f32 % 5.0) * 6.0;

        let px = center_x + particle_angle.cos() * particle_dist;
        let py = center_y + particle_angle.sin() * particle_dist * 0.5;

        if px >= 0.0
            && px < area.width as f32
            && py >= 0.0
            && py < area.height as f32
        {
            let color = particle_color(i, frame_index);
            frame.render_widget(
                Paragraph::new("·").style(Style::default().fg(color)),
                Rect::new(area.x + px as u16, area.y + py as u16, 1, 1),
            );
        }
    }
}

fn draw_rotating_shape(
    frame: &mut Frame,
    area: Rect,
    cx: f32,
    cy: f32,
    angle: f32,
    scale: f32,
    shape_type: usize,
    frame_index: usize,
) {
    // Shape vertices (normalized -1 to 1)
    let vertices: &[(f32, f32)] = match shape_type % 3 {
        0 => &[
            // Square
            (-1.0, -1.0),
            (1.0, -1.0),
            (1.0, 1.0),
            (-1.0, 1.0),
        ],
        1 => &[
            // Triangle
            (0.0, -1.0),
            (1.0, 0.7),
            (-1.0, 0.7),
        ],
        _ => &[
            // Diamond
            (0.0, -1.0),
            (1.0, 0.0),
            (0.0, 1.0),
            (-1.0, 0.0),
        ],
    };

    let color = shape_color(shape_type, frame_index);
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    // Draw edges
    for i in 0..vertices.len() {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % vertices.len()];

        // Rotate and scale
        let rx1 = (x1 * cos_a - y1 * sin_a) * scale + cx;
        let ry1 = (x1 * sin_a + y1 * cos_a) * scale * 0.5 + cy;
        let rx2 = (x2 * cos_a - y2 * sin_a) * scale + cx;
        let ry2 = (x2 * sin_a + y2 * cos_a) * scale * 0.5 + cy;

        // Draw line between points
        draw_line(frame, area, rx1, ry1, rx2, ry2, color);
    }
}

fn draw_line(frame: &mut Frame, area: Rect, x1: f32, y1: f32, x2: f32, y2: f32, color: Color) {
    let steps = ((x2 - x1).abs().max((y2 - y1).abs()) * 2.0) as usize + 1;

    for step in 0..=steps {
        let t = step as f32 / steps as f32;
        let x = x1 + (x2 - x1) * t;
        let y = y1 + (y2 - y1) * t;

        if x >= 0.0 && x < area.width as f32 && y >= 0.0 && y < area.height as f32 {
            let ch = line_char(x2 - x1, y2 - y1);
            frame.render_widget(
                Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                Rect::new(area.x + x as u16, area.y + y as u16, 1, 1),
            );
        }
    }
}

fn line_char(dx: f32, dy: f32) -> char {
    let angle = dy.atan2(dx);
    let normalized = (angle / std::f32::consts::PI * 4.0 + 8.0) as usize % 8;

    match normalized {
        0 | 4 => '─',
        1 | 5 => '╲',
        2 | 6 => '│',
        3 | 7 => '╱',
        _ => '·',
    }
}

fn shape_color(shape_type: usize, frame_index: usize) -> Color {
    let cycle = (frame_index / 10 + shape_type * 3) % 6;
    match cycle {
        0 => Color::Rgb(255, 100, 100), // Red
        1 => Color::Rgb(255, 255, 100), // Yellow
        2 => Color::Rgb(100, 255, 100), // Green
        3 => Color::Rgb(100, 255, 255), // Cyan
        4 => Color::Rgb(100, 100, 255), // Blue
        _ => Color::Rgb(255, 100, 255), // Magenta
    }
}

fn particle_color(idx: usize, frame_index: usize) -> Color {
    let hue = ((idx + frame_index / 5) % 6) as f32 / 6.0;
    hsv_to_rgb(hue)
}

fn hsv_to_rgb(h: f32) -> Color {
    let h = h * 6.0;
    let i = h.floor() as i32;
    let f = h - i as f32;

    let (r, g, b) = match i % 6 {
        0 => (1.0, f, 0.0),
        1 => (1.0 - f, 1.0, 0.0),
        2 => (0.0, 1.0, f),
        3 => (0.0, 1.0 - f, 1.0),
        4 => (f, 0.0, 1.0),
        _ => (1.0, 0.0, 1.0 - f),
    };

    Color::Rgb((r * 200.0) as u8, (g * 200.0) as u8, (b * 200.0) as u8)
}
