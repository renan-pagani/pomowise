use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Glitch - Corrupted scanlines, RGB split effects, digital noise, cyberpunk aesthetic

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

/// Generate glitch offset for a scanline
fn scanline_glitch(y: u16, frame_index: usize) -> i16 {
    let t = frame_index;

    // Occasional heavy glitch
    let heavy_glitch = simple_hash(t / 15, 100) % 20 == 0;
    let glitch_line = (simple_hash(t / 8, 101) % 30) as u16;

    if heavy_glitch && y.abs_diff(glitch_line) < 3 {
        // Major displacement
        ((simple_hash(y as usize + t, 102) % 20) as i16) - 10
    } else if simple_hash(y as usize + t / 5, 103) % 40 == 0 {
        // Minor displacement
        ((simple_hash(y as usize + t, 104) % 6) as i16) - 3
    } else {
        0
    }
}

/// Check if we should have RGB split at this position
fn rgb_split_intensity(x: u16, y: u16, frame_index: usize) -> (bool, i16) {
    let t = frame_index;
    let split_active = simple_hash(t / 12, 200) % 3 == 0;

    if split_active {
        let intensity = (fast_sin(y as f32 * 0.3 + t as f32 * 0.1) * 2.0) as i16;
        (true, intensity.clamp(-3, 3))
    } else {
        (false, 0)
    }
}

/// Generate digital noise character
fn noise_char(x: u16, y: u16, frame_index: usize) -> Option<(char, Color)> {
    let t = frame_index;
    let noise_seed = simple_hash(x as usize + y as usize * 1000 + t * 7, 300);

    // Static noise probability
    let noise_prob = if simple_hash(t / 20, 301) % 4 == 0 { 15 } else { 50 };

    if noise_seed % noise_prob == 0 {
        let char_set = ['░', '▒', '▓', '█', '▀', '▄', '▌', '▐', '·', ':', ';', '!'];
        let ch = char_set[noise_seed % char_set.len()];
        let brightness = (noise_seed % 150 + 50) as u8;
        Some((ch, Color::Rgb(brightness, brightness, brightness)))
    } else {
        None
    }
}

/// Generate scanline effect
fn scanline_effect(y: u16, frame_index: usize) -> u8 {
    // CRT scanline darkness
    let scanline = if y % 2 == 0 { 0 } else { 15 };

    // Occasional bright scanline flash
    let flash_line = (frame_index / 3) % 40;
    let flash = if y as usize == flash_line { 30 } else { 0 };

    scanline + flash
}

/// Generate corruption block
fn corruption_block(x: u16, y: u16, width: u16, height: u16, frame_index: usize) -> Option<(char, Color)> {
    let t = frame_index;

    // Check if we're in a corruption zone
    let block_seed = simple_hash(t / 25, 400);
    let block_active = block_seed % 5 == 0;

    if !block_active {
        return None;
    }

    let block_x = (block_seed % width as usize) as u16;
    let block_y = (simple_hash(t / 25, 401) % height as usize) as u16;
    let block_w = (block_seed % 15 + 5) as u16;
    let block_h = (block_seed % 4 + 2) as u16;

    if x >= block_x && x < block_x + block_w && y >= block_y && y < block_y + block_h {
        let glitch_chars = ['▓', '█', '▒', '░', '▀', '▄'];
        let ch = glitch_chars[simple_hash(x as usize + y as usize, 402) % glitch_chars.len()];

        // Glitchy colors
        let colors = [
            Color::Rgb(255, 0, 100),   // Hot pink
            Color::Rgb(0, 255, 200),   // Cyan
            Color::Rgb(255, 255, 0),   // Yellow
            Color::Rgb(100, 0, 255),   // Purple
        ];
        let color = colors[simple_hash(x as usize, 403) % colors.len()];

        Some((ch, color))
    } else {
        None
    }
}

/// Draw cyberpunk grid lines
fn grid_line(x: u16, y: u16, width: u16, height: u16, frame_index: usize) -> Option<Color> {
    let t = frame_index as f32 * 0.05;

    // Horizontal grid lines
    let h_spacing = 8;
    let v_spacing = 12;

    let on_h_line = y % h_spacing == 0;
    let on_v_line = x % v_spacing == 0;

    if on_h_line || on_v_line {
        let pulse = (fast_sin(t + x as f32 * 0.1 + y as f32 * 0.1) * 0.5 + 0.5) as u8;
        let base = 30 + pulse * 20;

        if on_h_line && on_v_line {
            // Intersection - brighter
            Some(Color::Rgb(base + 40, base / 2, base + 60))
        } else {
            Some(Color::Rgb(base, base / 3, base + 20))
        }
    } else {
        None
    }
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Dark cyberpunk background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(5, 5, 12)));
    frame.render_widget(bg, area);

    // Calculate RGB split for this frame
    let (rgb_active, rgb_offset) = rgb_split_intensity(0, 0, frame_index);

    for y in 0..area.height {
        let scanline_offset = scanline_glitch(y, frame_index);
        let scanline_dark = scanline_effect(y, frame_index);

        for x in 0..area.width {
            // Apply scanline offset
            let effective_x = (x as i16 + scanline_offset).clamp(0, area.width as i16 - 1) as u16;

            // Check for corruption blocks first
            if let Some((ch, color)) = corruption_block(effective_x, y, area.width, area.height, frame_index) {
                frame.render_widget(
                    Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
                continue;
            }

            // Grid lines
            if let Some(grid_color) = grid_line(effective_x, y, area.width, area.height, frame_index) {
                frame.render_widget(
                    Paragraph::new("·").style(Style::default().fg(grid_color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }

            // Digital noise
            if let Some((ch, color)) = noise_char(effective_x, y, frame_index) {
                frame.render_widget(
                    Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }

            // RGB split effect - draw offset colored artifacts
            if rgb_active && simple_hash(x as usize + y as usize * 100 + frame_index, 500) % 30 == 0 {
                let r_offset = rgb_offset;
                let b_offset = -rgb_offset;

                // Red channel offset
                let rx = (x as i16 + r_offset).clamp(0, area.width as i16 - 1) as u16;
                if rx < area.width {
                    frame.render_widget(
                        Paragraph::new("▒").style(Style::default().fg(Color::Rgb(200, 0, 0))),
                        Rect::new(area.x + rx, area.y + y, 1, 1),
                    );
                }

                // Blue channel offset
                let bx = (x as i16 + b_offset).clamp(0, area.width as i16 - 1) as u16;
                if bx < area.width {
                    frame.render_widget(
                        Paragraph::new("▒").style(Style::default().fg(Color::Rgb(0, 0, 200))),
                        Rect::new(area.x + bx, area.y + y, 1, 1),
                    );
                }
            }
        }
    }

    // Add glitch text fragments
    let glitch_texts = ["ERR0R", "SYS_", "0x", "NULL", "VOID", ">>", "<<"];
    let text_count = 3;

    for i in 0..text_count {
        let h1 = simple_hash(frame_index / 20 + i, 600);
        let h2 = simple_hash(frame_index / 20 + i, 601);

        if h1 % 4 == 0 {
            let text_x = (h1 % area.width as usize) as u16;
            let text_y = (h2 % area.height as usize) as u16;
            let text = glitch_texts[h1 % glitch_texts.len()];

            if text_x + text.len() as u16 <= area.width && text_y < area.height {
                let glitch_color = Color::Rgb(255, 0, 100);
                frame.render_widget(
                    Paragraph::new(text).style(Style::default().fg(glitch_color)),
                    Rect::new(area.x + text_x, area.y + text_y, text.len() as u16, 1),
                );
            }
        }
    }
}
