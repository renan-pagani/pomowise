use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Ripple structure for expanding circles
struct Ripple {
    x: u16,
    y: u16,
    birth_frame: usize,
    max_radius: u16,
}

impl Ripple {
    fn new(seed: usize, width: u16, height: u16) -> Self {
        let h1 = simple_hash(seed, 1);
        let h2 = simple_hash(seed, 2);
        let h3 = simple_hash(seed, 3);

        Self {
            x: (h1 % width as usize) as u16,
            y: (h2 % height as usize) as u16,
            birth_frame: h3 % 100,
            max_radius: ((h3 % 10) + 5) as u16,
        }
    }

    fn radius_at(&self, frame_index: usize) -> Option<u16> {
        let age = (frame_index as i32 - self.birth_frame as i32) % 100;
        if age < 0 {
            return None;
        }
        let r = (age as u16) / 2;
        if r > self.max_radius {
            None
        } else {
            Some(r)
        }
    }

    fn intensity_at(&self, frame_index: usize) -> f32 {
        let age = (frame_index as i32 - self.birth_frame as i32) % 100;
        if age < 0 {
            return 0.0;
        }
        // Fade out as ripple expands
        1.0 - (age as f32 / (self.max_radius as f32 * 2.0)).min(1.0)
    }
}

fn simple_hash(seed: usize, salt: usize) -> usize {
    let mut h = seed.wrapping_mul(2654435761);
    h ^= salt.wrapping_mul(1597334677);
    h = h.wrapping_mul(2654435761);
    h ^ (h >> 16)
}

/// Check if a point is on a ripple circle
fn point_on_circle(px: u16, py: u16, cx: u16, cy: u16, radius: u16) -> bool {
    let dx = (px as i32 - cx as i32).abs();
    let dy = (py as i32 - cy as i32).abs();

    // Approximate circle using Manhattan distance (for ASCII look)
    let dist = ((dx * dx + dy * dy) as f32).sqrt() as u16;
    dist == radius || dist == radius.saturating_sub(1)
}

/// Rain drop falling
struct RainDrop {
    x: u16,
    start_y: i32,
    speed: u8,
}

impl RainDrop {
    fn new(seed: usize, width: u16) -> Self {
        let h1 = simple_hash(seed, 1);
        let h2 = simple_hash(seed, 2);
        let h3 = simple_hash(seed, 3);

        Self {
            x: (h1 % width as usize) as u16,
            start_y: -((h2 % 30) as i32),
            speed: ((h3 % 3) + 2) as u8,
        }
    }

    fn y_at(&self, frame_index: usize, height: u16) -> Option<u16> {
        let y = self.start_y + ((frame_index / self.speed as usize) as i32);
        let y = y % ((height as i32) + 10);
        if y >= 0 && y < height as i32 {
            Some(y as u16)
        } else {
            None
        }
    }
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Dark blue background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(5, 10, 20)));
    frame.render_widget(bg, area);

    // Create ripples
    let num_ripples = 8;
    let mut ripple_grid: Vec<Vec<(f32, bool)>> =
        vec![vec![(0.0, false); area.width as usize]; area.height as usize];

    for i in 0..num_ripples {
        let ripple = Ripple::new(i * 7919 + (frame_index / 50) * 1000, area.width, area.height);

        if let Some(radius) = ripple.radius_at(frame_index) {
            let intensity = ripple.intensity_at(frame_index);

            for y in 0..area.height {
                for x in 0..area.width {
                    if point_on_circle(x, y, ripple.x, ripple.y, radius) {
                        let current = &mut ripple_grid[y as usize][x as usize];
                        current.0 = (current.0 + intensity).min(1.0);
                        current.1 = true;
                    }
                }
            }
        }
    }

    // Render ripples
    for y in 0..area.height {
        for x in 0..area.width {
            let (intensity, is_ripple) = ripple_grid[y as usize][x as usize];
            if is_ripple && intensity > 0.1 {
                let b = (100.0 + intensity * 155.0) as u8;
                let color = Color::Rgb(50, 150, b);
                let ch = if intensity > 0.7 {
                    '◎'
                } else if intensity > 0.4 {
                    '○'
                } else {
                    '·'
                };

                frame.render_widget(
                    Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }

    // Render falling rain drops
    let num_drops = 30;
    for i in 0..num_drops {
        let drop = RainDrop::new(i * 3571, area.width);
        if let Some(y) = drop.y_at(frame_index, area.height) {
            if drop.x < area.width && y < area.height {
                // Draw drop and trail
                let color = Color::Rgb(100, 180, 220);
                frame.render_widget(
                    Paragraph::new("│").style(Style::default().fg(color)),
                    Rect::new(area.x + drop.x, area.y + y, 1, 1),
                );

                // Short trail above
                if y > 0 {
                    let trail_color = Color::Rgb(50, 100, 150);
                    frame.render_widget(
                        Paragraph::new("·").style(Style::default().fg(trail_color)),
                        Rect::new(area.x + drop.x, area.y + y - 1, 1, 1),
                    );
                }
            }
        }
    }
}
