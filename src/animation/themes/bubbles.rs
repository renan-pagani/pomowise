use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Rising bubbles animation

fn simple_hash(seed: usize, salt: usize) -> usize {
    let mut h = seed.wrapping_mul(2654435761);
    h ^= salt.wrapping_mul(1597334677);
    h = h.wrapping_mul(2654435761);
    h ^ (h >> 16)
}

/// Bubble structure
struct Bubble {
    x: f32,
    start_y: f32,
    size: u8,     // 0-2 for different sizes
    speed: f32,
    wobble_freq: f32,
    wobble_amp: f32,
}

impl Bubble {
    fn new(seed: usize, width: u16, height: u16) -> Self {
        let h1 = simple_hash(seed, 1);
        let h2 = simple_hash(seed, 2);
        let h3 = simple_hash(seed, 3);
        let h4 = simple_hash(seed, 4);
        let h5 = simple_hash(seed, 5);
        
        Self {
            x: (h1 % width as usize) as f32,
            start_y: height as f32 + (h2 % 50) as f32,
            size: (h3 % 3) as u8,
            speed: 0.3 + (h4 % 100) as f32 / 200.0,
            wobble_freq: 0.05 + (h5 % 50) as f32 / 500.0,
            wobble_amp: 1.0 + (h5 % 30) as f32 / 10.0,
        }
    }
    
    fn position(&self, frame_index: usize, height: u16) -> (f32, f32) {
        let total_dist = height as f32 + 60.0;
        let y_offset = (frame_index as f32 * self.speed) % total_dist;
        let y = self.start_y - y_offset;
        
        // Wobble side to side
        let wobble = fast_sin(frame_index as f32 * self.wobble_freq) * self.wobble_amp;
        let x = self.x + wobble;
        
        (x, y)
    }
    
    fn char(&self) -> char {
        match self.size {
            0 => '·',  // Tiny
            1 => '○',  // Medium
            _ => '◯',  // Large
        }
    }
    
    fn highlight_char(&self) -> Option<char> {
        match self.size {
            2 => Some('◌'), // Large bubbles have highlight
            _ => None,
        }
    }
    
    fn color(&self, frame_index: usize) -> Color {
        // Iridescent shimmer effect
        let shimmer = (frame_index as f32 * 0.1 + self.x) % 3.0;
        
        if shimmer < 1.0 {
            Color::Rgb(150, 200, 255) // Blue tint
        } else if shimmer < 2.0 {
            Color::Rgb(200, 150, 255) // Purple tint
        } else {
            Color::Rgb(150, 255, 200) // Green tint
        }
    }
}

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

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Deep water gradient background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(5, 15, 35)));
    frame.render_widget(bg, area);
    
    // Render gradient effect (darker at bottom)
    for y in 0..area.height {
        let depth = y as f32 / area.height as f32;
        let r = (5.0 + depth * 10.0) as u8;
        let g = (15.0 + depth * 15.0) as u8;
        let b = (35.0 + depth * 25.0) as u8;
        
        // Scattered ambient particles
        for x in 0..area.width {
            let particle_chance = simple_hash(x as usize + y as usize * 100, frame_index / 20) % 200;
            if particle_chance < 1 {
                frame.render_widget(
                    Paragraph::new("∘").style(Style::default().fg(Color::Rgb(40, 60, 80))),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }
    
    // Create and render bubbles
    let num_bubbles = 40;
    
    for i in 0..num_bubbles {
        let bubble = Bubble::new(i * 7919, area.width, area.height);
        let (bx, by) = bubble.position(frame_index, area.height);
        
        let x = bx as i16;
        let y = by as i16;
        
        if x >= 0 && x < area.width as i16 && y >= 0 && y < area.height as i16 {
            let color = bubble.color(frame_index);
            let ch = bubble.char();
            
            frame.render_widget(
                Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                Rect::new(area.x + x as u16, area.y + y as u16, 1, 1),
            );
            
            // Add highlight for large bubbles
            if let Some(highlight) = bubble.highlight_char() {
                if x > 0 && y > 0 {
                    frame.render_widget(
                        Paragraph::new(highlight.to_string())
                            .style(Style::default().fg(Color::Rgb(220, 240, 255))),
                        Rect::new(area.x + x as u16 - 1, area.y + y as u16 - 1, 1, 1),
                    );
                }
            }
        }
        
        // Pop effect at top
        if y < 2 && y >= -2 {
            let pop_chars = ['∗', '✧', '·'];
            let pop_idx = (frame_index + i) % 3;
            if x >= 0 && x < area.width as i16 {
                frame.render_widget(
                    Paragraph::new(pop_chars[pop_idx].to_string())
                        .style(Style::default().fg(Color::Rgb(200, 220, 255))),
                    Rect::new(area.x + x as u16, area.y, 1, 1),
                );
            }
        }
    }
    
    // Caustic light patterns at top (light filtering through water surface)
    for x in 0..area.width {
        let caustic_intensity = fast_sin(x as f32 * 0.3 + frame_index as f32 * 0.1);
        if caustic_intensity > 0.5 {
            let brightness = ((caustic_intensity - 0.5) * 100.0) as u8;
            frame.render_widget(
                Paragraph::new("~").style(Style::default().fg(Color::Rgb(50 + brightness, 80 + brightness, 120 + brightness))),
                Rect::new(area.x + x, area.y, 1, 1),
            );
        }
    }
}
