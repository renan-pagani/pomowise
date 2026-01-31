use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Aurora Borealis - flowing curtains of colorful light

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

/// Get aurora intensity at a position
fn aurora_intensity(x: u16, y: u16, width: u16, height: u16, frame_index: usize) -> f32 {
    let t = frame_index as f32 * 0.03;
    let fx = x as f32 / width as f32;
    let fy = y as f32 / height as f32;
    
    // Aurora appears in upper portion of sky
    let y_factor = 1.0 - fy; // Stronger at top
    let y_falloff = (y_factor * 2.0).min(1.0);
    
    // Flowing wave patterns
    let wave1 = fast_sin(fx * 4.0 + t);
    let wave2 = fast_sin(fx * 7.0 - t * 0.7 + 1.0);
    let wave3 = fast_sin(fx * 3.0 + t * 1.3 + fy * 2.0);
    
    // Vertical curtain effect
    let curtain_base = (y_factor - 0.3).max(0.0) * 2.0;
    let curtain_wave = fast_sin(fx * 10.0 + t * 0.5) * 0.2;
    let curtain_height = curtain_base + curtain_wave;
    
    // Check if within curtain
    let in_curtain = fy < (0.7 + wave1 * 0.15 + wave2 * 0.1);
    
    if !in_curtain {
        return 0.0;
    }
    
    let combined = (wave1 + wave2 + wave3) / 3.0;
    let intensity = (combined * 0.5 + 0.5) * y_falloff * curtain_height;
    
    intensity.clamp(0.0, 1.0)
}

/// Get aurora color based on position and intensity
fn aurora_color(x: u16, width: u16, intensity: f32, frame_index: usize) -> Color {
    let t = frame_index as f32 * 0.02;
    let fx = x as f32 / width as f32;
    
    // Color shifts across the aurora
    let color_phase = (fx * 2.0 + t) % 3.0;
    
    let i = (intensity * 255.0) as u8;
    
    if color_phase < 1.0 {
        // Green (most common aurora color)
        Color::Rgb(i / 4, i, i / 3)
    } else if color_phase < 2.0 {
        // Cyan to blue
        Color::Rgb(i / 6, i * 3 / 4, i)
    } else {
        // Purple/pink (rare aurora)
        Color::Rgb(i / 2, i / 4, i)
    }
}

fn aurora_char(intensity: f32) -> char {
    if intensity > 0.7 {
        '█'
    } else if intensity > 0.5 {
        '▓'
    } else if intensity > 0.3 {
        '▒'
    } else if intensity > 0.15 {
        '░'
    } else {
        ' '
    }
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Dark night sky background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(5, 5, 15)));
    frame.render_widget(bg, area);
    
    // Render aurora
    for y in 0..area.height {
        for x in 0..area.width {
            let intensity = aurora_intensity(x, y, area.width, area.height, frame_index);
            
            if intensity > 0.1 {
                let color = aurora_color(x, area.width, intensity, frame_index);
                let ch = aurora_char(intensity);
                
                frame.render_widget(
                    Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }
    
    // Add some stars in the background
    for i in 0..30 {
        let h1 = simple_hash(i + 500, 1);
        let h2 = simple_hash(i + 500, 2);
        let x = (h1 % area.width as usize) as u16;
        let y = (h2 % area.height as usize) as u16;
        let twinkle = (frame_index + i * 7) % 20 < 17;
        
        if twinkle && x < area.width && y < area.height {
            // Only show stars where aurora is dim
            let aurora_here = aurora_intensity(x, y, area.width, area.height, frame_index);
            if aurora_here < 0.2 {
                let brightness = (simple_hash(i, 5) % 100 + 50) as u8;
                frame.render_widget(
                    Paragraph::new("·").style(Style::default().fg(Color::Rgb(brightness, brightness, brightness))),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }
}
