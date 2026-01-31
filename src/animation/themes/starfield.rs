use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Star structure
struct Star {
    x: f32,      // -1.0 to 1.0 (center at 0)
    y: f32,      // -1.0 to 1.0 (center at 0)
    z: f32,      // 0.0 to 1.0 (depth, smaller = farther)
    brightness: u8,
}

impl Star {
    fn new(seed: usize) -> Self {
        let h1 = simple_hash(seed, 1);
        let h2 = simple_hash(seed, 2);
        let h3 = simple_hash(seed, 3);
        let h4 = simple_hash(seed, 4);

        Self {
            x: (h1 % 2000) as f32 / 1000.0 - 1.0,
            y: (h2 % 2000) as f32 / 1000.0 - 1.0,
            z: (h3 % 1000) as f32 / 1000.0,
            brightness: (h4 % 200 + 55) as u8,
        }
    }

    fn project(&self, width: u16, height: u16, frame_index: usize) -> Option<(u16, u16, u8)> {
        // Z moves toward viewer over time (wrapping)
        let speed = 0.005;
        let z = (self.z - (frame_index as f32 * speed)) % 1.0;
        let z = if z < 0.0 { z + 1.0 } else { z };

        // Skip if too close (would be off screen)
        if z < 0.05 {
            return None;
        }

        // Perspective projection
        let scale = 1.0 / z;
        let screen_x = (self.x * scale + 1.0) * width as f32 / 2.0;
        let screen_y = (self.y * scale + 1.0) * height as f32 / 2.0;

        // Check bounds
        if screen_x < 0.0 || screen_x >= width as f32 || screen_y < 0.0 || screen_y >= height as f32
        {
            return None;
        }

        // Brightness increases as star gets closer
        let brightness = ((1.0 - z) * self.brightness as f32) as u8;

        Some((screen_x as u16, screen_y as u16, brightness))
    }
}

fn simple_hash(seed: usize, salt: usize) -> usize {
    let mut h = seed.wrapping_mul(2654435761);
    h ^= salt.wrapping_mul(1597334677);
    h = h.wrapping_mul(2654435761);
    h ^ (h >> 16)
}

/// Star characters based on brightness
fn star_char(brightness: u8) -> char {
    if brightness > 200 {
        '★'
    } else if brightness > 150 {
        '✦'
    } else if brightness > 100 {
        '✧'
    } else if brightness > 50 {
        '·'
    } else {
        '.'
    }
}

/// Star color based on seed (white, pale blue, pale yellow)
fn star_color(seed: usize, brightness: u8) -> Color {
    let color_type = seed % 10;
    let b = brightness;

    match color_type {
        0..=5 => Color::Rgb(b, b, b),                           // White
        6..=7 => Color::Rgb(b * 9 / 10, b * 9 / 10, b),        // Pale blue
        _ => Color::Rgb(b, b, b * 9 / 10),                      // Pale yellow
    }
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Deep space background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(0, 0, 15)));
    frame.render_widget(bg, area);

    // Create and render stars
    let num_stars = 150;

    for i in 0..num_stars {
        let star = Star::new(i * 7919);

        if let Some((sx, sy, brightness)) = star.project(area.width, area.height, frame_index) {
            let ch = star_char(brightness);
            let color = star_color(i, brightness);

            if sx < area.width && sy < area.height {
                frame.render_widget(
                    Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                    Rect::new(area.x + sx, area.y + sy, 1, 1),
                );
            }
        }
    }

    // Add some distant static stars (small dots)
    for i in 0..50 {
        let h1 = simple_hash(i + 1000, 1);
        let h2 = simple_hash(i + 1000, 2);
        let x = (h1 % area.width as usize) as u16;
        let y = (h2 % area.height as usize) as u16;
        let twinkle = (frame_index + i) % 30 < 25; // Occasional twinkle off

        if twinkle && x < area.width && y < area.height {
            frame.render_widget(
                Paragraph::new(".").style(Style::default().fg(Color::Rgb(60, 60, 80))),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
}
