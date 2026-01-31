use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Minimal - Subtle gradient pulse, zen-like dots, breathing animation, calm and sparse

fn simple_hash(x: usize, seed: usize) -> usize {
    let mut h = x.wrapping_mul(2654435761);
    h ^= seed;
    h = h.wrapping_mul(2654435761);
    h ^ (h >> 16)
}

/// Fast sine approximation for smooth animations
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

/// Calculate breathing pulse intensity - slow, calming rhythm
fn breathing_pulse(frame_index: usize) -> f32 {
    // Very slow breathing cycle (~4 seconds per breath)
    let t = frame_index as f32 * 0.015;
    (fast_sin(t) * 0.5 + 0.5).powf(0.7) // Smooth ease
}

/// Calculate gradient intensity based on position
fn gradient_intensity(x: u16, y: u16, width: u16, height: u16, frame_index: usize) -> f32 {
    let t = frame_index as f32 * 0.008;

    // Center point with slow drift
    let cx = width as f32 / 2.0 + fast_sin(t) * (width as f32 * 0.1);
    let cy = height as f32 / 2.0 + fast_sin(t * 0.7) * (height as f32 * 0.1);

    // Distance from center, normalized
    let dx = (x as f32 - cx) / width as f32;
    let dy = (y as f32 - cy) / height as f32 * 2.0; // Adjust for aspect ratio
    let dist = (dx * dx + dy * dy).sqrt();

    // Soft radial gradient
    let gradient = (1.0 - dist * 1.2).max(0.0).powf(2.0);

    // Combine with breathing
    gradient * breathing_pulse(frame_index)
}

/// Determine if a zen dot should appear at this position
fn zen_dot(x: u16, y: u16, width: u16, height: u16, frame_index: usize) -> Option<(char, f32)> {
    // Sparse dot grid with large spacing
    let grid_x = 8;
    let grid_y = 4;

    // Only on grid intersections
    if x % grid_x != 0 || y % grid_y != 0 {
        return None;
    }

    let t = frame_index as f32 * 0.02;

    // Each dot has its own phase
    let dot_id = (x / grid_x) as usize + (y / grid_y) as usize * 100;
    let phase = simple_hash(dot_id, 1) as f32 / 1000.0;

    // Slow ripple from center
    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;
    let dist = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();

    let ripple = fast_sin(dist * 0.1 - t + phase);
    let intensity = (ripple * 0.5 + 0.5).powf(0.5);

    // Choose dot character based on intensity
    let ch = if intensity > 0.7 {
        '•'
    } else if intensity > 0.3 {
        '·'
    } else {
        ' '
    };

    if ch == ' ' {
        None
    } else {
        Some((ch, intensity))
    }
}

/// Calculate subtle wave pattern
fn subtle_wave(x: u16, y: u16, width: u16, frame_index: usize) -> f32 {
    let t = frame_index as f32 * 0.01;
    let fx = x as f32 / width as f32;
    let fy = y as f32;

    // Very gentle overlapping waves
    let wave1 = fast_sin(fx * 3.0 + t) * 0.3;
    let wave2 = fast_sin(fx * 5.0 - t * 0.5 + fy * 0.1) * 0.2;

    (wave1 + wave2 + 0.5).clamp(0.0, 1.0)
}

/// Get minimal color palette - muted, calm tones
fn minimal_color(intensity: f32, variant: usize) -> Color {
    let base = (intensity * 40.0) as u8 + 15;

    match variant % 3 {
        0 => Color::Rgb(base, base + 5, base + 10),      // Cool grey-blue
        1 => Color::Rgb(base + 5, base + 8, base + 5),   // Sage green tint
        _ => Color::Rgb(base + 8, base + 5, base + 3),   // Warm grey
    }
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Calm, dark background with subtle warmth
    let bg = Block::default().style(Style::default().bg(Color::Rgb(12, 12, 15)));
    frame.render_widget(bg, area);

    // Layer 1: Subtle gradient pulse from center
    for y in 0..area.height {
        for x in 0..area.width {
            let gradient = gradient_intensity(x, y, area.width, area.height, frame_index);

            if gradient > 0.05 {
                let wave = subtle_wave(x, y, area.width, frame_index);
                let combined = gradient * wave;

                if combined > 0.1 {
                    let color = Color::Rgb(
                        (12.0 + combined * 20.0) as u8,
                        (12.0 + combined * 22.0) as u8,
                        (15.0 + combined * 25.0) as u8,
                    );

                    // Very subtle texture
                    let ch = if combined > 0.4 { '░' } else { ' ' };

                    if ch != ' ' {
                        frame.render_widget(
                            Paragraph::new(ch.to_string())
                                .style(Style::default().fg(color)),
                            Rect::new(area.x + x, area.y + y, 1, 1),
                        );
                    }
                }
            }
        }
    }

    // Layer 2: Zen dots with ripple animation
    for y in 0..area.height {
        for x in 0..area.width {
            if let Some((ch, intensity)) = zen_dot(x, y, area.width, area.height, frame_index) {
                let variant = simple_hash(x as usize + y as usize * 1000, 10);
                let color = minimal_color(intensity, variant);

                frame.render_widget(
                    Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }

    // Layer 3: Breathing center indicator (very subtle)
    let pulse = breathing_pulse(frame_index);
    let cx = area.width / 2;
    let cy = area.height / 2;

    if pulse > 0.3 {
        let brightness = (pulse * 60.0) as u8 + 30;
        let center_color = Color::Rgb(brightness, brightness + 5, brightness + 10);

        // Small breathing dot at center
        frame.render_widget(
            Paragraph::new("·").style(Style::default().fg(center_color)),
            Rect::new(area.x + cx, area.y + cy, 1, 1),
        );
    }

    // Layer 4: Occasional drifting particles (very sparse)
    let particle_count = 5;
    for i in 0..particle_count {
        let seed = simple_hash(i + frame_index / 100, 500);
        let lifetime = frame_index % 200;

        if seed % 3 == 0 && lifetime < 180 {
            let start_x = simple_hash(i, 501) % area.width as usize;
            let start_y = simple_hash(i, 502) % area.height as usize;

            // Slow drift
            let drift_x = (lifetime as f32 * 0.05) as usize;
            let drift_y = (fast_sin(lifetime as f32 * 0.03) * 2.0) as i16;

            let x = ((start_x + drift_x) % area.width as usize) as u16;
            let y = (start_y as i16 + drift_y).clamp(0, area.height as i16 - 1) as u16;

            // Fade in and out
            let fade = if lifetime < 30 {
                lifetime as f32 / 30.0
            } else if lifetime > 150 {
                (180 - lifetime) as f32 / 30.0
            } else {
                1.0
            };

            let brightness = (fade * 50.0) as u8 + 20;
            let particle_color = Color::Rgb(brightness, brightness, brightness + 5);

            frame.render_widget(
                Paragraph::new("·").style(Style::default().fg(particle_color)),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
}
