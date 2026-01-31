use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Electric/Lightning theme - crackling energy bolts

fn simple_hash(seed: usize, salt: usize) -> usize {
    let mut h = seed.wrapping_mul(2654435761);
    h ^= salt.wrapping_mul(1597334677);
    h = h.wrapping_mul(2654435761);
    h ^ (h >> 16)
}

/// Lightning bolt structure
struct Bolt {
    start_x: u16,
    start_y: u16,
    seed: usize,
    lifetime: usize,
    birth_frame: usize,
}

impl Bolt {
    fn new(seed: usize, width: u16, height: u16, frame_index: usize) -> Self {
        let h1 = simple_hash(seed, 1);
        let h2 = simple_hash(seed, 2);
        let h3 = simple_hash(seed, 3);
        
        Self {
            start_x: (h1 % width as usize) as u16,
            start_y: 0,
            seed,
            lifetime: 5 + h2 % 10,
            birth_frame: (h3 % 50) + (frame_index / 50) * 50,
        }
    }
    
    fn is_active(&self, frame_index: usize) -> bool {
        let age = frame_index.saturating_sub(self.birth_frame);
        age < self.lifetime
    }
    
    fn brightness(&self, frame_index: usize) -> u8 {
        let age = frame_index.saturating_sub(self.birth_frame);
        if age >= self.lifetime {
            return 0;
        }
        
        // Flash bright then fade
        let progress = age as f32 / self.lifetime as f32;
        if progress < 0.2 {
            255
        } else {
            ((1.0 - progress) * 255.0) as u8
        }
    }
    
    /// Generate bolt path points
    fn path(&self, height: u16) -> Vec<(u16, u16)> {
        let mut points = Vec::new();
        let mut x = self.start_x as i16;
        let mut y = self.start_y;
        
        points.push((x as u16, y));
        
        while y < height {
            // Random zigzag
            let h = simple_hash(self.seed + y as usize, x as usize);
            let dx = (h % 5) as i16 - 2; // -2 to 2
            x = (x + dx).max(0);
            y += 1 + (h % 2) as u16;
            
            points.push((x as u16, y.min(height - 1)));
            
            // Occasional branch
            if h % 10 < 2 && y < height - 5 {
                let branch_len = 3 + (h % 4) as u16;
                let branch_dir = if h % 2 == 0 { 1i16 } else { -1i16 };
                for i in 1..=branch_len {
                    let bx = (x + branch_dir * i as i16).max(0) as u16;
                    let by = y + i;
                    if by < height {
                        points.push((bx, by));
                    }
                }
            }
        }
        
        points
    }
}

/// Electric arc between two points
struct Arc {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
    seed: usize,
}

impl Arc {
    fn new(seed: usize, width: u16, height: u16) -> Self {
        let h1 = simple_hash(seed, 1);
        let h2 = simple_hash(seed, 2);
        let h3 = simple_hash(seed, 3);
        let h4 = simple_hash(seed, 4);
        
        Self {
            x1: (h1 % width as usize) as u16,
            y1: (h2 % height as usize) as u16,
            x2: (h3 % width as usize) as u16,
            y2: (h4 % height as usize) as u16,
            seed,
        }
    }
    
    fn points(&self, frame_index: usize) -> Vec<(u16, u16, char)> {
        let mut pts = Vec::new();
        
        let dx = self.x2 as i16 - self.x1 as i16;
        let dy = self.y2 as i16 - self.y1 as i16;
        let steps = dx.abs().max(dy.abs()) as usize;
        
        if steps == 0 {
            return pts;
        }
        
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let noise_x = simple_hash(self.seed + i + frame_index, 1) % 3;
            let noise_y = simple_hash(self.seed + i + frame_index, 2) % 3;
            
            let x = (self.x1 as f32 + dx as f32 * t + noise_x as f32 - 1.0) as u16;
            let y = (self.y1 as f32 + dy as f32 * t + noise_y as f32 - 1.0) as u16;
            
            let ch = if i % 3 == 0 { '⚡' } else if i % 2 == 0 { '╳' } else { '·' };
            pts.push((x, y, ch));
        }
        
        pts
    }
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Dark stormy background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(10, 10, 20)));
    frame.render_widget(bg, area);
    
    // Ambient electric particles
    for i in 0..50 {
        let h1 = simple_hash(i + frame_index / 5, 1);
        let h2 = simple_hash(i + frame_index / 5, 2);
        let x = (h1 % area.width as usize) as u16;
        let y = (h2 % area.height as usize) as u16;
        
        let flicker = (frame_index + i) % 3 != 0;
        if flicker && x < area.width && y < area.height {
            let intensity = (simple_hash(i, 5) % 100 + 30) as u8;
            frame.render_widget(
                Paragraph::new("·").style(Style::default().fg(Color::Rgb(intensity, intensity, intensity + 50))),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
    
    // Main lightning bolts
    let num_bolts = 3;
    for i in 0..num_bolts {
        let bolt = Bolt::new(i * 7919 + (frame_index / 30) * 1000, area.width, area.height, frame_index);
        
        if bolt.is_active(frame_index) {
            let brightness = bolt.brightness(frame_index);
            let path = bolt.path(area.height);
            
            for (px, py) in path {
                if px < area.width && py < area.height {
                    // Core (brightest)
                    let core_color = Color::Rgb(brightness, brightness, 255);
                    frame.render_widget(
                        Paragraph::new("│").style(Style::default().fg(core_color)),
                        Rect::new(area.x + px, area.y + py, 1, 1),
                    );
                    
                    // Glow around bolt
                    if brightness > 100 {
                        let glow_intensity = brightness / 3;
                        let glow_color = Color::Rgb(glow_intensity / 2, glow_intensity / 2, glow_intensity);
                        
                        if px > 0 {
                            frame.render_widget(
                                Paragraph::new("░").style(Style::default().fg(glow_color)),
                                Rect::new(area.x + px - 1, area.y + py, 1, 1),
                            );
                        }
                        if px + 1 < area.width {
                            frame.render_widget(
                                Paragraph::new("░").style(Style::default().fg(glow_color)),
                                Rect::new(area.x + px + 1, area.y + py, 1, 1),
                            );
                        }
                    }
                }
            }
        }
    }
    
    // Small arcs between random points
    let num_arcs = 5;
    for i in 0..num_arcs {
        let arc_seed = i * 3571 + (frame_index / 20);
        let active = simple_hash(arc_seed, 100) % 4 == 0;
        
        if active {
            let arc = Arc::new(arc_seed, area.width, area.height);
            let points = arc.points(frame_index);
            
            for (px, py, ch) in points {
                if px < area.width && py < area.height {
                    let color = Color::Rgb(100, 150, 255);
                    frame.render_widget(
                        Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                        Rect::new(area.x + px, area.y + py, 1, 1),
                    );
                }
            }
        }
    }
    
    // Tesla coil in center (decorative)
    let coil_x = area.width / 2;
    let coil_y = area.height - 3;
    
    if coil_y > 0 && coil_x < area.width {
        // Coil base
        let coil_chars = ['╥', '║', '╨'];
        for (i, ch) in coil_chars.iter().enumerate() {
            let y = coil_y + i as u16;
            if y < area.height {
                frame.render_widget(
                    Paragraph::new(ch.to_string())
                        .style(Style::default().fg(Color::Rgb(100, 100, 120))),
                    Rect::new(area.x + coil_x, area.y + y, 1, 1),
                );
            }
        }
        
        // Sparks from coil top
        let spark_active = frame_index % 5 < 3;
        if spark_active && coil_y > 0 {
            let spark_color = Color::Rgb(200, 220, 255);
            for dx in [-1i16, 0, 1] {
                let sx = (coil_x as i16 + dx) as u16;
                if sx < area.width {
                    frame.render_widget(
                        Paragraph::new("*").style(Style::default().fg(spark_color)),
                        Rect::new(area.x + sx, area.y + coil_y - 1, 1, 1),
                    );
                }
            }
        }
    }
}
