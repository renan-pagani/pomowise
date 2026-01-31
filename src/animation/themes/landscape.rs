use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

/// Landscape - A serene Bob Ross-inspired pastoral scene
/// Features: rolling hills with parallax, sun/moon cycle, clouds, river, acacia trees, birds, fireflies

// Acacia tree patterns - more organic, asymmetric silhouettes
// Small acacia - scraggly young tree
const ACACIA_SMALL: &[&str] = &[
    "  ░▒▓█▒░  ",
    "  ▓██▓░   ",
    "    ▓▒    ",
    "    █     ",
    "    █     ",
];

// Medium acacia - classic flat-top shape
const ACACIA_MEDIUM: &[&str] = &[
    " ░▒▓███▓▒░ ",
    "▒▓█████████▓▒",
    "  ▓█████▓  ",
    "    ▓█▓    ",
    "     █     ",
    "     █     ",
    "     █     ",
];

// Large acacia - majestic spreading canopy
const ACACIA_LARGE: &[&str] = &[
    "  ░▒▓████▓▒░░  ",
    "░▓█████████████░",
    "▓███████████████▓",
    "  ▒▓███████▓▒  ",
    "     ▓███▓     ",
    "      ▓█▓      ",
    "       █       ",
    "       █       ",
    "      ▄█▄      ",
];

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

/// Get time of day phase: 0 = midnight, 0.25 = dawn, 0.5 = noon, 0.75 = dusk
fn get_day_phase(frame_index: usize) -> f32 {
    let t = frame_index as f32 * 0.003;
    ((fast_sin(t) + 1.0) / 2.0)
}

/// Check if it's dawn or dusk (transition period)
fn is_transition_period(day_phase: f32) -> bool {
    (day_phase > 0.15 && day_phase < 0.35) || (day_phase > 0.65 && day_phase < 0.85)
}

/// Get hill height at x position (multiple overlapping hills) - now with 6 layers
fn hill_height(x: u16, width: u16, height: u16, layer: usize) -> u16 {
    let fx = x as f32 / width as f32;
    let base_height = height as f32 * 0.35;

    // Different hill patterns for each layer
    let offset = layer as f32 * 0.4;
    let freq = 1.5 + layer as f32 * 0.6;

    let h1 = fast_sin(fx * freq + offset) * 0.35;
    let h2 = fast_sin(fx * freq * 1.8 + offset + 1.2) * 0.2;
    let h3 = fast_sin(fx * freq * 0.4 + offset + 2.5) * 0.15;

    let hill_offset = (h1 + h2 + h3) * base_height;
    // More spread between layers
    let layer_base = height as f32 * (0.45 + layer as f32 * 0.08);

    (layer_base + hill_offset).max(0.0) as u16
}

/// Get river path at x position - dramatic S-curve meander
fn river_y(x: u16, width: u16, height: u16, _frame_index: usize) -> u16 {
    let fx = x as f32 / width as f32;

    // Base river path - in the mid-lower landscape
    let base = height as f32 * 0.68;

    // DRAMATIC S-CURVE - large sweeping meanders (15% of height!)
    let meander1 = fast_sin(fx * 1.8 + 0.3) * height as f32 * 0.15;
    // Secondary curve for natural variation
    let meander2 = fast_sin(fx * 3.5 + 1.5) * height as f32 * 0.06;
    // Small ripples
    let ripple = fast_sin(fx * 8.0 + 2.0) * height as f32 * 0.02;

    (base + meander1 + meander2 + ripple).clamp(height as f32 * 0.5, height as f32 * 0.85) as u16
}

/// Get river width at position - wider at bends/pools, narrower at straights
fn river_width_at(x: u16, width: u16) -> u16 {
    let fx = x as f32 / width as f32;

    // Pools form at the outer bends of meanders
    let bend_factor = fast_sin(fx * 1.8 + 0.3).abs(); // Where river curves most
    let pool_factor = fast_sin(fx * 2.5 + 1.0).max(0.0);

    // Base width 2, bends expand to 4-5
    let base_width = 2;
    let pool_width = (bend_factor * 2.0 + pool_factor * 1.5) as u16;

    base_width + pool_width
}

/// Check if position is a riverbank rock
fn is_riverbank_rock(x: u16, y: u16, width: u16, height: u16, frame_index: usize) -> Option<Color> {
    let river_center = river_y(x, width, height, frame_index);
    let river_w = river_width_at(x, width);

    let river_top = river_center.saturating_sub(river_w / 2);
    let river_bottom = river_center + river_w / 2 + 1;

    // Rocks appear 1-2 cells outside the river
    let rock_zone_top = river_top.saturating_sub(2)..river_top;
    let rock_zone_bottom = (river_bottom + 1)..(river_bottom + 3);

    let in_rock_zone = rock_zone_top.contains(&y) || rock_zone_bottom.contains(&y);

    if in_rock_zone {
        // Sparse rock placement using hash
        let rock_seed = simple_hash(x as usize * 31 + y as usize * 17, 9999);
        if rock_seed % 5 == 0 {
            // Gray rock colors - varied
            let shade = 60 + (rock_seed % 40) as u8;
            return Some(Color::Rgb(shade, shade - 5, shade - 10));
        }
    }
    None
}

/// Check if position is in the river
fn is_river(x: u16, y: u16, width: u16, height: u16, frame_index: usize) -> (bool, bool) {
    let river_center = river_y(x, width, height, frame_index);
    let river_w = river_width_at(x, width);

    // River boundaries
    let river_top = river_center.saturating_sub(river_w / 2);
    let river_bottom = river_center + river_w / 2 + 1;

    let in_river = y >= river_top && y <= river_bottom;

    // Shimmer effect - more active in pools (wider areas)
    let shimmer_rate = if river_w > 3 { 3 } else { 5 };
    let is_shimmer = (x as usize + frame_index / shimmer_rate) % 4 == 0;

    // Edge ripples
    let is_edge = y == river_top || y == river_bottom;

    (in_river, is_shimmer || is_edge)
}

/// Get sky color with dawn/dusk gradients and horizontal color bands
fn sky_color(y: u16, height: u16, day_phase: f32) -> Color {
    let fy = y as f32 / height as f32;

    // Dawn phase (0.15-0.35): pink/orange gradient
    if day_phase > 0.15 && day_phase < 0.35 {
        let dawn_intensity = 1.0 - ((day_phase - 0.25).abs() * 10.0).min(1.0);

        let (r, g, b) = if fy > 0.7 {
            // Horizon: warm orange
            (255, 140, 90)
        } else if fy > 0.4 {
            // Middle: pink
            (255, 180, 200)
        } else {
            // Top: purple-blue
            (150, 140, 200)
        };

        // Blend with base sky
        let base = base_sky_color(fy, day_phase);
        if let Color::Rgb(br, bg, bb) = base {
            return Color::Rgb(
                ((r as f32 * dawn_intensity + br as f32 * (1.0 - dawn_intensity))) as u8,
                ((g as f32 * dawn_intensity + bg as f32 * (1.0 - dawn_intensity))) as u8,
                ((b as f32 * dawn_intensity + bb as f32 * (1.0 - dawn_intensity))) as u8,
            );
        }
        return Color::Rgb(r, g, b);
    }

    // Golden hour (0.35-0.45 or 0.55-0.65): golden/warm gradient
    if (day_phase > 0.35 && day_phase < 0.45) || (day_phase > 0.55 && day_phase < 0.65) {
        let golden_intensity = if day_phase < 0.5 {
            1.0 - ((day_phase - 0.40).abs() * 20.0).min(1.0)
        } else {
            1.0 - ((day_phase - 0.60).abs() * 20.0).min(1.0)
        };

        let (r, g, b) = if fy > 0.7 {
            // Horizon: golden
            (255, 180, 50)
        } else if fy > 0.4 {
            // Middle: warm yellow
            (255, 220, 120)
        } else {
            // Top: light blue
            (180, 200, 240)
        };

        let base = base_sky_color(fy, day_phase);
        if let Color::Rgb(br, bg, bb) = base {
            return Color::Rgb(
                ((r as f32 * golden_intensity + br as f32 * (1.0 - golden_intensity))) as u8,
                ((g as f32 * golden_intensity + bg as f32 * (1.0 - golden_intensity))) as u8,
                ((b as f32 * golden_intensity + bb as f32 * (1.0 - golden_intensity))) as u8,
            );
        }
        return Color::Rgb(r, g, b);
    }

    // Dusk phase (0.65-0.85): red/purple gradient
    if day_phase > 0.65 && day_phase < 0.85 {
        let dusk_intensity = 1.0 - ((day_phase - 0.75).abs() * 10.0).min(1.0);

        let (r, g, b) = if fy > 0.7 {
            // Horizon: deep orange/red
            (255, 80, 30)
        } else if fy > 0.4 {
            // Middle: magenta
            (180, 50, 120)
        } else {
            // Top: deep purple
            (60, 30, 80)
        };

        let base = base_sky_color(fy, day_phase);
        if let Color::Rgb(br, bg, bb) = base {
            return Color::Rgb(
                ((r as f32 * dusk_intensity + br as f32 * (1.0 - dusk_intensity))) as u8,
                ((g as f32 * dusk_intensity + bg as f32 * (1.0 - dusk_intensity))) as u8,
                ((b as f32 * dusk_intensity + bb as f32 * (1.0 - dusk_intensity))) as u8,
            );
        }
        return Color::Rgb(r, g, b);
    }

    base_sky_color(fy, day_phase)
}

/// Base sky color for day/night without special transitions
fn base_sky_color(fy: f32, day_phase: f32) -> Color {
    // Day colors: light blue fading to white near horizon
    let day_r = (100.0 + fy * 100.0) as u8;
    let day_g = (180.0 + fy * 50.0) as u8;
    let day_b = (240.0 + fy * 10.0) as u8;

    // Night colors: deep blue to purple
    let night_r = (5.0 + fy * 15.0) as u8;
    let night_g = (5.0 + fy * 20.0) as u8;
    let night_b = (30.0 + fy * 50.0) as u8;

    // Blend based on day_phase
    let r = (night_r as f32 + (day_r as f32 - night_r as f32) * day_phase) as u8;
    let g = (night_g as f32 + (day_g as f32 - night_g as f32) * day_phase) as u8;
    let b = (night_b as f32 + (day_b as f32 - night_b as f32) * day_phase) as u8;

    Color::Rgb(r, g, b)
}

/// Get river color reflecting sky
fn river_color(x: u16, y: u16, width: u16, height: u16, day_phase: f32, is_shimmer: bool) -> Color {
    let sky = sky_color(y, height, day_phase);
    let river_w = river_width_at(x, width);

    // Deeper blue in pools (wider sections)
    let depth_factor = (river_w as f32 / 4.0).min(1.0);

    if let Color::Rgb(r, g, b) = sky {
        // Base water color - darken and blue-shift
        let water_r = (r as f32 * (0.3 - depth_factor * 0.1)) as u8;
        let water_g = (g as f32 * (0.4 - depth_factor * 0.1)) as u8;
        let water_b = ((b as f32 * 0.6).min(200.0) + 50.0 + depth_factor * 30.0) as u8;

        if is_shimmer {
            // Shimmer/reflection - brighter highlights
            Color::Rgb(
                (water_r as u16 + 50).min(255) as u8,
                (water_g as u16 + 60).min(255) as u8,
                (water_b as u16 + 40).min(255) as u8,
            )
        } else {
            Color::Rgb(water_r, water_g, water_b)
        }
    } else {
        // Fallback blue
        let base_b = 120 + (depth_factor * 40.0) as u8;
        Color::Rgb(30, 60, base_b)
    }
}

/// Get character for river based on width and position
fn river_char(x: u16, y: u16, width: u16, height: u16, frame_index: usize) -> char {
    let river_center = river_y(x, width, height, frame_index);
    let river_w = river_width_at(x, width);

    let is_edge = y == river_center.saturating_sub(river_w / 2) ||
                  y == river_center + river_w / 2 + 1;

    // Animation phase
    let phase = (x as usize + frame_index / 4) % 3;

    if river_w > 2 {
        // Pool - calmer water
        if is_edge {
            match phase { 0 => '~', 1 => '∼', _ => '~' }
        } else {
            match phase { 0 => '≈', 1 => '~', _ => '∽' }
        }
    } else {
        // Stream - flowing water
        match phase { 0 => '~', 1 => '≈', _ => '∿' }
    }
}

/// Get hill color with atmospheric perspective (distant hills are hazier/bluer)
fn hill_color(layer: usize, day_phase: f32) -> Color {
    // Base colors - earthy African savanna tones
    // Closer layers are warmer/more saturated, distant are cooler/hazier
    let base = match layer {
        0 => (55, 75, 35),     // Closest - rich olive green
        1 => (65, 85, 45),     // Dark sage
        2 => (80, 100, 55),    // Savanna green
        3 => (95, 110, 70),    // Dusty green
        4 => (110, 120, 85),   // Sage with haze
        _ => (125, 130, 100),  // Furthest - hazy blue-green
    };

    // Atmospheric haze - distant hills get bluer/cooler
    let haze = (layer as f32 * 0.15).min(0.6);
    let haze_color = if day_phase > 0.4 && day_phase < 0.6 {
        // Day - blue-gray haze
        (120, 135, 160)
    } else if is_transition_period(day_phase) {
        // Sunset/sunrise - warm amber haze
        if day_phase < 0.5 {
            (160, 120, 100) // Dawn
        } else {
            (140, 100, 110) // Dusk
        }
    } else {
        // Night - deep blue haze
        (50, 60, 90)
    };

    let r = (base.0 as f32 * (1.0 - haze) + haze_color.0 as f32 * haze) as u8;
    let g = (base.1 as f32 * (1.0 - haze) + haze_color.1 as f32 * haze) as u8;
    let b = (base.2 as f32 * (1.0 - haze) + haze_color.2 as f32 * haze) as u8;

    // Time of day adjustment
    let night_factor = 0.2 + day_phase * 0.8;
    let r = (r as f32 * night_factor) as u8;
    let g = (g as f32 * night_factor) as u8;
    let b = (b as f32 * (0.35 + day_phase * 0.65)) as u8;

    Color::Rgb(r, g, b)
}

/// Cloud data structure
struct Cloud {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    density: f32,
}

/// Get clouds with depth layering
fn get_clouds(width: u16, height: u16, frame_index: usize) -> Vec<Cloud> {
    let t = frame_index as f32 * 0.015;
    let mut clouds = Vec::new();

    for i in 0..7 {
        let base_x = (simple_hash(i, 100) % (width as usize * 2)) as f32;
        let speed = 1.0 + (simple_hash(i, 150) % 20) as f32 * 0.1;

        clouds.push(Cloud {
            x: (base_x + t * speed * 8.0) % (width as f32 * 1.5) - width as f32 * 0.25,
            y: 2.0 + (simple_hash(i, 200) % (height as usize / 5)) as f32,
            width: 10.0 + (simple_hash(i, 300) % 8) as f32,
            height: 2.0 + (simple_hash(i, 400) % 2) as f32,
            density: 0.6 + (simple_hash(i, 500) % 40) as f32 * 0.01,
        });
    }
    clouds
}

/// Check if position is part of a cloud, returns (char, brightness_factor) for wispy effect
fn cloud_at(x: u16, y: u16, clouds: &[Cloud], day_phase: f32) -> Option<(char, Color)> {
    for cloud in clouds {
        let dx = x as f32 - cloud.x;
        let dy = y as f32 - cloud.y;

        if dx.abs() < cloud.width && dy.abs() < cloud.height {
            let dist = ((dx / cloud.width).powi(2) + (dy / cloud.height).powi(2)).sqrt();

            let (ch, brightness_factor) = if dist < 0.15 {
                ('\u{2588}', 1.0)      // Full block - cloud core
            } else if dist < 0.3 {
                ('\u{2593}', 0.95)     // Dark shade
            } else if dist < 0.5 {
                ('\u{2592}', 0.85)     // Medium shade
            } else if dist < 0.7 {
                ('\u{2591}', 0.75)     // Light shade
            } else if dist < 0.85 {
                (':', 0.6)             // Sparse
            } else if dist < 0.95 {
                ('\u{00B7}', 0.5)      // Very sparse (middle dot)
            } else if dist < 1.0 {
                ('.', 0.4)             // Wispy edge
            } else {
                continue;
            };

            // Calculate base cloud color
            let base_brightness = (180.0 + day_phase * 70.0) as u8;
            let b = (base_brightness as f32 * brightness_factor) as u8;

            // Sunset tinting for cloud undersides during transition periods
            if dy > 0.0 && is_transition_period(day_phase) {
                // Add warm tint to bottom of clouds during sunrise/sunset
                let tint_intensity = (dy / cloud.height).min(1.0) * 0.6;
                let r = (b as f32 + (255.0 - b as f32) * tint_intensity) as u8;
                let g = (b as f32 * (1.0 - tint_intensity * 0.3)) as u8;
                let blue = (b as f32 * (1.0 - tint_intensity * 0.5)) as u8;
                return Some((ch, Color::Rgb(r, g, blue)));
            }

            return Some((ch, Color::Rgb(b, b, b)));
        }
    }
    None
}

/// Get sun/moon position and properties
fn celestial_body(width: u16, height: u16, frame_index: usize) -> (i16, i16, bool, f32) {
    let t = frame_index as f32 * 0.003;
    let phase = fast_sin(t);
    let day_phase = (phase + 1.0) / 2.0;

    // Arc across sky - wider arc
    let x = (width as f32 * 0.5 + fast_sin(t + std::f32::consts::PI / 2.0) * width as f32 * 0.4) as i16;
    let y = (height as f32 * 0.15 - phase.abs() * height as f32 * 0.12) as i16;
    let is_sun = phase > 0.0;

    (x, y, is_sun, day_phase)
}

/// Render the sun with rays
fn render_sun(frame: &mut Frame, area: Rect, cx: i16, cy: i16, frame_index: usize) {
    let sun_color = Color::Rgb(255, 220, 80);
    let ray_color = Color::Rgb(255, 200, 100);
    let glow_color = Color::Rgb(255, 240, 180);

    // Sun body (larger circle effect)
    let sun_chars = [
        (-1, -1, '\u{2591}'), (0, -1, '\u{2592}'), (1, -1, '\u{2592}'), (2, -1, '\u{2591}'),
        (-1, 0, '\u{2592}'), (0, 0, 'O'), (1, 0, 'O'), (2, 0, '\u{2592}'),
        (-1, 1, '\u{2592}'), (0, 1, 'O'), (1, 1, 'O'), (2, 1, '\u{2592}'),
        (-1, 2, '\u{2591}'), (0, 2, '\u{2592}'), (1, 2, '\u{2592}'), (2, 2, '\u{2591}'),
    ];

    for (dx, dy, ch) in sun_chars {
        let x = cx + dx;
        let y = cy + dy;
        if x >= 0 && x < area.width as i16 && y >= 0 && y < area.height as i16 / 2 {
            let color = if ch == 'O' { sun_color } else { glow_color };
            frame.render_widget(
                Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                Rect::new(area.x + x as u16, area.y + y as u16, 1, 1),
            );
        }
    }

    // Animated rays
    let ray_offsets = [
        (-3, 0), (4, 0), (0, -2), (0, 3),
        (-2, -2), (3, -2), (-2, 3), (3, 3),
    ];
    let ray_char = if frame_index % 20 < 10 { '*' } else { '+' };

    for (i, (dx, dy)) in ray_offsets.iter().enumerate() {
        if (frame_index / 5 + i) % 3 == 0 {
            let x = cx + dx;
            let y = cy + dy;
            if x >= 0 && x < area.width as i16 && y >= 0 && y < area.height as i16 / 2 {
                frame.render_widget(
                    Paragraph::new(ray_char.to_string()).style(Style::default().fg(ray_color)),
                    Rect::new(area.x + x as u16, area.y + y as u16, 1, 1),
                );
            }
        }
    }
}

/// Render the moon with glow
fn render_moon(frame: &mut Frame, area: Rect, cx: i16, cy: i16) {
    let moon_color = Color::Rgb(230, 230, 245);
    let glow_color = Color::Rgb(180, 180, 200);

    // Moon body (crescent effect)
    let moon_chars = [
        (0, -1, '\u{2591}'), (1, -1, '\u{2591}'),
        (-1, 0, '\u{2591}'), (0, 0, 'C'), (1, 0, ')'), (2, 0, '\u{2591}'),
        (-1, 1, '\u{2591}'), (0, 1, 'C'), (1, 1, ')'), (2, 1, '\u{2591}'),
        (0, 2, '\u{2591}'), (1, 2, '\u{2591}'),
    ];

    for (dx, dy, ch) in moon_chars {
        let x = cx + dx;
        let y = cy + dy;
        if x >= 0 && x < area.width as i16 && y >= 0 && y < area.height as i16 / 2 {
            let color = if ch == 'C' || ch == ')' { moon_color } else { glow_color };
            frame.render_widget(
                Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                Rect::new(area.x + x as u16, area.y + y as u16, 1, 1),
            );
        }
    }
}

/// Render stars at night
fn render_stars(frame: &mut Frame, area: Rect, day_phase: f32, frame_index: usize) {
    // Stars visible at night: day_phase < 0.25 (night/dawn) or > 0.75 (dusk/night)
    let is_night = day_phase < 0.25 || day_phase > 0.75;
    if !is_night { return; }

    // Calculate star brightness based on how deep into night we are
    let star_brightness = if day_phase < 0.25 {
        1.0 - (day_phase * 4.0) // Fade out towards dawn
    } else {
        (day_phase - 0.75) * 4.0 // Fade in from dusk
    };

    let num_stars = 60; // More stars for a fuller sky

    for i in 0..num_stars {
        let x = (simple_hash(i, 5000) % area.width as usize) as u16;
        let y = (simple_hash(i, 5100) % (area.height as usize / 2)) as u16;

        // Twinkling effect - varied speeds
        let twinkle_speed = 20 + simple_hash(i, 5250) % 25;
        let twinkle = ((frame_index + simple_hash(i, 5200)) % twinkle_speed) as f32 / twinkle_speed as f32;
        let brightness = (220.0 * star_brightness * (0.4 + twinkle * 0.6)) as u8;

        if brightness > 30 {
            // Variety of star characters
            let star_char = match simple_hash(i, 5300) % 8 {
                0 => '✦',
                1 => '✧',
                2 => '+',
                3 => '·',
                _ => '.',
            };

            // Slight color variation - some stars warmer, some cooler
            let tint = simple_hash(i, 5400) % 3;
            let color = match tint {
                0 => Color::Rgb(brightness, brightness, brightness.saturating_add(40)), // Blue-white
                1 => Color::Rgb(brightness.saturating_add(30), brightness, brightness.saturating_sub(20)), // Warm
                _ => Color::Rgb(brightness, brightness, brightness), // Pure white
            };

            frame.render_widget(
                Paragraph::new(star_char.to_string()).style(Style::default().fg(color)),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
}

/// Render shooting stars at night
fn render_shooting_stars(frame: &mut Frame, area: Rect, day_phase: f32, frame_index: usize) {
    // Only at night
    let is_night = day_phase < 0.2 || day_phase > 0.8;
    if !is_night { return; }

    // Only occasional shooting stars
    let star_seed = frame_index / 60;
    if simple_hash(star_seed, 6000) % 8 != 0 { return; }

    let progress = (frame_index % 60) as f32 / 60.0;
    let start_x = (simple_hash(star_seed, 6100) % (area.width as usize / 2)) as i16 + area.width as i16 / 4;
    let start_y = (simple_hash(star_seed, 6200) % (area.height as usize / 4)) as i16;

    // Shooting star trail
    for i in 0..5 {
        let trail_progress = (progress - i as f32 * 0.03).max(0.0);
        let x = start_x + (trail_progress * 20.0) as i16;
        let y = start_y + (trail_progress * 8.0) as i16;

        if x >= 0 && x < area.width as i16 && y >= 0 && y < area.height as i16 / 3 {
            let brightness = (255.0 * (1.0 - i as f32 * 0.2)) as u8;
            let ch = if i == 0 { '*' } else { '-' };
            frame.render_widget(
                Paragraph::new(ch.to_string())
                    .style(Style::default().fg(Color::Rgb(brightness, brightness, brightness))),
                Rect::new(area.x + x as u16, area.y + y as u16, 1, 1),
            );
        }
    }
}

/// Render flying birds
fn render_birds(frame: &mut Frame, area: Rect, day_phase: f32, frame_index: usize) {
    // More birds at dawn/dusk
    let num_birds = if is_transition_period(day_phase) { 8 } else { 4 };

    for i in 0..num_birds {
        let t = frame_index as f32 * 0.03;
        let base_x = (simple_hash(i, 7000) % (area.width as usize * 2)) as f32;
        let speed = 1.5 + (simple_hash(i, 7100) % 10) as f32 * 0.2;

        let x = ((base_x + t * speed * 15.0) % (area.width as f32 * 1.5)) as i16 - (area.width as i16 / 4);
        let y = (simple_hash(i, 7200) % (area.height as usize / 3)) as i16 + 2;

        // Bird flapping animation
        let flap = (frame_index + simple_hash(i, 7300)) % 20;
        let bird_char = if flap < 10 { 'v' } else { '^' };

        // Bird color based on distance/silhouette
        let brightness = if day_phase > 0.4 { 40 } else { 150 };

        if x >= 0 && x < area.width as i16 && y >= 0 && y < area.height as i16 / 2 {
            frame.render_widget(
                Paragraph::new(bird_char.to_string())
                    .style(Style::default().fg(Color::Rgb(brightness, brightness, brightness))),
                Rect::new(area.x + x as u16, area.y + y as u16, 1, 1),
            );
        }
    }
}

/// Get acacia tree color based on time of day, character density, and whether it's trunk or canopy
fn acacia_tree_color(day_phase: f32, ch: char, is_trunk: bool) -> Color {
    // Determine base darkness based on character (for depth effect)
    let density = match ch {
        '█' => 1.0,   // Solid - darkest
        '▓' => 0.85,  // Dense
        '▒' => 0.65,  // Medium
        '░' => 0.45,  // Light - edges catch light
        '▄' => 0.9,   // Lower half - trunk area
        _ => 0.7,
    };

    if day_phase > 0.4 && day_phase < 0.6 {
        // Day - distinct trunk and canopy colors
        if is_trunk {
            // Brown trunk
            let base_r = 70.0 + (1.0 - density) * 25.0;
            let base_g = 45.0 + (1.0 - density) * 15.0;
            let base_b = 25.0 + (1.0 - density) * 10.0;
            Color::Rgb(base_r as u8, base_g as u8, base_b as u8)
        } else {
            // Green canopy
            let base_r = 25.0 + (1.0 - density) * 30.0;
            let base_g = 55.0 + (1.0 - density) * 40.0;
            let base_b = 15.0 + (1.0 - density) * 15.0;
            Color::Rgb(base_r as u8, base_g as u8, base_b as u8)
        }
    } else if is_transition_period(day_phase) {
        // Dawn/Dusk - silhouettes but still visible color difference
        let rim = (1.0 - density) * 40.0;
        if day_phase < 0.5 {
            // Dawn - warm edge glow
            if is_trunk {
                Color::Rgb((25.0 + rim * 0.6) as u8, (15.0 + rim * 0.3) as u8, (8.0) as u8)
            } else {
                Color::Rgb((12.0 + rim * 0.8) as u8, (20.0 + rim * 0.4) as u8, (5.0) as u8)
            }
        } else {
            // Dusk - purple edge glow
            if is_trunk {
                Color::Rgb((20.0 + rim * 0.4) as u8, (12.0) as u8, (8.0 + rim * 0.3) as u8)
            } else {
                Color::Rgb((12.0 + rim * 0.5) as u8, (15.0 + rim * 0.2) as u8, (10.0 + rim * 0.4) as u8)
            }
        }
    } else {
        // Night - deep dark with subtle hints
        let blue_hint = (1.0 - density) * 15.0;
        if is_trunk {
            Color::Rgb(8, 5, 3)
        } else {
            Color::Rgb(3, 6, (8.0 + blue_hint) as u8)
        }
    }
}

/// Render an acacia tree at the given position with depth-based coloring
fn render_acacia_tree(frame: &mut Frame, area: Rect, base_x: u16, base_y: u16, size: usize, day_phase: f32) {
    let pattern = match size {
        0 => ACACIA_SMALL,
        1 => ACACIA_MEDIUM,
        _ => ACACIA_LARGE,
    };

    let tree_height = pattern.len();

    // Trunk starts at different rows depending on tree size
    // Small (5 rows): trunk rows 3-4 (indices 3, 4)
    // Medium (7 rows): trunk rows 4-6 (indices 4, 5, 6)
    // Large (9 rows): trunk rows 6-8 (indices 6, 7, 8)
    let trunk_start_row = match size {
        0 => 3,  // Small
        1 => 4,  // Medium
        _ => 6,  // Large
    };

    for (row_idx, row) in pattern.iter().enumerate() {
        let y = base_y.saturating_sub((tree_height - row_idx) as u16);
        // Strict bounds check
        if y >= area.height {
            continue;
        }

        let row_width = row.chars().count();
        let start_x = base_x.saturating_sub(row_width as u16 / 2);

        // Is this row in the trunk region?
        let is_trunk_row = row_idx >= trunk_start_row;

        for (col_idx, ch) in row.chars().enumerate() {
            if ch == ' ' {
                continue;
            }

            let x = start_x + col_idx as u16;
            // Strict bounds check - must be within area dimensions
            if x >= area.width {
                continue;
            }

            // Trunk detection: in trunk row AND solid block characters
            // Canopy uses ░▒▓ gradient characters, trunk uses solid █ or ▄
            let is_trunk = is_trunk_row && (ch == '█' || ch == '▄');

            // Each character gets its own color based on density and trunk/canopy
            let color = acacia_tree_color(day_phase, ch, is_trunk);

            frame.render_widget(
                Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
}

/// Tree position data - shared between rendering and vegetation zones
const TREE_DATA: [(usize, usize, usize); 12] = [
    // (seed for x, layer, size: 0=small, 1=medium, 2=large)
    (100, 1, 1), (200, 1, 0), (350, 2, 1),
    (450, 2, 2), (550, 1, 0), (650, 2, 1),
    (750, 3, 0), (850, 2, 1), (950, 3, 2),
    (150, 3, 0), (250, 2, 0), (500, 3, 1),
];

/// Check if position is in a vegetation zone near a tree
/// Returns a green tint factor (0.0 = no zone, 1.0 = center of zone)
fn vegetation_zone_factor(x: u16, y: u16, width: u16, height: u16, layer: usize) -> f32 {
    let mut max_factor = 0.0f32;

    for (seed, tree_layer, size) in TREE_DATA {
        // Only check trees on this layer or adjacent
        if tree_layer.abs_diff(layer) > 1 {
            continue;
        }

        let tree_x = (simple_hash(seed, 8000) % width as usize) as i32;
        let tree_hill_y = hill_height(tree_x as u16, width, height, tree_layer) as i32;

        // Vegetation radius based on tree size
        let radius = match size {
            0 => 6,   // Small tree
            1 => 10,  // Medium tree
            _ => 14,  // Large tree
        };

        let dx = (x as i32 - tree_x).abs();
        let dy = (y as i32 - tree_hill_y).abs();

        // Elliptical zone (wider than tall)
        let dist = ((dx as f32 / 1.5).powi(2) + (dy as f32).powi(2)).sqrt();

        if dist < radius as f32 {
            let factor = 1.0 - (dist / radius as f32);
            max_factor = max_factor.max(factor);
        }
    }

    max_factor
}

/// Render acacia trees on hills
fn render_trees(frame: &mut Frame, area: Rect, day_phase: f32) {
    for (seed, layer, size) in TREE_DATA {
        let x = (simple_hash(seed, 8000) % area.width as usize) as u16;
        let hill_y = hill_height(x, area.width, area.height, layer);

        if hill_y > 6 && hill_y < area.height {
            render_acacia_tree(frame, area, x, hill_y, size, day_phase);
        }
    }
}

/// Render fireflies at dusk/dawn
fn render_fireflies(frame: &mut Frame, area: Rect, day_phase: f32, frame_index: usize) {
    if !is_transition_period(day_phase) { return; }

    for i in 0..15 {
        let x = (simple_hash(i, 9000) % area.width as usize) as u16;
        let y = (simple_hash(i, 9100) % (area.height as usize / 2)) as u16 + area.height / 2;

        // Blinking pattern - each firefly has its own rhythm
        let blink_phase = (frame_index + simple_hash(i, 9200)) % 40;
        let is_lit = blink_phase < 8;

        if is_lit && y < area.height {
            let brightness = if blink_phase < 4 { 255 } else { 180 };
            frame.render_widget(
                Paragraph::new("*").style(Style::default().fg(Color::Rgb(brightness, brightness, 50))),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
}

/// Render sun rays during transition periods (dawn/dusk)
fn render_sun_rays(frame: &mut Frame, area: Rect, sun_x: i16, sun_y: i16, day_phase: f32, frame_index: usize) {
    if !is_transition_period(day_phase) { return; }

    let ray_dirs: [(i16, i16); 8] = [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0),           (1, 0),
        (-1, 1),  (0, 1),  (1, 1),
    ];
    let ray_chars = ['/', '|', '\\', '-', '-', '\\', '|', '/'];

    for (i, (dx, dy)) in ray_dirs.iter().enumerate() {
        for dist in 1..15 {
            let x = sun_x + dx * dist;
            let y = sun_y + dy * dist;

            // Bounds check - ensure within area
            if x < 0 || x as u16 >= area.width || y < 0 || y as u16 >= area.height { continue; }
            if (frame_index / 3 + dist as usize) % 3 == 0 { continue; }

            let brightness = (255i32 - dist as i32 * 15).max(50) as u8;
            frame.render_widget(
                Paragraph::new(ray_chars[i].to_string())
                    .style(Style::default().fg(Color::Rgb(brightness, brightness * 3/4, brightness / 2))),
                Rect::new(area.x + x as u16, area.y + y as u16, 1, 1),
            );
        }
    }
}

/// Render heat shimmer effect during peak day hours
fn render_heat_shimmer(frame: &mut Frame, area: Rect, day_phase: f32, frame_index: usize) {
    if day_phase < 0.4 || day_phase > 0.6 { return; }
    if area.height < 3 { return; } // Need minimum height

    let shimmer_chars = ['~', '\u{2248}', '\u{223F}', '~']; // ~, ≈, ∿, ~
    let shimmer_y = area.height * 2 / 3;

    // Bounds check for shimmer_y
    if shimmer_y >= area.height { return; }

    for x in 0..area.width {
        let char_idx = ((x as usize + frame_index / 2) + (frame_index / 5)) % shimmer_chars.len();
        frame.render_widget(
            Paragraph::new(shimmer_chars[char_idx].to_string())
                .style(Style::default().fg(Color::Rgb(200 + (x % 55) as u8, 180 + (x % 40) as u8, 120))),
            Rect::new(area.x + x, area.y + shimmer_y, 1, 1),
        );
    }
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    let day_phase = get_day_phase(frame_index);
    let clouds = get_clouds(area.width, area.height, frame_index);
    let (sun_x, sun_y, is_sun, _) = celestial_body(area.width, area.height, frame_index);

    // Render sky gradient
    for y in 0..area.height {
        for x in 0..area.width {
            let color = sky_color(y, area.height, day_phase);
            frame.render_widget(
                Paragraph::new(" ").style(Style::default().bg(color)),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }

    // Render stars at night
    render_stars(frame, area, day_phase, frame_index);
    render_shooting_stars(frame, area, day_phase, frame_index);

    // Render sun/moon
    if sun_y < area.height as i16 / 2 && sun_y > -5 {
        if is_sun {
            render_sun(frame, area, sun_x, sun_y, frame_index);
            // Render sun rays during transition periods
            render_sun_rays(frame, area, sun_x, sun_y, day_phase, frame_index);
        } else {
            render_moon(frame, area, sun_x, sun_y);
        }
    }

    // Render birds
    render_birds(frame, area, day_phase, frame_index);

    // Render clouds with depth and wispy edges
    for y in 0..area.height / 3 {
        for x in 0..area.width {
            if let Some((cloud_char, cloud_color)) = cloud_at(x, y, &clouds, day_phase) {
                frame.render_widget(
                    Paragraph::new(cloud_char.to_string())
                        .style(Style::default().fg(cloud_color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }

    // Subtle grass characters - only small dots and simple shapes
    let grass_chars = ['·', '\'', ',', '.', '`'];
    let t = frame_index as f32 * 0.04;

    // Render hills (back to front) - 6 layers now
    for layer in (0..6).rev() {
        let base_color = hill_color(layer, day_phase);
        for x in 0..area.width {
            let hill_y = hill_height(x, area.width, area.height, layer);

            // Skip river area on appropriate layers
            let river_layer = 1; // River cuts through layer 1

            for y in hill_y..area.height {
                // Check if this is river position
                if layer == river_layer {
                    let (in_river, is_shimmer) = is_river(x, y, area.width, area.height, frame_index);
                    if in_river {
                        let river_col = river_color(x, y, area.width, area.height, day_phase, is_shimmer);
                        let rchar = river_char(x, y, area.width, area.height, frame_index);
                        frame.render_widget(
                            Paragraph::new(rchar.to_string()).style(Style::default().fg(river_col)),
                            Rect::new(area.x + x, area.y + y, 1, 1),
                        );
                        continue;
                    }

                    // Check for riverbank rocks
                    if let Some(rock_color) = is_riverbank_rock(x, y, area.width, area.height, frame_index) {
                        let rock_chars = ['•', '○', '◦'];
                        let rock_idx = simple_hash(x as usize + y as usize * 7, 1234) % rock_chars.len();
                        frame.render_widget(
                            Paragraph::new(rock_chars[rock_idx].to_string())
                                .style(Style::default().fg(rock_color)),
                            Rect::new(area.x + x, area.y + y, 1, 1),
                        );
                        continue;
                    }
                }

                // Apply vegetation zone green tint near trees
                let veg_factor = vegetation_zone_factor(x, y, area.width, area.height, layer);
                let hill_col = if veg_factor > 0.0 {
                    // Tint towards lush green in vegetation zones
                    if let Color::Rgb(br, bg, bb) = base_color {
                        // Richer, darker green near trees
                        let green_r = (br as f32 * 0.7) as u8;
                        let green_g = (bg as f32 * 1.15).min(255.0) as u8;
                        let green_b = (bb as f32 * 0.6) as u8;
                        // Blend based on vegetation factor
                        let r = (br as f32 * (1.0 - veg_factor) + green_r as f32 * veg_factor) as u8;
                        let g = (bg as f32 * (1.0 - veg_factor) + green_g as f32 * veg_factor) as u8;
                        let b = (bb as f32 * (1.0 - veg_factor) + green_b as f32 * veg_factor) as u8;
                        Color::Rgb(r, g, b)
                    } else {
                        base_color
                    }
                } else {
                    base_color
                };

                // Sparse grass texture only on hill edges (first 2 rows) of close layers
                let rows_from_top = y.saturating_sub(hill_y);
                let is_grass_zone = layer <= 1 && rows_from_top <= 2;

                if is_grass_zone {
                    let fx = x as f32;

                    // Gentle breathing wave
                    let wave = fast_sin(fx * 0.06 + t * 1.2);
                    let breath = (wave + 1.0) / 2.0;

                    // Very sparse - only ~20% of cells get a character
                    let cell_hash = simple_hash(x as usize * 31 + y as usize * 17, 7777);
                    let show_grass = cell_hash % 5 == 0;

                    if show_grass && breath > 0.4 {
                        let Color::Rgb(br, bg, bb) = base_color else { continue };
                        // Slightly lighter for grass highlights
                        let intensity = 1.0 + breath * 0.25;
                        let r = (br as f32 * intensity).min(255.0) as u8;
                        let g = (bg as f32 * intensity).min(255.0) as u8;
                        let b = (bb as f32 * intensity).min(255.0) as u8;

                        let char_idx = simple_hash(x as usize, (t * 0.3) as usize) % grass_chars.len();
                        frame.render_widget(
                            Paragraph::new(grass_chars[char_idx].to_string())
                                .style(Style::default().fg(Color::Rgb(r, g, b))),
                            Rect::new(area.x + x, area.y + y, 1, 1),
                        );
                    } else {
                        // Solid hill color
                        let ch = if y == hill_y { '\u{2593}' } else { '\u{2588}' };
                        frame.render_widget(
                            Paragraph::new(ch.to_string()).style(Style::default().fg(base_color)),
                            Rect::new(area.x + x, area.y + y, 1, 1),
                        );
                    }
                } else {
                    // Standard solid hill rendering
                    let ch = if y == hill_y { '\u{2593}' } else { '\u{2588}' };
                    frame.render_widget(
                        Paragraph::new(ch.to_string()).style(Style::default().fg(base_color)),
                        Rect::new(area.x + x, area.y + y, 1, 1),
                    );
                }
            }
        }
    }

    // Render heat shimmer during peak day
    render_heat_shimmer(frame, area, day_phase, frame_index);

    // Render trees on hills
    render_trees(frame, area, day_phase);

    // Render fireflies at dusk/dawn
    render_fireflies(frame, area, day_phase, frame_index);

    // Add natural grass tufts and wildflowers on closest hill
    render_grass_and_flowers(frame, area, day_phase, frame_index);
}

/// Render grass field with breathing special characters effect
/// Similar to synthwave floor but with earthy green tones
fn render_grass_and_flowers(frame: &mut Frame, area: Rect, day_phase: f32, frame_index: usize) {
    // Special characters for breathing grass effect
    let grass_chars = ['*', '·', '°', '∘', '+', '×', '•', '`', '\'', ',', '"', '^', '~', '>', '<'];

    let t = frame_index as f32 * 0.06; // Animation time

    // Get the foreground hill boundary
    let hill_0_start = (0..area.width)
        .map(|x| hill_height(x, area.width, area.height, 0))
        .min()
        .unwrap_or(area.height);

    // Render breathing grass across the foreground area (multiple rows)
    for y in hill_0_start.saturating_sub(3)..area.height {
        for x in 0..area.width {
            let hill_y = hill_height(x, area.width, area.height, 0);

            // Only render in the grass zone (on top of foreground hill)
            if y > hill_y || y < hill_y.saturating_sub(4) {
                continue;
            }

            let fx = x as f32;
            let fy = y as f32;

            // === BREATHING WAVE SYSTEM ===
            // Multiple overlapping waves for organic feel (like synthwave)
            let wave1 = fast_sin(fx * 0.08 + fy * 0.05 + t * 1.5);
            let wave2 = fast_sin(fx * 0.12 + fy * 0.08 - t * 0.8);
            let wave3 = fast_sin(fx * 0.04 + t * 2.0);
            let breath = (wave1 * 0.4 + wave2 * 0.35 + wave3 * 0.25 + 1.0) / 2.0;

            // Day/night factor
            let night_factor = 0.3 + day_phase * 0.7;

            // === COLOR: earthy greens with breathing intensity ===
            let depth = (y as f32 - hill_0_start as f32 + 4.0) / 6.0;
            let base_intensity = 25.0 + depth * 15.0;
            let breath_boost = breath * 45.0;

            // Green-dominant with earth tones
            let r = ((base_intensity + breath_boost * 0.6) * night_factor) as u8;
            let g = ((base_intensity * 1.6 + breath_boost) * night_factor) as u8;
            let b = ((base_intensity * 0.5 + breath_boost * 0.3) * night_factor) as u8;

            // === CHARACTER SELECTION ===
            let char_seed = simple_hash(x as usize + y as usize * 100, (t * 2.0) as usize);
            let char_idx = char_seed % grass_chars.len();

            // Some cells empty for breathing space
            let show_char = (breath > 0.25) && (simple_hash(x as usize, y as usize + (t * 0.5) as usize) % 3 != 0);

            if show_char {
                frame.render_widget(
                    Paragraph::new(grass_chars[char_idx].to_string())
                        .style(Style::default().fg(Color::Rgb(r, g, b))),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }

    // Scattered wildflowers - sparse, colorful accents
    for i in 0..20 {
        let x = (simple_hash(i + 100, 4000) % area.width as usize) as u16;
        let hill_y = hill_height(x, area.width, area.height, 0);

        if hill_y <= 2 || hill_y >= area.height {
            continue;
        }

        let flower_y = hill_y.saturating_sub(1 + (simple_hash(i, 4500) % 3) as u16);
        if x >= area.width || flower_y >= area.height {
            continue;
        }

        // Only show flowers during day
        if day_phase < 0.25 || day_phase > 0.75 {
            continue;
        }

        let flower_chars = ['✿', '❀', '✾', '❁', '✻', '⚘'];
        let ch = flower_chars[simple_hash(i, 5000) % flower_chars.len()];

        // Flower colors - varied
        let color_idx = simple_hash(i, 6000) % 4;
        let night_factor = 0.5 + day_phase * 0.5;
        let color = match color_idx {
            0 => Color::Rgb((180.0 * night_factor) as u8, (120.0 * night_factor) as u8, (80.0 * night_factor) as u8),  // Golden
            1 => Color::Rgb((160.0 * night_factor) as u8, (100.0 * night_factor) as u8, (130.0 * night_factor) as u8), // Lavender
            2 => Color::Rgb((200.0 * night_factor) as u8, (180.0 * night_factor) as u8, (140.0 * night_factor) as u8), // Cream
            _ => Color::Rgb((140.0 * night_factor) as u8, (110.0 * night_factor) as u8, (90.0 * night_factor) as u8),  // Dusty rose
        };

        frame.render_widget(
            Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
            Rect::new(area.x + x, area.y + flower_y, 1, 1),
        );
    }
}
