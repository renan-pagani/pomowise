use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Fire characters from dense to sparse
const FIRE_CHARS: &[char] = &['█', '▓', '▒', '░', '∙', ' '];

fn simple_hash(x: usize, y: usize, seed: usize) -> usize {
    let mut h = x.wrapping_mul(2654435761);
    h ^= y.wrapping_mul(1597334677);
    h ^= seed;
    h = h.wrapping_mul(2654435761);
    h ^ (h >> 16)
}

/// Get fire intensity at a position (0.0 to 1.0)
fn fire_intensity(x: u16, y: u16, height: u16, frame_index: usize) -> f32 {
    // Base intensity increases toward bottom
    let y_factor = y as f32 / height as f32;
    let base = y_factor * y_factor; // Quadratic falloff upward

    // Add noise/turbulence
    let noise1 = simple_hash(x as usize, frame_index / 2, 1) % 100;
    let noise2 = simple_hash(x as usize + 1, frame_index / 3, 2) % 100;
    let noise3 = simple_hash(x as usize, y as usize + frame_index / 4, 3) % 100;

    let turbulence = (noise1 as f32 + noise2 as f32 + noise3 as f32) / 300.0 - 0.5;

    // Flame tongues - occasional peaks
    let tongue_x = (x as usize + frame_index / 5) % 7;
    let tongue_boost = if tongue_x < 2 { 0.2 } else { 0.0 };

    (base + turbulence * 0.3 + tongue_boost).clamp(0.0, 1.0)
}

/// Get fire color based on intensity
fn fire_color(intensity: f32) -> Color {
    if intensity < 0.2 {
        // Dark/no fire
        Color::Rgb(30, 10, 0)
    } else if intensity < 0.4 {
        // Dark red
        let r = (intensity * 400.0) as u8;
        Color::Rgb(r, 0, 0)
    } else if intensity < 0.6 {
        // Red to orange
        let r = 200 + ((intensity - 0.4) * 275.0) as u8;
        let g = ((intensity - 0.4) * 300.0) as u8;
        Color::Rgb(r, g, 0)
    } else if intensity < 0.8 {
        // Orange to yellow
        let g = 60 + ((intensity - 0.6) * 475.0) as u8;
        Color::Rgb(255, g, 0)
    } else {
        // Yellow to white (hottest)
        let g = 155 + ((intensity - 0.8) * 500.0) as u8;
        let b = ((intensity - 0.8) * 400.0) as u8;
        Color::Rgb(255, g.min(255), b.min(200))
    }
}

fn fire_char(intensity: f32) -> char {
    let idx = ((1.0 - intensity) * (FIRE_CHARS.len() - 1) as f32) as usize;
    FIRE_CHARS[idx.min(FIRE_CHARS.len() - 1)]
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Dark reddish background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(20, 5, 0)));
    frame.render_widget(bg, area);

    // Only render fire in bottom 2/3 of screen
    let fire_start_y = area.height / 3;

    for y in 0..area.height {
        for x in 0..area.width {
            if y < fire_start_y {
                // Above fire zone - occasional ember/spark
                let spark_chance = simple_hash(x as usize, y as usize, frame_index) % 200;
                if spark_chance < 2 {
                    let spark_color = Color::Rgb(255, 200, 50);
                    frame.render_widget(
                        Paragraph::new("·").style(Style::default().fg(spark_color)),
                        Rect::new(area.x + x, area.y + y, 1, 1),
                    );
                }
            } else {
                // In fire zone
                let fire_y = y - fire_start_y;
                let fire_height = area.height - fire_start_y;
                let intensity = fire_intensity(x, fire_y, fire_height, frame_index);

                if intensity > 0.15 {
                    let color = fire_color(intensity);
                    let ch = fire_char(intensity);
                    frame.render_widget(
                        Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                        Rect::new(area.x + x, area.y + y, 1, 1),
                    );
                }
            }
        }
    }
}
