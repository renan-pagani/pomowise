use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Nature - Falling leaves, gentle forest breeze, tree silhouettes, peaceful green palette

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

/// Leaf structure for animation
struct Leaf {
    x: f32,
    y: f32,
    fall_speed: f32,
    sway_phase: f32,
    sway_amount: f32,
    char_idx: usize,
    color_idx: usize,
}

impl Leaf {
    fn new(idx: usize, width: u16, height: u16) -> Self {
        let h1 = simple_hash(idx, 1);
        let h2 = simple_hash(idx, 2);
        let h3 = simple_hash(idx, 3);
        let h4 = simple_hash(idx, 4);
        let h5 = simple_hash(idx, 5);
        let h6 = simple_hash(idx, 6);

        Leaf {
            x: (h1 % width as usize) as f32,
            y: (h2 % (height as usize * 2)) as f32 - height as f32,
            fall_speed: 0.15 + (h3 % 100) as f32 / 400.0,
            sway_phase: (h4 % 628) as f32 / 100.0,
            sway_amount: 1.0 + (h5 % 30) as f32 / 10.0,
            char_idx: h6 % 4,
            color_idx: h6 % 5,
        }
    }

    fn update(&self, frame_index: usize, height: u16) -> (f32, f32) {
        let t = frame_index as f32;
        let y = (self.y + t * self.fall_speed) % (height as f32 + 10.0);
        let sway = fast_sin(t * 0.05 + self.sway_phase) * self.sway_amount;
        let x = self.x + sway;
        (x, y)
    }

    fn get_char(&self) -> char {
        match self.char_idx {
            0 => '🍂',
            1 => '🍃',
            2 => '·',
            _ => '•',
        }
    }

    fn get_color(&self) -> Color {
        match self.color_idx {
            0 => Color::Rgb(180, 100, 40),   // Brown leaf
            1 => Color::Rgb(200, 150, 50),   // Golden leaf
            2 => Color::Rgb(100, 160, 80),   // Green leaf
            3 => Color::Rgb(220, 120, 30),   // Orange leaf
            _ => Color::Rgb(150, 180, 90),   // Light green
        }
    }
}

/// Draw a tree silhouette
fn draw_tree(frame: &mut Frame, area: Rect, tree_x: u16, frame_index: usize) {
    let trunk_color = Color::Rgb(60, 40, 20);
    let leaf_colors = [
        Color::Rgb(30, 80, 30),
        Color::Rgb(40, 90, 35),
        Color::Rgb(25, 70, 25),
    ];

    // Tree trunk
    let trunk_height = 6;
    let trunk_y = area.height.saturating_sub(trunk_height);
    for y in trunk_y..area.height {
        if tree_x < area.width {
            frame.render_widget(
                Paragraph::new("█").style(Style::default().fg(trunk_color)),
                Rect::new(area.x + tree_x, area.y + y, 1, 1),
            );
        }
    }

    // Tree canopy (triangular shape with slight movement)
    let sway = (fast_sin(frame_index as f32 * 0.03) * 0.5) as i16;
    let canopy_rows = [
        (0, 1),   // top
        (-1, 3),  // middle
        (-2, 5),  // bottom
        (-1, 3),  // extra fullness
    ];

    for (row_idx, (offset, width)) in canopy_rows.iter().enumerate() {
        let y = trunk_y.saturating_sub(row_idx as u16 + 1);
        if y < area.height {
            for dx in 0..*width {
                let x = (tree_x as i16 + offset + dx as i16 + sway).clamp(0, area.width as i16 - 1) as u16;
                let color = leaf_colors[simple_hash(tree_x as usize + dx + row_idx * 10, 7) % 3];
                frame.render_widget(
                    Paragraph::new("▓").style(Style::default().fg(color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }
}

/// Draw grass at the bottom
fn draw_grass(frame: &mut Frame, area: Rect, frame_index: usize) {
    let grass_chars = ['▒', '░', '·'];
    let grass_colors = [
        Color::Rgb(40, 100, 40),
        Color::Rgb(50, 120, 50),
        Color::Rgb(35, 90, 35),
    ];

    let grass_height = 2;
    for y in (area.height.saturating_sub(grass_height))..area.height {
        for x in 0..area.width {
            let wave = (fast_sin(x as f32 * 0.3 + frame_index as f32 * 0.05) * 0.5 + 0.5) as usize;
            let char_idx = (simple_hash(x as usize, 10) + wave) % 3;
            let color_idx = simple_hash(x as usize + y as usize, 11) % 3;

            frame.render_widget(
                Paragraph::new(grass_chars[char_idx].to_string())
                    .style(Style::default().fg(grass_colors[color_idx])),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Forest green gradient background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(15, 30, 20)));
    frame.render_widget(bg, area);

    // Draw sky gradient (lighter at horizon)
    for y in 0..area.height.saturating_sub(3) {
        let gradient = (y as f32 / area.height as f32 * 15.0) as u8;
        let sky_color = Color::Rgb(15 + gradient, 30 + gradient, 25 + gradient / 2);
        for x in 0..area.width {
            if simple_hash(x as usize + y as usize * 100, 20) % 30 == 0 {
                frame.render_widget(
                    Paragraph::new("·").style(Style::default().fg(sky_color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }

    // Draw trees at various positions
    let tree_positions: [u16; 5] = [5, 15, 28, 42, 55];
    for &tree_x in &tree_positions {
        if tree_x < area.width {
            draw_tree(frame, area, tree_x, frame_index);
        }
    }

    // Draw grass
    draw_grass(frame, area, frame_index);

    // Animate falling leaves
    let num_leaves = 25;
    for i in 0..num_leaves {
        let leaf = Leaf::new(i, area.width, area.height);
        let (x, y) = leaf.update(frame_index, area.height);

        if y >= 0.0 && (y as u16) < area.height.saturating_sub(2) && (x as u16) < area.width {
            // Use simple chars for compatibility
            let leaf_char = if leaf.char_idx < 2 { '•' } else { '·' };
            frame.render_widget(
                Paragraph::new(leaf_char.to_string())
                    .style(Style::default().fg(leaf.get_color())),
                Rect::new(area.x + x as u16, area.y + y as u16, 1, 1),
            );
        }
    }

    // Add gentle breeze particles
    let breeze_count = 15;
    for i in 0..breeze_count {
        let h1 = simple_hash(i + 1000, 1);
        let h2 = simple_hash(i + 1000, 2);
        let y = (h2 % area.height as usize) as u16;
        let x_base = (h1 % area.width as usize) as f32;
        let x = (x_base + frame_index as f32 * 0.3) % area.width as f32;

        if (x as u16) < area.width && y < area.height.saturating_sub(3) {
            let breeze_color = Color::Rgb(100, 140, 100);
            frame.render_widget(
                Paragraph::new("~").style(Style::default().fg(breeze_color)),
                Rect::new(area.x + x as u16, area.y + y, 1, 1),
            );
        }
    }
}
