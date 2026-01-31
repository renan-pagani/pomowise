use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// DNA Double Helix - rotating 3D helix structure

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

fn fast_cos(x: f32) -> f32 {
    fast_sin(x + std::f32::consts::PI / 2.0)
}

/// DNA base pair colors
fn base_color(base_type: usize) -> Color {
    match base_type % 4 {
        0 => Color::Rgb(255, 100, 100), // Adenine - Red
        1 => Color::Rgb(100, 255, 100), // Guanine - Green
        2 => Color::Rgb(100, 100, 255), // Cytosine - Blue
        _ => Color::Rgb(255, 255, 100), // Thymine - Yellow
    }
}

/// Complementary base (A-T, G-C)
fn complement_color(base_type: usize) -> Color {
    match base_type % 4 {
        0 => Color::Rgb(255, 255, 100), // Thymine pairs with Adenine
        1 => Color::Rgb(100, 100, 255), // Cytosine pairs with Guanine
        2 => Color::Rgb(100, 255, 100), // Guanine pairs with Cytosine
        _ => Color::Rgb(255, 100, 100), // Adenine pairs with Thymine
    }
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Dark background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(5, 10, 20)));
    frame.render_widget(bg, area);
    
    let center_x = area.width as f32 / 2.0;
    let helix_radius = (area.width as f32 / 4.0).min(15.0);
    let t = frame_index as f32 * 0.05; // Rotation speed
    
    // Draw the double helix
    for y in 0..area.height {
        let fy = y as f32;
        let phase = fy * 0.3 + t; // Vertical twist + rotation
        
        // Two strands, 180 degrees apart
        let strand1_x = center_x + fast_sin(phase) * helix_radius;
        let strand2_x = center_x + fast_sin(phase + std::f32::consts::PI) * helix_radius;
        
        // Z-depth for 3D effect (determines which strand is in front)
        let strand1_z = fast_cos(phase);
        let strand2_z = fast_cos(phase + std::f32::consts::PI);
        
        // Brightness based on z-depth
        let strand1_bright = ((strand1_z + 1.0) / 2.0 * 155.0 + 100.0) as u8;
        let strand2_bright = ((strand2_z + 1.0) / 2.0 * 155.0 + 100.0) as u8;
        
        // Base pair index for coloring
        let base_idx = (y as usize + frame_index / 10) % 20;
        
        // Draw backbone (phosphate-sugar)
        let backbone_char = if strand1_z > strand2_z { '●' } else { '○' };
        let x1 = strand1_x as u16;
        let x2 = strand2_x as u16;
        
        // Draw strand 1 backbone
        if x1 < area.width {
            let color = Color::Rgb(strand1_bright, strand1_bright / 2, strand1_bright);
            frame.render_widget(
                Paragraph::new(backbone_char.to_string()).style(Style::default().fg(color)),
                Rect::new(area.x + x1, area.y + y, 1, 1),
            );
        }
        
        // Draw strand 2 backbone
        if x2 < area.width {
            let backbone_char2 = if strand2_z > strand1_z { '●' } else { '○' };
            let color = Color::Rgb(strand2_bright / 2, strand2_bright, strand2_bright);
            frame.render_widget(
                Paragraph::new(backbone_char2.to_string()).style(Style::default().fg(color)),
                Rect::new(area.x + x2, area.y + y, 1, 1),
            );
        }
        
        // Draw base pairs (rungs) - only at certain intervals
        if y % 2 == 0 {
            let (left_x, right_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            
            // Only draw rungs when strands are at similar z-depth (side view)
            let z_diff = (strand1_z - strand2_z).abs();
            if z_diff < 0.8 {
                // Draw the connecting base pair
                let base1_color = base_color(base_idx);
                let base2_color = complement_color(base_idx);
                
                let mid_x = (left_x + right_x) / 2;
                
                // Left half of rung
                for bx in (left_x + 1)..mid_x {
                    if bx < area.width {
                        let rung_char = '─';
                        frame.render_widget(
                            Paragraph::new(rung_char.to_string()).style(Style::default().fg(base1_color)),
                            Rect::new(area.x + bx, area.y + y, 1, 1),
                        );
                    }
                }
                
                // Right half of rung
                for bx in mid_x..right_x {
                    if bx < area.width {
                        let rung_char = '─';
                        frame.render_widget(
                            Paragraph::new(rung_char.to_string()).style(Style::default().fg(base2_color)),
                            Rect::new(area.x + bx, area.y + y, 1, 1),
                        );
                    }
                }
                
                // Base letters at connection point
                if mid_x > 0 && mid_x < area.width {
                    let bases = ['A', 'G', 'C', 'T'];
                    let base_char = bases[base_idx % 4];
                    frame.render_widget(
                        Paragraph::new(base_char.to_string())
                            .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                        Rect::new(area.x + mid_x, area.y + y, 1, 1),
                    );
                }
            }
        }
    }
    
    // Add floating particles for effect
    for i in 0..15 {
        let px = ((i * 7919 + frame_index) % area.width as usize) as u16;
        let py = ((i * 3571 + frame_index / 2) % area.height as usize) as u16;
        let particle_chars = ['·', '∘', '°'];
        let ch = particle_chars[i % 3];
        
        if px < area.width && py < area.height {
            frame.render_widget(
                Paragraph::new(ch.to_string()).style(Style::default().fg(Color::Rgb(60, 80, 100))),
                Rect::new(area.x + px, area.y + py, 1, 1),
            );
        }
    }
}
