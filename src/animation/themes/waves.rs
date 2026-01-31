use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Radio wave expanding circles from center
pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Dark purple background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(10, 0, 20)));
    frame.render_widget(bg, area);

    let center_x = area.width / 2;
    let center_y = area.height / 2;

    // Maximum radius
    let max_radius = ((area.width.max(area.height) as f32) * 0.7) as u16;

    // Draw multiple expanding waves
    let num_waves = 5;
    let wave_spacing = 8;

    for y in 0..area.height {
        for x in 0..area.width {
            // Calculate distance from center
            let dx = x as f32 - center_x as f32;
            let dy = (y as f32 - center_y as f32) * 2.0; // Stretch vertically for aspect ratio
            let dist = (dx * dx + dy * dy).sqrt();

            // Check if on any wave ring
            let mut on_wave = false;
            let mut wave_intensity = 0.0f32;

            for wave_idx in 0..num_waves {
                // Each wave has a different radius based on frame
                let wave_offset = (frame_index + wave_idx * wave_spacing) % (max_radius as usize * 2);
                let wave_radius = wave_offset as f32;

                // Distance from wave ring
                let ring_dist = (dist - wave_radius).abs();

                if ring_dist < 2.0 {
                    on_wave = true;
                    // Intensity based on how close to the exact ring
                    let ring_intensity = 1.0 - ring_dist / 2.0;
                    // Fade out as wave expands
                    let fade = 1.0 - (wave_radius / (max_radius as f32 * 2.0));
                    wave_intensity = wave_intensity.max(ring_intensity * fade);
                }
            }

            if on_wave && wave_intensity > 0.1 {
                let (color, ch) = wave_color_char(wave_intensity, dist, frame_index);
                frame.render_widget(
                    Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }

    // Draw center emitter
    let emitter_chars = ['◉', '●', '◎', '○'];
    let emitter_idx = (frame_index / 3) % emitter_chars.len();
    let emitter_color = Color::Rgb(0, 255, 255);

    if center_x < area.width && center_y < area.height {
        frame.render_widget(
            Paragraph::new(emitter_chars[emitter_idx].to_string())
                .style(Style::default().fg(emitter_color)),
            Rect::new(area.x + center_x, area.y + center_y, 1, 1),
        );
    }
}

fn wave_color_char(intensity: f32, dist: f32, frame_index: usize) -> (Color, char) {
    // Cycle colors based on distance and time
    let hue_shift = (dist / 20.0 + frame_index as f32 * 0.05) % 3.0;

    let color = if hue_shift < 1.0 {
        // Cyan
        let i = (intensity * 255.0) as u8;
        Color::Rgb(0, i, i)
    } else if hue_shift < 2.0 {
        // Magenta
        let i = (intensity * 255.0) as u8;
        Color::Rgb(i, 0, i)
    } else {
        // Purple
        let i = (intensity * 200.0) as u8;
        Color::Rgb(i / 2, 0, i)
    };

    let ch = if intensity > 0.7 {
        '█'
    } else if intensity > 0.5 {
        '▓'
    } else if intensity > 0.3 {
        '▒'
    } else {
        '░'
    };

    (color, ch)
}
