use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Medieval - Epic fantasy castle at night with siege atmosphere
/// Features: Dragon silhouette, smoke/mist, realistic torches with embers,
/// patrolling guards, distant army, waving banners, trebuchet, owls/bats,
/// lightning flashes, glowing windows

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

/// Fast cosine approximation
fn fast_cos(x: f32) -> f32 {
    fast_sin(x + std::f32::consts::PI / 2.0)
}

/// Castle tower structure
struct Tower {
    x: u16,
    width: u16,
    height: u16,
    has_flag: bool,
    crenellation: bool,
    has_window: bool,
    flag_color: (u8, u8, u8),
}

fn get_towers(area_width: u16, area_height: u16) -> Vec<Tower> {
    vec![
        Tower {
            x: 3, width: 10, height: area_height * 9 / 20,
            has_flag: true, crenellation: true, has_window: true,
            flag_color: (180, 30, 30) // Red
        },
        Tower {
            x: 15, width: 14, height: area_height * 7 / 20,
            has_flag: false, crenellation: true, has_window: true,
            flag_color: (30, 30, 180) // Blue
        },
        Tower {
            x: area_width / 2 - 7, width: 14, height: area_height * 11 / 20,
            has_flag: true, crenellation: true, has_window: true,
            flag_color: (180, 150, 30) // Gold
        },
        Tower {
            x: area_width - 25, width: 10, height: area_height * 8 / 20,
            has_flag: true, crenellation: true, has_window: true,
            flag_color: (30, 120, 30) // Green
        },
        Tower {
            x: area_width - 12, width: 8, height: area_height * 7 / 20,
            has_flag: true, crenellation: true, has_window: false,
            flag_color: (120, 30, 120) // Purple
        },
    ]
}

/// Enhanced stone texture - mostly solid blocks with subtle detail
fn stone_char(x: u16, y: u16, _frame_index: usize) -> char {
    let pattern = simple_hash(x as usize * 31 + y as usize * 17, 100);

    // Horizontal mortar lines between stone blocks (sparse)
    if y % 4 == 0 && pattern % 12 == 0 {
        return '▀'; // Upper half block for mortar line effect
    }

    // Vertical mortar joints (rare)
    if pattern % 40 == 0 && y % 4 != 0 {
        return '▌'; // Left half block for vertical joint
    }

    // Mostly solid blocks with occasional texture variation
    match pattern % 20 {
        0..=14 => '█',  // 75% solid block
        15..=17 => '▓', // 15% dark shade (subtle shadow)
        18 => '▒',      // 5% medium shade (deep crack)
        _ => '█',       // Default to solid
    }
}

/// Enhanced stone color with more variation - depth through color, not transparency
fn stone_color(x: u16, y: u16, lightning_flash: bool) -> Color {
    let base_variation = simple_hash(x as usize + y as usize * 50, 200) % 25;
    let depth_pattern = simple_hash(x as usize * 13 + y as usize * 7, 250) % 100;

    // Base stone color - solid and visible
    let mut base = 55 + base_variation as u8;

    // Add depth through color variation
    // Horizontal "block" shadows (every 4 rows)
    if y % 4 == 0 {
        base = base.saturating_sub(12); // Darker mortar lines
    }

    // Random darker stones for variety
    if depth_pattern < 20 {
        base = base.saturating_sub(15);
    }
    // Random lighter stones (highlights)
    if depth_pattern > 85 {
        base = base.saturating_add(12);
    }

    // Vertical depth gradient - lower stones slightly darker
    let y_factor = (y as f32 / 40.0).min(1.0);
    base = base.saturating_sub((y_factor * 8.0) as u8);

    // Lightning illumination
    if lightning_flash {
        base = base.saturating_add(70);
    }

    // Warmer, more solid stone tones (brownish-gray)
    Color::Rgb(
        base.saturating_add(5),        // Slightly warmer
        base.saturating_sub(3),
        base.saturating_sub(12)        // Less blue = warmer
    )
}

/// Torch flicker effect - more erratic
fn torch_brightness(torch_id: usize, frame_index: usize) -> f32 {
    let t = frame_index as f32 * 0.2;
    let flicker1 = fast_sin(t + torch_id as f32 * 2.0) * 0.2;
    let flicker2 = fast_sin(t * 3.1 + torch_id as f32 * 1.3) * 0.15;
    let flicker3 = fast_sin(t * 5.7 + torch_id as f32 * 0.7) * 0.1;
    let flicker4 = fast_sin(t * 8.3 + torch_id as f32 * 2.1) * 0.05;

    0.65 + flicker1 + flicker2 + flicker3 + flicker4
}

/// Render enhanced torch with flame and floating embers
fn render_torch(frame: &mut Frame, area: Rect, x: u16, y: u16, torch_id: usize, frame_index: usize) {
    if x >= area.width || y >= area.height { return; }

    let brightness = torch_brightness(torch_id, frame_index);

    // Torch bracket
    if x > 0 && y + 1 < area.height {
        frame.render_widget(
            Paragraph::new("╢").style(Style::default().fg(Color::Rgb(60, 45, 25))),
            Rect::new(area.x + x - 1, area.y + y + 1, 1, 1),
        );
    }

    // Torch handle
    if y + 1 < area.height {
        frame.render_widget(
            Paragraph::new("║").style(Style::default().fg(Color::Rgb(90, 55, 25))),
            Rect::new(area.x + x, area.y + y + 1, 1, 1),
        );
    }

    // Main flame - multi-layered
    let flame_r = (255.0 * brightness) as u8;
    let flame_g = (180.0 * brightness) as u8;
    let flame_b = (50.0 * brightness * 0.4) as u8;

    let flame_chars = ['*', '^', '▲', '◆', '♦', '⬥'];
    let flame_idx = (frame_index / 4 + torch_id) % flame_chars.len();

    frame.render_widget(
        Paragraph::new(flame_chars[flame_idx].to_string())
            .style(Style::default().fg(Color::Rgb(flame_r, flame_g, flame_b))),
        Rect::new(area.x + x, area.y + y, 1, 1),
    );

    // Inner flame (white hot core)
    if y > 0 && (frame_index / 3 + torch_id) % 4 != 0 {
        frame.render_widget(
            Paragraph::new("·").style(Style::default().fg(Color::Rgb(255, 255, 200))),
            Rect::new(area.x + x, area.y + y, 1, 1),
        );
    }

    // Floating embers rising from torch
    for ember_i in 0..4 {
        let ember_offset = (frame_index + torch_id * 17 + ember_i * 23) % 60;
        if ember_offset < 40 {
            let ember_y = y.saturating_sub((ember_offset / 4) as u16 + 1);
            let wobble = fast_sin((frame_index + ember_i * 11) as f32 * 0.3) * 2.0;
            let ember_x = (x as f32 + wobble) as u16;

            if ember_x < area.width && ember_y > 0 && ember_y < area.height {
                let ember_fade = 1.0 - (ember_offset as f32 / 40.0);
                let er = (255.0 * ember_fade * brightness) as u8;
                let eg = (100.0 * ember_fade * brightness) as u8;

                let ember_char = if ember_offset < 20 { '.' } else { '·' };
                frame.render_widget(
                    Paragraph::new(ember_char.to_string())
                        .style(Style::default().fg(Color::Rgb(er, eg, 10))),
                    Rect::new(area.x + ember_x, area.y + ember_y, 1, 1),
                );
            }
        }
    }

    // Glow effect on nearby walls (larger radius)
    for dy in -2i16..=2 {
        for dx in -2i16..=2 {
            if dx == 0 && dy == 0 { continue; }
            let dist = ((dx * dx + dy * dy) as f32).sqrt();
            if dist > 2.5 { continue; }

            let nx = x as i16 + dx;
            let ny = y as i16 + dy;
            if nx >= 0 && ny >= 0 && nx < area.width as i16 && ny < area.height as i16 {
                let glow_intensity = brightness * (1.0 - dist / 3.0) * 0.4;
                let gr = (220.0 * glow_intensity) as u8;
                let gg = (120.0 * glow_intensity) as u8;
                let gb = (30.0 * glow_intensity) as u8;

                frame.render_widget(
                    Paragraph::new("░").style(Style::default().fg(Color::Rgb(gr, gg, gb))),
                    Rect::new(area.x + nx as u16, area.y + ny as u16, 1, 1),
                );
            }
        }
    }
}

/// Render realistic waving banner with cloth physics
fn render_banner(frame: &mut Frame, area: Rect, x: u16, y: u16, frame_index: usize, color: (u8, u8, u8)) {
    if x >= area.width || y >= area.height { return; }

    // Flag pole
    for pole_y in 0..3 {
        if y + pole_y < area.height {
            frame.render_widget(
                Paragraph::new("│").style(Style::default().fg(Color::Rgb(70, 50, 30))),
                Rect::new(area.x + x, area.y + y + pole_y, 1, 1),
            );
        }
    }

    // Waving flag - multiple segments for cloth effect
    let t = frame_index as f32 * 0.15;
    let flag_length = 4u16;

    for i in 0..flag_length {
        let wave_phase = t + i as f32 * 0.8;
        let wave_y = (fast_sin(wave_phase) * 0.8) as i16;
        let wave_x = i + 1;

        let fx = x + wave_x;
        let fy = (y as i16 + wave_y) as u16;

        if fx < area.width && fy < area.height {
            // Flag segment with shading based on wave position
            let shade = (fast_sin(wave_phase) * 0.3 + 0.7) as f32;
            let r = (color.0 as f32 * shade) as u8;
            let g = (color.1 as f32 * shade) as u8;
            let b = (color.2 as f32 * shade) as u8;

            let flag_char = if i == flag_length - 1 { '▸' } else { '█' };
            frame.render_widget(
                Paragraph::new(flag_char.to_string())
                    .style(Style::default().fg(Color::Rgb(r, g, b))),
                Rect::new(area.x + fx, area.y + fy, 1, 1),
            );
        }
    }
}

/// Render glowing castle window
fn render_window(frame: &mut Frame, area: Rect, x: u16, y: u16, frame_index: usize, window_id: usize) {
    if x >= area.width || y >= area.height { return; }

    // Some windows flicker (candle), some are steady
    let flicker = if window_id % 3 == 0 {
        0.7 + fast_sin(frame_index as f32 * 0.3 + window_id as f32) * 0.3
    } else {
        0.9
    };

    let warm_r = (200.0 * flicker) as u8;
    let warm_g = (150.0 * flicker) as u8;
    let warm_b = (80.0 * flicker) as u8;

    // Window shape - Gothic arch
    frame.render_widget(
        Paragraph::new("▄").style(Style::default().fg(Color::Rgb(warm_r, warm_g, warm_b))),
        Rect::new(area.x + x, area.y + y, 1, 1),
    );

    // Window glow
    for dy in -1i16..=1 {
        for dx in -1i16..=1 {
            if dx == 0 && dy == 0 { continue; }
            let nx = x as i16 + dx;
            let ny = y as i16 + dy;
            if nx >= 0 && ny >= 0 && nx < area.width as i16 && ny < area.height as i16 {
                let glow_r = (warm_r as f32 * 0.3) as u8;
                let glow_g = (warm_g as f32 * 0.3) as u8;
                let glow_b = (warm_b as f32 * 0.3) as u8;
                frame.render_widget(
                    Paragraph::new("·").style(Style::default().fg(Color::Rgb(glow_r, glow_g, glow_b))),
                    Rect::new(area.x + nx as u16, area.y + ny as u16, 1, 1),
                );
            }
        }
    }
}

/// Render castle silhouette with enhanced details
fn render_castle(frame: &mut Frame, area: Rect, frame_index: usize, lightning_flash: bool) {
    let towers = get_towers(area.width, area.height);

    for (tower_idx, tower) in towers.iter().enumerate() {
        let tower_top = area.height.saturating_sub(tower.height);

        // Tower body with enhanced stone
        for y in tower_top..area.height {
            for x in tower.x..tower.x + tower.width {
                if x < area.width {
                    let ch = stone_char(x, y, frame_index);
                    let color = stone_color(x, y, lightning_flash);

                    frame.render_widget(
                        Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                        Rect::new(area.x + x, area.y + y, 1, 1),
                    );
                }
            }
        }

        // Crenellations (battlements) - more detailed
        if tower.crenellation && tower_top > 1 {
            for i in 0..(tower.width / 2) {
                let cx = tower.x + i * 2;
                if cx < area.width {
                    // Merlon (raised part)
                    frame.render_widget(
                        Paragraph::new("▀").style(Style::default().fg(stone_color(cx, tower_top - 1, lightning_flash))),
                        Rect::new(area.x + cx, area.y + tower_top - 1, 1, 1),
                    );
                }
            }
        }

        // Waving banner
        if tower.has_flag && tower_top > 4 {
            let flag_x = tower.x + tower.width / 2;
            if flag_x < area.width {
                render_banner(frame, area, flag_x, tower_top.saturating_sub(4), frame_index, tower.flag_color);
            }
        }

        // Glowing windows
        if tower.has_window {
            let window_y = tower_top + tower.height / 3;
            let window_x = tower.x + tower.width / 2;
            render_window(frame, area, window_x, window_y, frame_index, tower_idx);

            // Second window on taller towers
            if tower.height > area.height / 3 {
                let window_y2 = tower_top + tower.height * 2 / 3;
                render_window(frame, area, window_x, window_y2, frame_index, tower_idx + 10);
            }
        }
    }

    // Connecting wall between towers
    let wall_height = area.height / 4;
    let wall_top = area.height - wall_height;
    for x in 0..area.width {
        for y in wall_top..area.height {
            let in_tower = towers.iter().any(|t| x >= t.x && x < t.x + t.width);
            if !in_tower {
                let ch = stone_char(x, y, frame_index);
                let color = stone_color(x, y, lightning_flash);

                frame.render_widget(
                    Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }

    // Wall crenellations
    for x in 0..area.width {
        let in_tower = towers.iter().any(|t| x >= t.x && x < t.x + t.width);
        if !in_tower && x % 3 == 0 && wall_top > 0 {
            frame.render_widget(
                Paragraph::new("▀").style(Style::default().fg(stone_color(x, wall_top - 1, lightning_flash))),
                Rect::new(area.x + x, area.y + wall_top - 1, 1, 1),
            );
        }
    }
}

/// Render patrolling guards on battlements
fn render_guards(frame: &mut Frame, area: Rect, frame_index: usize) {
    let towers = get_towers(area.width, area.height);

    // Guard patrol paths along wall tops
    for (i, tower) in towers.iter().enumerate() {
        let guard_speed = 120 + (i * 30);
        let patrol_range = tower.width as usize;
        let guard_pos = (frame_index / 8) % (patrol_range * 2);
        let guard_x = if guard_pos < patrol_range {
            tower.x + guard_pos as u16
        } else {
            tower.x + (patrol_range * 2 - guard_pos) as u16
        };

        let tower_top = area.height.saturating_sub(tower.height);

        if guard_x < area.width && tower_top > 1 && i % 2 == 0 {
            // Guard figure (tiny)
            frame.render_widget(
                Paragraph::new("♟").style(Style::default().fg(Color::Rgb(60, 60, 70))),
                Rect::new(area.x + guard_x, area.y + tower_top - 2, 1, 1),
            );
        }
    }

    // Guards on main wall
    let wall_top = area.height - area.height / 4;
    for i in 0..3 {
        let patrol_start = area.width / 4 + i * area.width / 4;
        let patrol_range = area.width / 6;
        let guard_pos = ((frame_index / 10 + i as usize * 40) % (patrol_range as usize * 2)) as u16;
        let guard_x = if guard_pos < patrol_range {
            patrol_start + guard_pos
        } else {
            patrol_start + patrol_range * 2 - guard_pos
        };

        if guard_x < area.width && wall_top > 1 {
            let in_tower = get_towers(area.width, area.height).iter().any(|t| guard_x >= t.x && guard_x < t.x + t.width);
            if !in_tower {
                frame.render_widget(
                    Paragraph::new("♟").style(Style::default().fg(Color::Rgb(50, 50, 60))),
                    Rect::new(area.x + guard_x, area.y + wall_top - 2, 1, 1),
                );
            }
        }
    }
}

/// Render distant army on horizon
fn render_distant_army(frame: &mut Frame, area: Rect, frame_index: usize) {
    let horizon_y = area.height * 2 / 3 - 2;

    // Army ranks - rows of tiny dots
    for row in 0..3 {
        let row_y = horizon_y + row;
        if row_y >= area.height { continue; }

        for col in 0..40 {
            let x = area.width / 6 + col * 2;
            if x >= area.width { continue; }

            // Stagger the rows
            let offset = if row % 2 == 0 { 0 } else { 1 };
            let soldier_x = x + offset;

            if soldier_x < area.width {
                // Occasional glint from armor/weapons
                let glint = simple_hash(col as usize + row as usize * 100 + frame_index / 20, 999) % 30 == 0;
                let color = if glint {
                    Color::Rgb(200, 200, 180) // Armor glint
                } else {
                    Color::Rgb(30 + row as u8 * 5, 25 + row as u8 * 5, 20 + row as u8 * 5)
                };

                frame.render_widget(
                    Paragraph::new("·").style(Style::default().fg(color)),
                    Rect::new(area.x + soldier_x, area.y + row_y, 1, 1),
                );
            }
        }
    }

    // Enemy torches in the distance
    for i in 0..8 {
        let torch_x = area.width / 5 + i * area.width / 10;
        if torch_x < area.width {
            let flicker = 0.6 + fast_sin(frame_index as f32 * 0.2 + i as f32) * 0.4;
            let r = (180.0 * flicker) as u8;
            let g = (100.0 * flicker) as u8;
            frame.render_widget(
                Paragraph::new("*").style(Style::default().fg(Color::Rgb(r, g, 20))),
                Rect::new(area.x + torch_x, area.y + horizon_y - 1, 1, 1),
            );
        }
    }
}

/// Render trebuchet with occasional firing
fn render_trebuchet(frame: &mut Frame, area: Rect, frame_index: usize) {
    let treb_x = area.width / 8;
    let treb_y = area.height * 2 / 3 + 2;

    if treb_x + 5 >= area.width || treb_y + 3 >= area.height { return; }

    // Trebuchet base
    let treb_chars = ["╔═══╗", " ╠═╣ ", "╔╩═╩╗"];
    for (i, line) in treb_chars.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            let px = treb_x + j as u16;
            let py = treb_y + i as u16;
            if px < area.width && py < area.height {
                frame.render_widget(
                    Paragraph::new(ch.to_string())
                        .style(Style::default().fg(Color::Rgb(70, 50, 30))),
                    Rect::new(area.x + px, area.y + py, 1, 1),
                );
            }
        }
    }

    // Trebuchet arm - swinging
    let fire_cycle = frame_index % 300;
    let arm_angle = if fire_cycle < 20 {
        // Firing animation
        fire_cycle as f32 * 0.15
    } else {
        0.0
    };

    let arm_end_x = treb_x + 2 + (fast_cos(arm_angle) * 3.0) as u16;
    let arm_end_y = treb_y.saturating_sub((fast_sin(arm_angle) * 2.0) as u16 + 1);

    if arm_end_x < area.width && arm_end_y < area.height {
        frame.render_widget(
            Paragraph::new("/").style(Style::default().fg(Color::Rgb(80, 60, 40))),
            Rect::new(area.x + arm_end_x, area.y + arm_end_y, 1, 1),
        );
    }

    // Projectile arc when firing
    if fire_cycle > 15 && fire_cycle < 80 {
        let t = (fire_cycle - 15) as f32 / 65.0;
        let proj_x = treb_x as f32 + t * area.width as f32 * 0.6;
        let proj_y = treb_y as f32 - (fast_sin(t * std::f32::consts::PI) * (area.height as f32 * 0.4));

        let px = proj_x as u16;
        let py = proj_y as u16;

        if px < area.width && py < area.height && py > 0 {
            frame.render_widget(
                Paragraph::new("●").style(Style::default().fg(Color::Rgb(100, 90, 80))),
                Rect::new(area.x + px, area.y + py, 1, 1),
            );
        }
    }
}

/// Render dragon silhouette flying across moon
fn render_dragon(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Dragon appears every ~400 frames
    let dragon_cycle = frame_index % 400;
    if dragon_cycle > 120 { return; }

    let t = dragon_cycle as f32 / 120.0;
    let dragon_x = (t * area.width as f32 * 1.3 - area.width as f32 * 0.15) as i16;
    let base_y = (area.height as f32 * 0.15) as i16;

    // Wing flap animation
    let wing_phase = (frame_index as f32 * 0.4).sin();

    // Dragon body parts (simplified ASCII art)
    let dragon_parts: [(i16, i16, &str); 7] = [
        (0, 0, "◄"),      // Head
        (1, 0, "═"),      // Neck
        (2, 0, "█"),      // Body
        (3, 0, "█"),      // Body
        (4, 0, "══"),     // Tail
        (2, if wing_phase > 0.0 { -1 } else { 1 }, "∧"), // Wing up/down
        (3, if wing_phase > 0.0 { -1 } else { 1 }, "∧"), // Wing up/down
    ];

    for (dx, dy, ch) in dragon_parts.iter() {
        let px = dragon_x + dx;
        let py = base_y + dy;

        if px >= 0 && px < area.width as i16 && py >= 0 && py < area.height as i16 {
            frame.render_widget(
                Paragraph::new(*ch).style(Style::default().fg(Color::Rgb(20, 20, 25))),
                Rect::new(area.x + px as u16, area.y + py as u16, 1, 1),
            );
        }
    }
}

/// Render owls and bats flying near towers
fn render_flying_creatures(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Bats - erratic flight pattern
    for i in 0..6 {
        let base_x = simple_hash(i, 1000) % area.width as usize;
        let base_y = simple_hash(i, 1001) % (area.height as usize / 2);

        let t = frame_index as f32 * 0.1 + i as f32 * 2.0;
        let x = (base_x as f32 + fast_sin(t) * 8.0 + (frame_index as f32 * 0.3)) as u16 % area.width;
        let y = (base_y as f32 + fast_cos(t * 1.3) * 4.0) as u16;

        if x < area.width && y < area.height && y > 2 {
            // Wing flap
            let bat_char = if (frame_index / 4 + i) % 2 == 0 { 'w' } else { 'v' };
            frame.render_widget(
                Paragraph::new(bat_char.to_string())
                    .style(Style::default().fg(Color::Rgb(30, 30, 35))),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }

    // Owls - more graceful, near towers
    for i in 0..2 {
        let tower_x = if i == 0 { area.width / 3 } else { area.width * 2 / 3 };
        let base_y = area.height / 4;

        let t = frame_index as f32 * 0.05 + i as f32 * 3.14;
        let x = (tower_x as f32 + fast_sin(t) * 6.0) as u16;
        let y = (base_y as f32 + fast_cos(t * 0.7) * 3.0) as u16;

        if x < area.width && y < area.height && y > 0 {
            frame.render_widget(
                Paragraph::new("^O^").style(Style::default().fg(Color::Rgb(80, 70, 60))),
                Rect::new(area.x + x.saturating_sub(1), area.y + y, 3, 1),
            );
        }
    }
}

/// Render rising smoke from chimneys
fn render_smoke(frame: &mut Frame, area: Rect, frame_index: usize) {
    let towers = get_towers(area.width, area.height);

    // Chimney smoke from select towers
    for (i, tower) in towers.iter().enumerate() {
        if i % 2 != 0 { continue; }

        let chimney_x = tower.x + tower.width / 2;
        let tower_top = area.height.saturating_sub(tower.height);

        // Multiple smoke particles
        for p in 0..8 {
            let particle_offset = (frame_index + p * 15) % 80;
            let rise = particle_offset as f32 / 10.0;
            let drift = fast_sin((frame_index + p * 7) as f32 * 0.1) * (rise * 0.5);

            let smoke_x = (chimney_x as f32 + drift) as u16;
            let smoke_y = tower_top.saturating_sub(rise as u16 + 1);

            if smoke_x < area.width && smoke_y > 0 && smoke_y < area.height {
                let fade = 1.0 - (particle_offset as f32 / 80.0);
                let gray = (60.0 * fade) as u8 + 20;
                let smoke_char = if particle_offset < 30 { '░' } else if particle_offset < 50 { '·' } else { '.' };

                frame.render_widget(
                    Paragraph::new(smoke_char.to_string())
                        .style(Style::default().fg(Color::Rgb(gray, gray, gray + 5))),
                    Rect::new(area.x + smoke_x, area.y + smoke_y, 1, 1),
                );
            }
        }
    }
}

/// Render ground fog
fn render_ground_fog(frame: &mut Frame, area: Rect, frame_index: usize) {
    let fog_y = area.height - 3;

    for x in 0..area.width {
        // Fog rolls and shifts
        let t = frame_index as f32 * 0.02 + x as f32 * 0.1;
        let fog_intensity = (fast_sin(t) * 0.5 + 0.5) * 0.6;
        let fog_height = (fast_sin(t * 0.7 + x as f32 * 0.05) * 2.0 + 2.0) as u16;

        for dy in 0..fog_height.min(3) {
            let y = fog_y + dy;
            if y < area.height {
                let layer_fade = 1.0 - (dy as f32 / 3.0);
                let gray = (50.0 * fog_intensity * layer_fade) as u8 + 15;

                frame.render_widget(
                    Paragraph::new("░").style(Style::default().fg(Color::Rgb(gray, gray, gray + 10))),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }
}

/// Enhanced night sky with depth gradient and detailed moon
fn render_sky(frame: &mut Frame, area: Rect, frame_index: usize, lightning_flash: bool) {
    let sky_height = area.height * 2 / 3;

    // Gradient sky - darker at top, lighter purple/blue at horizon
    for y in 0..sky_height {
        for x in 0..area.width {
            let fy = y as f32 / sky_height as f32;

            // Deeper gradient
            let (r, g, b) = if lightning_flash {
                // Lightning illumination
                let base_r = 5.0 + fy * 15.0;
                let base_g = 8.0 + fy * 20.0;
                let base_b = 20.0 + fy * 35.0;
                (
                    (base_r + 100.0) as u8,
                    (base_g + 100.0) as u8,
                    (base_b + 120.0) as u8,
                )
            } else {
                (
                    (5.0 + fy * 15.0) as u8,
                    (8.0 + fy * 20.0) as u8,
                    (20.0 + fy * 35.0) as u8,
                )
            };

            frame.render_widget(
                Paragraph::new(" ").style(Style::default().bg(Color::Rgb(r, g, b))),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }

    // Horizon glow (distant fires/dawn approaching)
    for x in 0..area.width {
        let horizon_y = sky_height;
        if horizon_y < area.height {
            let glow = fast_sin(x as f32 * 0.05 + frame_index as f32 * 0.01) * 0.3 + 0.4;
            let r = (40.0 * glow) as u8;
            let g = (20.0 * glow) as u8;
            let b = (35.0 + glow * 10.0) as u8;

            frame.render_widget(
                Paragraph::new("▄").style(Style::default().fg(Color::Rgb(r, g, b))),
                Rect::new(area.x + x, area.y + horizon_y, 1, 1),
            );
        }
    }

    // Stars - different sizes and twinkle patterns
    for i in 0..40 {
        let x = (simple_hash(i, 600) % area.width as usize) as u16;
        let y = (simple_hash(i, 700) % (sky_height as usize - 2)) as u16;

        // Twinkle with different rates
        let twinkle_rate = 20 + simple_hash(i, 750) % 20;
        let twinkle = (frame_index + i * 11) % twinkle_rate < (twinkle_rate - 5);

        if twinkle && !lightning_flash {
            let brightness = 120 + (simple_hash(i, 800) % 135) as u8;
            let star_size = simple_hash(i, 850) % 10;
            let star_char = if star_size < 3 { '.' } else if star_size < 7 { '*' } else { '+' };

            // Slight color variation (blue/white/yellow stars)
            let color_type = simple_hash(i, 860) % 3;
            let (r, g, b) = match color_type {
                0 => (brightness, brightness, brightness.saturating_add(20)), // Blue-ish
                1 => (brightness.saturating_add(10), brightness.saturating_add(10), brightness.saturating_sub(20)), // Yellow-ish
                _ => (brightness, brightness, brightness), // White
            };

            frame.render_widget(
                Paragraph::new(star_char.to_string())
                    .style(Style::default().fg(Color::Rgb(r, g, b))),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }

    // Detailed crescent moon with craters
    let moon_x = area.width * 3 / 4;
    let moon_y = 2u16;

    if moon_x + 4 < area.width && moon_y + 3 < area.height {
        // Moon main body (crescent)
        let moon_art = [
            "  ◜█",
            " ◜██",
            "  ◜█",
        ];

        for (dy, line) in moon_art.iter().enumerate() {
            for (dx, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    let px = moon_x + dx as u16;
                    let py = moon_y + dy as u16;
                    if px < area.width && py < area.height {
                        // Crater detail - darker spots
                        let is_crater = simple_hash(dx + dy * 5, 900) % 8 == 0;
                        let (r, g, b) = if is_crater {
                            (180, 180, 160)
                        } else {
                            (230, 230, 210)
                        };

                        frame.render_widget(
                            Paragraph::new(ch.to_string())
                                .style(Style::default().fg(Color::Rgb(r, g, b))),
                            Rect::new(area.x + px, area.y + py, 1, 1),
                        );
                    }
                }
            }
        }

        // Moon glow
        for dy in -1i16..=4 {
            for dx in -1i16..=5 {
                let px = moon_x as i16 + dx;
                let py = moon_y as i16 + dy;
                if px >= 0 && py >= 0 && px < area.width as i16 && py < area.height as i16 {
                    let dist = ((dx - 2) * (dx - 2) + (dy - 1) * (dy - 1)) as f32;
                    if dist > 4.0 && dist < 16.0 {
                        let glow = (1.0 - dist / 16.0) * 40.0;
                        frame.render_widget(
                            Paragraph::new("·").style(Style::default().fg(Color::Rgb(
                                (glow + 15.0) as u8,
                                (glow + 15.0) as u8,
                                (glow + 20.0) as u8,
                            ))),
                            Rect::new(area.x + px as u16, area.y + py as u16, 1, 1),
                        );
                    }
                }
            }
        }
    }
}

/// Check if lightning should flash this frame
fn is_lightning_flash(frame_index: usize) -> bool {
    // Lightning every ~200-300 frames, lasting 3-5 frames
    let lightning_cycle = frame_index % 250;
    lightning_cycle < 4 || (lightning_cycle > 2 && lightning_cycle < 6 && frame_index % 500 < 250)
}

/// Render distant lightning bolt
fn render_lightning(frame: &mut Frame, area: Rect, frame_index: usize) {
    if !is_lightning_flash(frame_index) { return; }

    // Lightning bolt position varies
    let bolt_x = (simple_hash(frame_index / 250, 1100) % (area.width as usize / 2)) as u16 + area.width / 4;

    // Jagged bolt pattern
    let bolt_segments: [(i16, i16); 6] = [
        (0, 0), (1, 1), (-1, 2), (2, 3), (0, 4), (1, 5)
    ];

    for (dx, dy) in bolt_segments.iter() {
        let px = (bolt_x as i16 + dx) as u16;
        let py = dy.unsigned_abs() as u16 + 1;

        if px < area.width && py < area.height {
            frame.render_widget(
                Paragraph::new("╲").style(Style::default().fg(Color::Rgb(255, 255, 200))),
                Rect::new(area.x + px, area.y + py, 1, 1),
            );
        }
    }
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Check for lightning flash (affects entire scene lighting)
    let lightning_flash = is_lightning_flash(frame_index);

    // Dark ground base
    let ground_color = if lightning_flash {
        Color::Rgb(35, 32, 30)
    } else {
        Color::Rgb(12, 10, 8)
    };
    let bg = Block::default().style(Style::default().bg(ground_color));
    frame.render_widget(bg, area);

    // Render night sky with gradient and stars
    render_sky(frame, area, frame_index, lightning_flash);

    // Render lightning bolt (when flashing)
    render_lightning(frame, area, frame_index);

    // Render distant army on horizon
    render_distant_army(frame, area, frame_index);

    // Render trebuchet
    render_trebuchet(frame, area, frame_index);

    // Render ground fog
    render_ground_fog(frame, area, frame_index);

    // Render castle with enhanced stone and windows
    render_castle(frame, area, frame_index, lightning_flash);

    // Render smoke from chimneys
    render_smoke(frame, area, frame_index);

    // Render patrolling guards
    render_guards(frame, area, frame_index);

    // Render dragon silhouette
    render_dragon(frame, area, frame_index);

    // Render bats and owls
    render_flying_creatures(frame, area, frame_index);

    // Render torches on tower walls
    let towers = get_towers(area.width, area.height);
    for (i, tower) in towers.iter().enumerate() {
        let torch_x = tower.x + tower.width / 2;
        let torch_y = area.height.saturating_sub(tower.height) + tower.height / 2;

        if torch_x < area.width && torch_y < area.height && torch_y > 0 {
            render_torch(frame, area, torch_x, torch_y - 1, i, frame_index);
        }

        // Additional torch on opposite side of tower
        let torch_x2 = tower.x + tower.width - 2;
        if torch_x2 < area.width && torch_y < area.height && torch_y > 0 && tower.width > 6 {
            render_torch(frame, area, torch_x2, torch_y + 2, i + 20, frame_index);
        }
    }

    // Wall torches with more variety
    let wall_torches = [
        (area.width / 5, area.height - area.height / 4 - 2),
        (area.width * 2 / 5, area.height - area.height / 4 - 2),
        (area.width * 3 / 5, area.height - area.height / 4 - 2),
        (area.width * 4 / 5, area.height - area.height / 4 - 2),
    ];

    for (i, (tx, ty)) in wall_torches.iter().enumerate() {
        if *tx < area.width && *ty < area.height {
            // Check not inside a tower
            let in_tower = towers.iter().any(|t| *tx >= t.x && *tx < t.x + t.width);
            if !in_tower {
                render_torch(frame, area, *tx, *ty, i + 100, frame_index);
            }
        }
    }
}
