use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};
use std::time::SystemTime;

/// Seasonal - Changes based on current month: spring flowers, summer sun, autumn leaves, winter snow

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

#[derive(Clone, Copy)]
enum Season {
    Spring, // March, April, May
    Summer, // June, July, August
    Autumn, // September, October, November
    Winter, // December, January, February
}

impl Season {
    fn from_month(month: u32) -> Season {
        match month {
            3 | 4 | 5 => Season::Spring,
            6 | 7 | 8 => Season::Summer,
            9 | 10 | 11 => Season::Autumn,
            _ => Season::Winter,
        }
    }

    fn current() -> Season {
        // Get current month from system time
        let now = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Approximate month calculation (days since epoch / 30 % 12 + 1)
        // More accurate: calculate from seconds
        let days = now / 86400;
        let years = days / 365;
        let day_of_year = days - years * 365;

        // Rough month approximation
        let month = (day_of_year / 30 + 1).min(12) as u32;

        Season::from_month(month)
    }
}

// ============ SPRING RENDERING ============

fn render_spring(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Soft spring sky
    let bg = Block::default().style(Style::default().bg(Color::Rgb(180, 210, 230)));
    frame.render_widget(bg, area);

    // Draw grass
    for x in 0..area.width {
        let grass_height = 2 + (simple_hash(x as usize, 1) % 2) as u16;
        for dy in 0..grass_height {
            let y = area.height.saturating_sub(dy + 1);
            let green = 100 + (simple_hash(x as usize + dy as usize, 2) % 50) as u8;
            frame.render_widget(
                Paragraph::new("▓").style(Style::default().fg(Color::Rgb(50, green, 50))),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }

    // Floating flower petals
    let petal_colors = [
        Color::Rgb(255, 180, 200), // Pink
        Color::Rgb(255, 255, 180), // Yellow
        Color::Rgb(200, 180, 255), // Lavender
        Color::Rgb(255, 200, 200), // Light pink
    ];

    for i in 0..20 {
        let h1 = simple_hash(i, 10);
        let h2 = simple_hash(i, 11);
        let base_x = (h1 % area.width as usize) as f32;
        let base_y = (h2 % (area.height as usize * 2)) as f32;

        let t = frame_index as f32;
        let sway = fast_sin(t * 0.05 + i as f32 * 0.5) * 3.0;
        let fall = (base_y + t * 0.1) % (area.height as f32 + 10.0);

        let x = (base_x + sway) as u16;
        let y = fall as u16;

        if x < area.width && y < area.height.saturating_sub(3) {
            let color = petal_colors[h1 % petal_colors.len()];
            let ch = if h2 % 2 == 0 { '•' } else { '·' };
            frame.render_widget(
                Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }

    // Spring flowers on ground
    for i in 0..15 {
        let x = (simple_hash(i + 100, 20) % area.width as usize) as u16;
        let y = area.height.saturating_sub(3);

        if x < area.width && y < area.height {
            let flower_color = petal_colors[simple_hash(i, 21) % petal_colors.len()];
            frame.render_widget(
                Paragraph::new("*").style(Style::default().fg(flower_color)),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
}

// ============ SUMMER RENDERING ============

fn render_summer(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Bright summer sky gradient
    for y in 0..area.height {
        let gradient = (y as f32 / area.height as f32 * 50.0) as u8;
        let sky_color = Color::Rgb(100 + gradient, 180 + gradient / 2, 255 - gradient);
        for x in 0..area.width {
            frame.render_widget(
                Paragraph::new(" ").style(Style::default().bg(sky_color)),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }

    // Draw sun with rays
    let sun_x = area.width / 4;
    let sun_y = area.height / 4;
    let t = frame_index as f32 * 0.05;

    // Sun body
    for dy in -2i16..=2 {
        for dx in -3i16..=3 {
            let x = (sun_x as i16 + dx).clamp(0, area.width as i16 - 1) as u16;
            let y = (sun_y as i16 + dy).clamp(0, area.height as i16 - 1) as u16;
            let dist = ((dx * dx + dy * dy * 2) as f32).sqrt();
            if dist < 3.0 {
                frame.render_widget(
                    Paragraph::new("█").style(Style::default().fg(Color::Rgb(255, 220, 50))),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }

    // Sun rays (animated)
    let ray_chars = ['─', '│', '╲', '╱', '*'];
    for i in 0..8 {
        let angle = i as f32 * std::f32::consts::PI / 4.0 + t * 0.1;
        let pulse = fast_sin(t + i as f32) * 0.5 + 1.5;
        let ray_len = (4.0 * pulse) as i16;

        for r in 3..ray_len + 3 {
            let dx = (angle.cos() * r as f32 * 1.5) as i16;
            let dy = (angle.sin() * r as f32 * 0.8) as i16;
            let x = (sun_x as i16 + dx).clamp(0, area.width as i16 - 1) as u16;
            let y = (sun_y as i16 + dy).clamp(0, area.height as i16 - 1) as u16;

            let brightness = 255 - (r * 20).min(100) as u8;
            frame.render_widget(
                Paragraph::new(ray_chars[i % ray_chars.len()].to_string())
                    .style(Style::default().fg(Color::Rgb(255, brightness, 50))),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }

    // Green grass
    for x in 0..area.width {
        for dy in 0..2 {
            let y = area.height.saturating_sub(dy + 1);
            let green = 130 + (simple_hash(x as usize, 30) % 40) as u8;
            frame.render_widget(
                Paragraph::new("▓").style(Style::default().fg(Color::Rgb(50, green, 30))),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
}

// ============ AUTUMN RENDERING ============

fn render_autumn(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Warm autumn sky
    let bg = Block::default().style(Style::default().bg(Color::Rgb(60, 40, 50)));
    frame.render_widget(bg, area);

    // Autumn sky gradient
    for y in 0..area.height / 2 {
        let gradient = (y as f32 / (area.height as f32 / 2.0) * 40.0) as u8;
        let sky_color = Color::Rgb(80 + gradient, 50 + gradient / 2, 40);
        for x in 0..area.width {
            if simple_hash(x as usize + y as usize * 100, 40) % 10 == 0 {
                frame.render_widget(
                    Paragraph::new("·").style(Style::default().fg(sky_color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }

    // Falling leaves
    let leaf_colors = [
        Color::Rgb(200, 80, 20),   // Orange
        Color::Rgb(180, 50, 30),   // Red-orange
        Color::Rgb(220, 150, 40),  // Gold
        Color::Rgb(150, 60, 20),   // Brown
        Color::Rgb(200, 100, 30),  // Amber
    ];

    for i in 0..30 {
        let h1 = simple_hash(i, 50);
        let h2 = simple_hash(i, 51);
        let h3 = simple_hash(i, 52);

        let base_x = (h1 % area.width as usize) as f32;
        let base_y = (h2 % (area.height as usize * 2)) as f32;

        let t = frame_index as f32;
        let sway = fast_sin(t * 0.03 + i as f32 * 0.7) * 4.0;
        let tumble = fast_sin(t * 0.08 + i as f32) * 2.0;
        let fall = (base_y + t * 0.15 + h3 as f32 * 0.01) % (area.height as f32 + 15.0);

        let x = ((base_x + sway + tumble) as u16).min(area.width.saturating_sub(1));
        let y = fall as u16;

        if y < area.height.saturating_sub(2) {
            let color = leaf_colors[h1 % leaf_colors.len()];
            let chars = ['•', '·', '▪', '○'];
            let ch = chars[h3 % chars.len()];
            frame.render_widget(
                Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }

    // Leaf pile on ground
    for x in 0..area.width {
        for dy in 0..3 {
            let y = area.height.saturating_sub(dy + 1);
            let color = leaf_colors[simple_hash(x as usize + dy as usize, 60) % leaf_colors.len()];
            let ch = if dy == 0 { '▓' } else { '▒' };
            frame.render_widget(
                Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
}

// ============ WINTER RENDERING ============

fn render_winter(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Cold winter night sky
    let bg = Block::default().style(Style::default().bg(Color::Rgb(15, 20, 35)));
    frame.render_widget(bg, area);

    // Stars
    for i in 0..20 {
        let h1 = simple_hash(i + 200, 70);
        let h2 = simple_hash(i + 200, 71);
        let x = (h1 % area.width as usize) as u16;
        let y = (h2 % (area.height as usize / 2)) as u16;

        let twinkle = (frame_index + i * 13) % 30 < 25;
        if twinkle && x < area.width && y < area.height {
            let brightness = 150 + (simple_hash(i, 72) % 100) as u8;
            frame.render_widget(
                Paragraph::new("·").style(Style::default().fg(Color::Rgb(brightness, brightness, 255))),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }

    // Falling snowflakes
    let snow_chars = ['*', '·', '•', '○', '+'];

    for i in 0..40 {
        let h1 = simple_hash(i, 80);
        let h2 = simple_hash(i, 81);
        let h3 = simple_hash(i, 82);

        let base_x = (h1 % area.width as usize) as f32;
        let base_y = (h2 % (area.height as usize * 2)) as f32;

        let t = frame_index as f32;
        let sway = fast_sin(t * 0.02 + i as f32 * 0.3) * 2.0;
        let drift = fast_sin(t * 0.05 + i as f32 * 0.7) * 1.0;
        let fall_speed = 0.1 + (h3 % 100) as f32 / 500.0;
        let fall = (base_y + t * fall_speed) % (area.height as f32 + 10.0);

        let x = ((base_x + sway + drift) as u16).min(area.width.saturating_sub(1));
        let y = fall as u16;

        if y < area.height.saturating_sub(3) {
            let ch = snow_chars[h3 % snow_chars.len()];
            let brightness = 180 + (h1 % 75) as u8;
            frame.render_widget(
                Paragraph::new(ch.to_string())
                    .style(Style::default().fg(Color::Rgb(brightness, brightness, 255))),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }

    // Snow on ground
    for x in 0..area.width {
        let pile_height = 2 + (simple_hash(x as usize, 90) % 2) as u16;
        for dy in 0..pile_height {
            let y = area.height.saturating_sub(dy + 1);
            let brightness = 200 + (simple_hash(x as usize + dy as usize, 91) % 55) as u8;
            let ch = if dy == 0 { '▓' } else { '░' };
            frame.render_widget(
                Paragraph::new(ch.to_string())
                    .style(Style::default().fg(Color::Rgb(brightness, brightness, 255))),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    let season = Season::current();

    match season {
        Season::Spring => render_spring(frame, area, frame_index),
        Season::Summer => render_summer(frame, area, frame_index),
        Season::Autumn => render_autumn(frame, area, frame_index),
        Season::Winter => render_winter(frame, area, frame_index),
    }
}
