use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// GitHub themed - Developer productivity visualization
/// Code flowing, commits happening, branches merging
/// A living codebase in real-time

// GitHub Dark color palette
const BG_COLOR: Color = Color::Rgb(13, 17, 23);           // #0D1117
const CONTRIB_0: Color = Color::Rgb(22, 27, 34);          // Empty cell
const CONTRIB_1: Color = Color::Rgb(14, 68, 41);          // #0E4429
const CONTRIB_2: Color = Color::Rgb(0, 109, 50);          // #006D32
const CONTRIB_3: Color = Color::Rgb(38, 166, 65);         // #26A641
const CONTRIB_4: Color = Color::Rgb(57, 211, 83);         // #39D353
const ACCENT_BLUE: Color = Color::Rgb(88, 166, 255);      // #58A6FF
const TEXT_GRAY: Color = Color::Rgb(139, 148, 158);       // #8B949E
const DIM_GRAY: Color = Color::Rgb(48, 54, 61);           // Border gray
const MERGE_FLASH: Color = Color::Rgb(163, 113, 247);     // Purple for merges

/// Code rain characters - actual programming symbols
const CODE_CHARS: &[char] = &[
    '{', '}', '(', ')', '[', ']', '<', '>',
    '=', '>', ';', ':', ',', '.', '|', '&',
    '+', '-', '*', '/', '%', '!', '?', '@',
];

/// Commit message prefixes
const COMMIT_PREFIXES: &[&str] = &[
    "fix:", "feat:", "docs:", "test:", "chore:",
    "refactor:", "style:", "perf:", "ci:", "build:",
];

/// Simple hash function for deterministic randomness
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

/// Fast cosine approximation
fn fast_cos(x: f32) -> f32 {
    fast_sin(x + std::f32::consts::PI / 2.0)
}

/// Contribution level to color with growth animation
fn contribution_color(level: u8, growth_phase: f32) -> Color {
    let base: (u8, u8, u8) = match level {
        0 => (22, 27, 34),
        1 => (14, 68, 41),
        2 => (0, 109, 50),
        3 => (38, 166, 65),
        _ => (57, 211, 83),
    };

    // Add subtle glow during growth
    if growth_phase > 0.0 && level > 0 {
        let boost = (growth_phase * 30.0) as u8;
        Color::Rgb(
            base.0.saturating_add(boost / 3),
            base.1.saturating_add(boost),
            base.2.saturating_add(boost / 2),
        )
    } else {
        Color::Rgb(base.0, base.1, base.2)
    }
}

/// Octocat ASCII art frames (simplified for TUI)
const OCTOCAT_NORMAL: &[&str] = &[
    "  .--.  ",
    " /    \\ ",
    "| o  o |",
    "|  <>  |",
    " \\    / ",
    "  '--'  ",
    "  /||\\  ",
    " / || \\ ",
];

const OCTOCAT_WAVE: &[&str] = &[
    "  .--.  ",
    " /    \\ ",
    "| o  o |",
    "|  <>  |",
    " \\    / ",
    "  '--'  ",
    " /||\\ ~ ",
    "/ || \\/ ",
];

/// Render the Octocat in a corner
fn render_octocat(frame: &mut Frame, area: Rect, frame_index: usize) {
    if area.width < 15 || area.height < 12 {
        return;
    }

    // Determine if waving (wave every ~3 seconds for ~0.5 seconds)
    let wave_cycle = frame_index % 180;
    let is_waving = wave_cycle > 150 && wave_cycle < 170;

    let octocat = if is_waving { OCTOCAT_WAVE } else { OCTOCAT_NORMAL };

    // Position in bottom-right corner
    let start_x = area.x + area.width.saturating_sub(10);
    let start_y = area.y + area.height.saturating_sub(10);

    for (i, line) in octocat.iter().enumerate() {
        let y = start_y + i as u16;
        if y < area.y + area.height {
            // Subtle color variation based on animation
            let brightness = if is_waving { 180 } else { 140 };
            let color = Color::Rgb(brightness, brightness, brightness);

            frame.render_widget(
                Paragraph::new(*line).style(Style::default().fg(color)),
                Rect::new(start_x, y, 8, 1),
            );
        }
    }
}

/// Branch structure for visualization
struct Branch {
    start_x: f32,
    end_x: f32,
    y: f32,
    curve_amplitude: f32,
    color: Color,
    is_main: bool,
}

/// Render git branch lines with smooth curves
fn render_branch_lines(frame: &mut Frame, area: Rect, frame_index: usize) {
    if area.width < 20 || area.height < 10 {
        return;
    }

    let t = frame_index as f32 * 0.02;

    // Main branch - horizontal line that curves slightly
    let main_y = area.height / 2;
    for x in 0..area.width {
        let wave = (fast_sin(x as f32 * 0.1 + t) * 1.5) as i16;
        let y = (main_y as i16 + wave).max(0) as u16;

        if y < area.height {
            let brightness = 100 + (fast_sin(x as f32 * 0.05 + t * 2.0) * 30.0) as u8;
            let color = Color::Rgb(0, brightness, 0);
            frame.render_widget(
                Paragraph::new("─").style(Style::default().fg(color)),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }

    // Feature branches that fork off
    for branch_idx in 0..4 {
        let branch_seed = simple_hash(branch_idx, 1234);
        let start_x = (branch_seed % (area.width as usize / 2)) as u16 + 5;
        let fork_direction: i16 = if branch_idx % 2 == 0 { -1 } else { 1 };

        // Animate branch growth
        let branch_phase = ((frame_index + branch_idx * 50) % 300) as f32 / 300.0;
        let branch_length = (branch_phase * 20.0).min(15.0) as u16;

        for i in 0..branch_length {
            let progress = i as f32 / branch_length as f32;
            let curve = (progress * std::f32::consts::PI * 0.5).sin();

            let bx = start_x + i;
            let offset = (curve * 4.0 * fork_direction as f32) as i16;
            let base_wave = (fast_sin(start_x as f32 * 0.1 + t) * 1.5) as i16;
            let by = ((main_y as i16) + base_wave + offset).max(0) as u16;

            if bx < area.width && by < area.height {
                let fade = 1.0 - progress * 0.5;
                let brightness = (fade * 80.0) as u8 + 40;

                // Branch color based on type
                let color = if branch_idx == 0 {
                    ACCENT_BLUE
                } else {
                    Color::Rgb(brightness, brightness + 20, brightness)
                };

                let ch = if i == branch_length - 1 { "●" } else { "─" };
                frame.render_widget(
                    Paragraph::new(ch).style(Style::default().fg(color)),
                    Rect::new(area.x + bx, area.y + by, 1, 1),
                );
            }
        }

        // Merge point animation
        if branch_phase > 0.9 {
            let merge_intensity = ((branch_phase - 0.9) * 10.0) as u8;
            let flash_color = Color::Rgb(
                100 + merge_intensity * 10,
                50 + merge_intensity * 5,
                150 + merge_intensity * 10,
            );

            let mx = start_x + branch_length;
            let base_wave = (fast_sin(start_x as f32 * 0.1 + t) * 1.5) as i16;
            let my = ((main_y as i16) + base_wave).max(0) as u16;

            if mx < area.width && my < area.height {
                frame.render_widget(
                    Paragraph::new("*").style(Style::default().fg(flash_color)),
                    Rect::new(area.x + mx, area.y + my, 1, 1),
                );
            }
        }
    }
}

/// Render the contribution grid with growth animations
fn render_contribution_grid(frame: &mut Frame, area: Rect, frame_index: usize) {
    let cell_width = 2u16;
    let cell_height = 1u16;
    let gap = 1u16;

    // Grid dimensions
    let max_cols = 52u16; // One year of weeks
    let max_rows = 7u16;  // Days of week

    let grid_cols = (area.width / (cell_width + gap)).min(max_cols);
    let grid_rows = (area.height / (cell_height + gap)).min(max_rows);

    // Center the grid in upper portion
    let grid_width = grid_cols * (cell_width + gap);
    let grid_height = grid_rows * (cell_height + gap);
    let offset_x = (area.width.saturating_sub(grid_width)) / 2;
    let offset_y = 3; // Slight offset from top

    for gy in 0..grid_rows {
        for gx in 0..grid_cols {
            let cell_id = gx as usize * 100 + gy as usize;
            let base_level = simple_hash(cell_id, 1234) % 5;

            // Growth animation - cells occasionally "grow"
            let growth_cycle = simple_hash(cell_id, 5678) % 200;
            let current_cycle = frame_index % 200;
            let is_growing = current_cycle >= growth_cycle && current_cycle < growth_cycle + 20;
            let growth_phase = if is_growing {
                let phase = (current_cycle - growth_cycle) as f32 / 20.0;
                fast_sin(phase * std::f32::consts::PI)
            } else {
                0.0
            };

            // Temporarily boost level during growth
            let level = if is_growing && base_level < 4 {
                (base_level + 1) as u8
            } else {
                base_level as u8
            };

            let color = contribution_color(level, growth_phase);

            let px = area.x + offset_x + gx * (cell_width + gap);
            let py = area.y + offset_y + gy * (cell_height + gap);

            // Render cell with rounded appearance
            for dy in 0..cell_height {
                for dx in 0..cell_width {
                    if px + dx < area.x + area.width && py + dy < area.y + area.height {
                        frame.render_widget(
                            Paragraph::new("█").style(Style::default().fg(color)),
                            Rect::new(px + dx, py + dy, 1, 1),
                        );
                    }
                }
            }
        }
    }
}

/// Render file tree pattern on the left side
fn render_file_tree(frame: &mut Frame, area: Rect, frame_index: usize) {
    if area.width < 30 || area.height < 15 {
        return;
    }

    let tree_lines = [
        "  src/",
        "  ├─ lib.rs",
        "  ├─ main.rs",
        "  ├─ utils/",
        "  │  ├─ mod.rs",
        "  │  └─ helpers.rs",
        "  └─ api/",
        "     ├─ mod.rs",
        "     └─ routes.rs",
    ];

    let start_y = area.height.saturating_sub(12);
    let pulse = (frame_index % 120) as f32 / 120.0;

    for (i, line) in tree_lines.iter().enumerate() {
        let y = area.y + start_y + i as u16;
        if y < area.y + area.height {
            // Subtle highlight animation on random files
            let highlight = simple_hash(i + frame_index / 60, 9999) % 10 == 0;
            let base_brightness = if highlight { 100 } else { 50 };
            let brightness = base_brightness + (pulse * 20.0) as u8;

            let color = Color::Rgb(brightness, brightness + 10, brightness);
            frame.render_widget(
                Paragraph::new(*line).style(Style::default().fg(color)),
                Rect::new(area.x + 1, y, line.len() as u16, 1),
            );
        }
    }
}

/// Render scrolling commit messages in background
fn render_commit_messages(frame: &mut Frame, area: Rect, frame_index: usize) {
    if area.width < 40 {
        return;
    }

    for lane in 0..3 {
        let lane_x = area.width / 3 * lane + 10;
        let scroll_speed = 1 + lane as usize;
        let y_offset = (frame_index * scroll_speed / 3) % (area.height as usize * 2);

        for msg_idx in 0..3 {
            let y_pos = (y_offset + msg_idx * 8) % (area.height as usize * 2);
            if y_pos >= area.height as usize {
                continue;
            }

            let prefix_idx = simple_hash(lane as usize * 10 + msg_idx + frame_index / 200, 7777)
                % COMMIT_PREFIXES.len();
            let prefix = COMMIT_PREFIXES[prefix_idx];

            // Very dim commit messages
            let brightness = 35 + (simple_hash(msg_idx + lane as usize, 8888) % 15) as u8;
            let color = Color::Rgb(brightness, brightness + 5, brightness);

            if lane_x < area.width {
                frame.render_widget(
                    Paragraph::new(prefix).style(Style::default().fg(color)),
                    Rect::new(area.x + lane_x, area.y + y_pos as u16, prefix.len() as u16, 1),
                );
            }
        }
    }
}

/// Render activity line graph
fn render_activity_graph(frame: &mut Frame, area: Rect, frame_index: usize) {
    if area.width < 30 || area.height < 8 {
        return;
    }

    let graph_width = 25u16;
    let graph_height = 5u16;
    let start_x = area.x + 3;
    let start_y = area.y + area.height - graph_height - 2;

    let t = frame_index as f32 * 0.05;

    // Draw graph background
    for x in 0..graph_width {
        frame.render_widget(
            Paragraph::new("·").style(Style::default().fg(DIM_GRAY)),
            Rect::new(start_x + x, start_y + graph_height - 1, 1, 1),
        );
    }

    // Draw activity line
    let mut prev_y: Option<u16> = None;
    for x in 0..graph_width {
        let wave1 = fast_sin(x as f32 * 0.3 + t);
        let wave2 = fast_sin(x as f32 * 0.5 + t * 1.3) * 0.5;
        let wave3 = fast_sin(x as f32 * 0.15 + t * 0.7) * 0.3;

        let combined = (wave1 + wave2 + wave3) * 0.5 + 0.5;
        let y_offset = ((1.0 - combined) * (graph_height - 1) as f32) as u16;
        let y = start_y + y_offset;

        // Connect points with lines
        if let Some(py) = prev_y {
            let steps = (py as i16 - y as i16).abs() as u16;
            if steps > 1 {
                let dir = if py > y { -1i16 } else { 1i16 };
                for step in 1..steps {
                    let intermediate_y = (py as i16 + dir * step as i16) as u16;
                    if intermediate_y >= start_y && intermediate_y < start_y + graph_height {
                        frame.render_widget(
                            Paragraph::new("│").style(Style::default().fg(CONTRIB_3)),
                            Rect::new(start_x + x - 1, intermediate_y, 1, 1),
                        );
                    }
                }
            }
        }

        // Draw point
        if y >= start_y && y < start_y + graph_height {
            let intensity = combined;
            let color = if intensity > 0.7 {
                CONTRIB_4
            } else if intensity > 0.4 {
                CONTRIB_3
            } else {
                CONTRIB_2
            };

            frame.render_widget(
                Paragraph::new("●").style(Style::default().fg(color)),
                Rect::new(start_x + x, y, 1, 1),
            );
        }

        prev_y = Some(y);
    }
}

/// Render code rain (Matrix-style but with code symbols)
fn render_code_rain(frame: &mut Frame, area: Rect, frame_index: usize) {
    let num_streams = (area.width / 6).min(15);

    for stream in 0..num_streams {
        let stream_seed = simple_hash(stream as usize, 3333);
        let x = (stream_seed % area.width as usize) as u16;
        let speed = 1 + (stream_seed >> 4) % 3;
        let length = 4 + (stream_seed >> 8) % 6;

        let y_offset = (frame_index * speed) % ((area.height as usize + length) * 2);

        for i in 0..length {
            let y = y_offset as i32 - i as i32;
            if y >= 0 && y < area.height as i32 {
                let char_idx = simple_hash(stream as usize + i + frame_index / 10, 4444)
                    % CODE_CHARS.len();
                let ch = CODE_CHARS[char_idx];

                // Fade based on position in trail
                let fade = 1.0 - (i as f32 / length as f32);
                let brightness = (fade * 60.0) as u8 + 20;

                // Green tint for code
                let color = Color::Rgb(brightness / 3, brightness, brightness / 2);

                frame.render_widget(
                    Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                    Rect::new(area.x + x, area.y + y as u16, 1, 1),
                );
            }
        }
    }
}

/// Render PR merge flash effects
fn render_merge_effects(frame: &mut Frame, area: Rect, frame_index: usize) {
    for effect_idx in 0..3 {
        let effect_period = 150 + simple_hash(effect_idx, 5555) % 100;
        let effect_frame = frame_index % effect_period;

        if effect_frame < 25 {
            let cx = (simple_hash(effect_idx + frame_index / effect_period, 6666)
                % (area.width as usize - 4)) as u16 + 2;
            let cy = (simple_hash(effect_idx + frame_index / effect_period, 7777)
                % (area.height as usize - 2)) as u16 + 1;

            let intensity = 1.0 - effect_frame as f32 / 25.0;
            let radius = (effect_frame as f32 * 0.3) as u16;

            // Draw expanding ring
            for angle in 0..8 {
                let a = angle as f32 * std::f32::consts::PI / 4.0;
                let dx = (fast_cos(a) * radius as f32) as i16;
                let dy = (fast_sin(a) * radius as f32 * 0.5) as i16; // Flatten for terminal

                let px = (cx as i16 + dx).max(0) as u16;
                let py = (cy as i16 + dy).max(0) as u16;

                if px < area.width && py < area.height {
                    let brightness = (intensity * 200.0) as u8 + 55;
                    let color = Color::Rgb(brightness / 2, brightness / 3, brightness);

                    frame.render_widget(
                        Paragraph::new("*").style(Style::default().fg(color)),
                        Rect::new(area.x + px, area.y + py, 1, 1),
                    );
                }
            }

            // Center flash
            if effect_frame < 10 {
                let center_brightness = ((1.0 - effect_frame as f32 / 10.0) * 255.0) as u8;
                frame.render_widget(
                    Paragraph::new("◆").style(Style::default().fg(
                        Color::Rgb(center_brightness, center_brightness, center_brightness)
                    )),
                    Rect::new(area.x + cx, area.y + cy, 1, 1),
                );
            }
        }
    }
}

/// Main render function
pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Dark GitHub background
    let bg = Block::default().style(Style::default().bg(BG_COLOR));
    frame.render_widget(bg, area);

    // Layer 1: Very subtle code rain in background
    render_code_rain(frame, area, frame_index);

    // Layer 2: Scrolling commit messages (very dim)
    render_commit_messages(frame, area, frame_index);

    // Layer 3: Branch visualization
    render_branch_lines(frame, area, frame_index);

    // Layer 4: Contribution grid (central element)
    render_contribution_grid(frame, area, frame_index);

    // Layer 5: File tree on left side
    render_file_tree(frame, area, frame_index);

    // Layer 6: Activity graph in bottom left
    render_activity_graph(frame, area, frame_index);

    // Layer 7: Octocat in corner
    render_octocat(frame, area, frame_index);

    // Layer 8: Merge flash effects (on top)
    render_merge_effects(frame, area, frame_index);

    // Corner decoration - repo indicator
    if area.width > 20 && area.height > 3 {
        let repo_text = " main ";
        frame.render_widget(
            Paragraph::new(repo_text).style(
                Style::default()
                    .fg(Color::Rgb(200, 200, 200))
                    .bg(Color::Rgb(35, 134, 54))
            ),
            Rect::new(area.x + 2, area.y + 1, repo_text.len() as u16, 1),
        );
    }
}
