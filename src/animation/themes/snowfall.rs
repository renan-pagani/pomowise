use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Gentle snowfall animation

fn simple_hash(seed: usize, salt: usize) -> usize {
    let mut h = seed.wrapping_mul(2654435761);
    h ^= salt.wrapping_mul(1597334677);
    h = h.wrapping_mul(2654435761);
    h ^ (h >> 16)
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

/// Snowflake structure
struct Snowflake {
    x: f32,
    start_y: f32,
    size: u8,     // 0-2 for different sizes
    speed: f32,
    wobble_freq: f32,
    wobble_amp: f32,
}

impl Snowflake {
    fn new(seed: usize, width: u16, height: u16) -> Self {
        let h1 = simple_hash(seed, 1);
        let h2 = simple_hash(seed, 2);
        let h3 = simple_hash(seed, 3);
        let h4 = simple_hash(seed, 4);
        let h5 = simple_hash(seed, 5);
        
        Self {
            x: (h1 % width as usize) as f32,
            start_y: -((h2 % 30) as f32),
            size: (h3 % 3) as u8,
            speed: 0.15 + (h4 % 100) as f32 / 400.0,
            wobble_freq: 0.02 + (h5 % 30) as f32 / 600.0,
            wobble_amp: 0.5 + (h5 % 20) as f32 / 10.0,
        }
    }
    
    fn position(&self, frame_index: usize, height: u16) -> (f32, f32) {
        let total_dist = height as f32 + 40.0;
        let y_offset = (frame_index as f32 * self.speed) % total_dist;
        let y = self.start_y + y_offset;
        
        // Gentle side-to-side drift
        let wobble = fast_sin(frame_index as f32 * self.wobble_freq + self.x * 0.1) * self.wobble_amp;
        let x = self.x + wobble;
        
        (x, y)
    }
    
    fn char(&self) -> char {
        match self.size {
            0 => '·',  // Tiny
            1 => '∘',  // Medium  
            _ => '❄',  // Large snowflake
        }
    }
    
    fn brightness(&self) -> u8 {
        match self.size {
            0 => 150,
            1 => 200,
            _ => 255,
        }
    }
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Dark winter night sky
    let bg = Block::default().style(Style::default().bg(Color::Rgb(10, 15, 25)));
    frame.render_widget(bg, area);
    
    // Ground accumulation (snow buildup at bottom)
    let snow_line = area.height.saturating_sub(3);
    for y in snow_line..area.height {
        let depth = y - snow_line;
        for x in 0..area.width {
            // Uneven snow surface
            let surface_variation = simple_hash(x as usize, 999) % 3;
            if depth >= surface_variation as u16 {
                let snow_brightness = 200 + (simple_hash(x as usize, y as usize) % 55) as u8;
                let ch = if depth == surface_variation as u16 { '▄' } else { '█' };
                frame.render_widget(
                    Paragraph::new(ch.to_string())
                        .style(Style::default().fg(Color::Rgb(snow_brightness, snow_brightness, snow_brightness))),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }
    
    // Render snowflakes
    let num_flakes = 60;
    
    for i in 0..num_flakes {
        let flake = Snowflake::new(i * 7919, area.width, area.height);
        let (fx, fy) = flake.position(frame_index, area.height);
        
        let x = fx as i16;
        let y = fy as i16;
        
        if x >= 0 && x < area.width as i16 && y >= 0 && y < snow_line as i16 {
            let brightness = flake.brightness();
            let ch = flake.char();
            
            frame.render_widget(
                Paragraph::new(ch.to_string())
                    .style(Style::default().fg(Color::Rgb(brightness, brightness, brightness))),
                Rect::new(area.x + x as u16, area.y + y as u16, 1, 1),
            );
        }
    }
    
    // Distant stars (fewer, dimmer due to clouds)
    for i in 0..15 {
        let h1 = simple_hash(i + 2000, 1);
        let h2 = simple_hash(i + 2000, 2);
        let x = (h1 % area.width as usize) as u16;
        let y = (h2 % (area.height / 3) as usize) as u16; // Only in upper third
        let twinkle = (frame_index + i * 11) % 30 < 20;
        
        if twinkle && x < area.width && y < area.height {
            let brightness = (simple_hash(i, 5) % 60 + 40) as u8;
            frame.render_widget(
                Paragraph::new("·").style(Style::default().fg(Color::Rgb(brightness, brightness, brightness + 20))),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
    
    // Occasional wind gust effect (horizontal streaks)
    if frame_index % 100 < 15 {
        let gust_y = (frame_index % area.height as usize) as u16;
        if gust_y < area.height {
            for x in 0..area.width {
                let show = simple_hash(x as usize, frame_index) % 5 == 0;
                if show {
                    frame.render_widget(
                        Paragraph::new("~").style(Style::default().fg(Color::Rgb(180, 180, 200))),
                        Rect::new(area.x + x, area.y + gust_y, 1, 1),
                    );
                }
            }
        }
    }
}
