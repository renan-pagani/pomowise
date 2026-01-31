use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Ocean waves - rolling waves with foam and depth

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

fn simple_hash(x: usize, seed: usize) -> usize {
    let mut h = x.wrapping_mul(2654435761);
    h ^= seed;
    h = h.wrapping_mul(2654435761);
    h ^ (h >> 16)
}

/// Get wave height at a given x position
fn wave_height(x: u16, width: u16, frame_index: usize, wave_layer: usize) -> f32 {
    let t = frame_index as f32 * 0.08;
    let fx = x as f32 / width as f32;
    
    // Multiple wave frequencies combined
    let speed_mult = 1.0 + wave_layer as f32 * 0.3;
    let w1 = fast_sin(fx * 6.0 + t * speed_mult) * 0.4;
    let w2 = fast_sin(fx * 12.0 - t * speed_mult * 0.7) * 0.2;
    let w3 = fast_sin(fx * 3.0 + t * speed_mult * 0.5) * 0.3;
    
    w1 + w2 + w3
}

/// Check if position is in a wave crest (foam zone)
fn is_foam(x: u16, y: u16, width: u16, height: u16, frame_index: usize, wave_y: u16) -> bool {
    let wave_h = wave_height(x, width, frame_index, 0);
    let wave_top = wave_y as f32 + wave_h * 3.0;
    let y_f = y as f32;
    
    // Foam appears at wave peaks
    let at_crest = (y_f - wave_top).abs() < 1.5;
    
    // Add some randomness to foam
    let foam_noise = simple_hash(x as usize + frame_index / 3, y as usize) % 10;
    at_crest && foam_noise < 6
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Deep ocean background (gradient would be nice but we'll simulate)
    let bg = Block::default().style(Style::default().bg(Color::Rgb(0, 20, 40)));
    frame.render_widget(bg, area);
    
    let water_start = area.height / 4; // Horizon line
    
    // Sky gradient (simple version)
    for y in 0..water_start {
        let sky_intensity = y as f32 / water_start as f32;
        let r = (20.0 + sky_intensity * 30.0) as u8;
        let g = (30.0 + sky_intensity * 50.0) as u8;
        let b = (60.0 + sky_intensity * 80.0) as u8;
        
        for x in 0..area.width {
            frame.render_widget(
                Paragraph::new(" ").style(Style::default().bg(Color::Rgb(r, g, b))),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
    
    // Sun/moon reflection
    let sun_x = area.width / 2;
    let t = frame_index as f32 * 0.02;
    
    // Draw multiple wave layers
    for layer in 0..5 {
        let layer_y = water_start + layer * (area.height - water_start) / 5;
        let depth_factor = layer as f32 / 5.0;
        
        for x in 0..area.width {
            let wave_h = wave_height(x, area.width, frame_index, layer as usize);
            let wave_offset = (wave_h * (3.0 - depth_factor * 2.0)) as i16;
            
            for dy in 0..((area.height - layer_y) / 5).max(1) {
                let y = layer_y + dy;
                let actual_y = (y as i16 + wave_offset).max(water_start as i16) as u16;
                
                if actual_y >= area.height {
                    continue;
                }
                
                // Color based on depth
                let depth_darkness = depth_factor * 0.5;
                let r = (10.0 * (1.0 - depth_darkness)) as u8;
                let g = (60.0 + 40.0 * (1.0 - depth_darkness)) as u8;
                let b = (120.0 + 60.0 * (1.0 - depth_darkness)) as u8;
                
                // Wave character based on position
                let ch = if is_foam(x, actual_y, area.width, area.height, frame_index, layer_y) {
                    '~'
                } else if dy == 0 {
                    '≈'
                } else {
                    '~'
                };
                
                let color = if is_foam(x, actual_y, area.width, area.height, frame_index, layer_y) {
                    Color::Rgb(200, 230, 255) // White foam
                } else {
                    Color::Rgb(r, g, b)
                };
                
                frame.render_widget(
                    Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                    Rect::new(area.x + x, area.y + actual_y, 1, 1),
                );
            }
        }
    }
    
    // Sun reflection on water
    let reflection_width = 3;
    for dy in 0..((area.height - water_start) / 2) {
        let y = water_start + dy;
        let wobble = (fast_sin(dy as f32 * 0.5 + t * 2.0) * 2.0) as i16;
        
        for dx in 0..reflection_width {
            let x = (sun_x as i16 + dx as i16 - reflection_width as i16 / 2 + wobble) as u16;
            if x < area.width && y < area.height {
                let intensity = (1.0 - dy as f32 / (area.height - water_start) as f32) * 255.0;
                let i = intensity as u8;
                frame.render_widget(
                    Paragraph::new("∗").style(Style::default().fg(Color::Rgb(i, i, i / 2))),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }
}
