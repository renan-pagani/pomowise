use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

/// Synthwave - Minimal sun over mountains with breathing darkness

// ============================================================================
// COLOR PALETTE
// ============================================================================

const GOLD: (u8, u8, u8) = (255, 214, 1);       // Sun

// ============================================================================
// UTILITY
// ============================================================================

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

// ============================================================================
// MAIN RENDER
// ============================================================================

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    let horizon_y = (area.height as f32 * 0.55) as u16;
    let t = frame_index as f32 * 0.02;

    // Sky - simple dark gradient
    for y in 0..horizon_y {
        let fy = y as f32 / horizon_y as f32;
        let dark = (8.0 + fy * 15.0) as u8;
        let color = Color::Rgb(dark, dark / 2, dark + 5);

        for x in 0..area.width {
            frame.render_widget(
                Paragraph::new(" ").style(Style::default().bg(color)),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }

    // Sun
    render_sun(frame, area, horizon_y, t);

    // Mountain silhouette
    render_mountains(frame, area, horizon_y);

    // Breathing darkness below
    render_breathing_floor(frame, area, horizon_y, t);
}

// ============================================================================
// SUN - Special character silhouette with breathing effect
// ============================================================================

fn render_sun(frame: &mut Frame, area: Rect, horizon_y: u16, t: f32) {
    let cx = area.width / 2;
    let radius = (area.width.min(area.height * 2) / 6).max(4) as f32;

    // Sun-themed special characters
    let sun_chars = ['*', '✦', '✧', '·', '°', '∘', '+', '×', '•', '◦', '⋆', '∙'];

    for y in 0..horizon_y {
        let dy = (horizon_y as f32 - y as f32) / 2.0;
        if dy > radius { continue; }

        for x in 0..area.width {
            let dx = (x as f32 - cx as f32) / 2.0;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < radius {
                // Stripe pattern for classic synthwave sun look
                let stripe_pos = dy / radius;
                let stripe_phase = (stripe_pos * 7.0 + t * 0.2) as i32;
                let in_gap = stripe_pos > 0.3 && stripe_phase % 2 == 1;

                if !in_gap {
                    // Breathing wave within the sun
                    let wave1 = fast_sin(x as f32 * 0.15 + y as f32 * 0.1 + t * 2.0);
                    let wave2 = fast_sin(x as f32 * 0.2 - t * 1.5);
                    let breath = (wave1 * 0.5 + wave2 * 0.5 + 1.0) / 2.0;

                    // Color gradient from center to edge
                    let edge_factor = dist / radius;
                    let grad = 1.0 - stripe_pos * 0.3;

                    // Warm gold palette with breathing intensity
                    let intensity = 0.6 + breath * 0.4;
                    let r = (GOLD.0 as f32 * grad * intensity) as u8;
                    let g = (GOLD.1 as f32 * grad * 0.75 * intensity) as u8;
                    let b = (60.0 * grad * intensity) as u8;

                    // Character selection based on position and breathing
                    let char_seed = simple_hash(x as usize + y as usize * 50, (t * 3.0) as usize);
                    let char_idx = char_seed % sun_chars.len();

                    // Denser characters near center, sparser at edges
                    let density_threshold = 0.2 + edge_factor * 0.3;
                    let show_char = breath > density_threshold;

                    if show_char {
                        frame.render_widget(
                            Paragraph::new(sun_chars[char_idx].to_string())
                                .style(Style::default().fg(Color::Rgb(r, g, b))),
                            Rect::new(area.x + x, area.y + y, 1, 1),
                        );
                    }
                }
            }
        }
    }
}

// ============================================================================
// MOUNTAINS - Simple silhouette
// ============================================================================

fn render_mountains(frame: &mut Frame, area: Rect, horizon_y: u16) {
    let mountain_color = Color::Rgb(8, 6, 12);

    for x in 0..area.width {
        let fx = x as f32 / area.width as f32;

        // Multiple overlapping peaks
        let peak1 = fast_sin(fx * 2.5 + 0.5) * 0.15;
        let peak2 = fast_sin(fx * 4.0 + 1.8) * 0.08;
        let peak3 = fast_sin(fx * 7.0 + 0.3) * 0.04;

        let mountain_height = (peak1 + peak2 + peak3).max(0.0);
        let mountain_top = horizon_y.saturating_sub((mountain_height * area.height as f32 * 0.3) as u16);

        for y in mountain_top..horizon_y {
            frame.render_widget(
                Paragraph::new("█").style(Style::default().fg(mountain_color)),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
}

// ============================================================================
// BREATHING FLOOR - Flowing special characters in dark tones
// ============================================================================

fn render_breathing_floor(frame: &mut Frame, area: Rect, horizon_y: u16, t: f32) {
    let chars = ['*', '>', '<', '&', '%', '@', '#', '~', '^', '+', '·', '∘', '°', '×'];

    for y in horizon_y..area.height {
        let depth = (y - horizon_y) as f32 / (area.height - horizon_y) as f32;

        for x in 0..area.width {
            // Breathing wave - multiple overlapping waves for organic feel
            let wave1 = fast_sin(x as f32 * 0.08 + y as f32 * 0.05 + t * 1.5);
            let wave2 = fast_sin(x as f32 * 0.12 + y as f32 * 0.08 - t * 0.8);
            let wave3 = fast_sin(x as f32 * 0.04 + t * 2.0);
            let breath = (wave1 * 0.4 + wave2 * 0.35 + wave3 * 0.25 + 1.0) / 2.0;

            // Color: black > gray > slate gradient based on breathing
            let base = 12.0 + depth * 8.0;
            let intensity = base + breath * 35.0;

            let r = intensity as u8;
            let g = (intensity * 0.9) as u8;
            let b = (intensity * 1.1).min(255.0) as u8; // Slight slate/blue tint

            // Character selection - changes with position and time
            let char_seed = simple_hash(x as usize + y as usize * 100, (t * 2.0) as usize);
            let char_idx = char_seed % chars.len();

            // Some cells are empty for breathing space
            let show_char = (breath > 0.3) && (simple_hash(x as usize, y as usize + frame_idx_slow(t)) % 3 != 0);

            if show_char {
                frame.render_widget(
                    Paragraph::new(chars[char_idx].to_string())
                        .style(Style::default().fg(Color::Rgb(r, g, b))),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            } else {
                // Dark background
                let bg = (base * 0.5) as u8;
                frame.render_widget(
                    Paragraph::new(" ").style(Style::default().bg(Color::Rgb(bg, bg / 2, bg))),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }
}

fn frame_idx_slow(t: f32) -> usize {
    (t * 0.5) as usize
}
