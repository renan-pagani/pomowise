use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

/// Claude/Anthropic themed - An artistic visualization of AI consciousness
/// Warm orange/amber gradients, hexagonal patterns, neural networks,
/// breathing pulses, and flowing geometric shapes

// ============================================================================
// ANTHROPIC BRAND COLORS
// ============================================================================
const PRIMARY_ORANGE: (u8, u8, u8) = (217, 119, 6);     // #D97706
const SECONDARY_AMBER: (u8, u8, u8) = (245, 158, 11);   // #F59E0B
const ACCENT_GOLD: (u8, u8, u8) = (252, 211, 77);       // #FCD34D
const BG_DARK: (u8, u8, u8) = (28, 20, 16);             // #1C1410
const BG_WARM: (u8, u8, u8) = (45, 31, 26);             // #2D1F1A

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

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

/// Fast cosine using sine phase shift
fn fast_cos(x: f32) -> f32 {
    fast_sin(x + std::f32::consts::PI / 2.0)
}

/// Smooth interpolation (ease in-out)
fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Linear interpolation for colors
fn lerp_color(c1: (u8, u8, u8), c2: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    (
        (c1.0 as f32 + (c2.0 as f32 - c1.0 as f32) * t) as u8,
        (c1.1 as f32 + (c2.1 as f32 - c1.1 as f32) * t) as u8,
        (c1.2 as f32 + (c2.2 as f32 - c1.2 as f32) * t) as u8,
    )
}

// ============================================================================
// HEXAGONAL GRID - Honeycomb pattern for the background
// ============================================================================

/// Calculate hexagonal grid cell and distance from center
fn hexagon_field(x: f32, y: f32, scale: f32, frame_index: usize) -> f32 {
    let t = frame_index as f32 * 0.015;

    // Hexagonal grid coordinates
    let hx = x / scale;
    let hy = y / (scale * 0.866); // sqrt(3)/2 for hex aspect

    // Offset odd rows
    let row = hy.floor() as i32;
    let offset = if row % 2 == 0 { 0.0 } else { 0.5 };
    let hx = hx + offset;

    // Cell center
    let cx = hx.floor() + 0.5;
    let cy = hy.floor() + 0.5;

    // Distance from cell center (hexagonal approximation)
    let dx = (hx - cx).abs();
    let dy = (hy - cy).abs();
    let hex_dist = dx.max(dy * 0.577 + dx * 0.5);

    // Pulsing glow based on cell position
    let cell_phase = fast_sin(t * 0.8 + cx * 0.3 + cy * 0.5);
    let pulse = (cell_phase + 1.0) * 0.5;

    // Edge glow (brighter at hex edges)
    let edge_glow = smoothstep(0.35, 0.45, hex_dist) * (1.0 - smoothstep(0.45, 0.5, hex_dist));

    edge_glow * pulse * 0.4
}

// ============================================================================
// NEURAL NETWORK - Nodes and connections like thoughts flowing
// ============================================================================

struct NeuralNode {
    x: f32,
    y: f32,
    activation: f32,
    layer: usize,
}

fn get_neural_nodes(width: u16, height: u16, frame_index: usize) -> Vec<NeuralNode> {
    let t = frame_index as f32 * 0.02;
    let num_layers = 4;
    let nodes_per_layer = 5;
    let mut nodes = Vec::with_capacity(num_layers * nodes_per_layer);

    for layer in 0..num_layers {
        let layer_x = (layer as f32 + 1.0) / (num_layers as f32 + 1.0) * width as f32;

        for i in 0..nodes_per_layer {
            let base_y = (i as f32 + 1.0) / (nodes_per_layer as f32 + 1.0) * height as f32;

            // Subtle floating motion
            let offset_x = fast_sin(t + layer as f32 * 0.5 + i as f32 * 0.3) * 2.0;
            let offset_y = fast_cos(t * 0.7 + layer as f32 * 0.3 + i as f32 * 0.5) * 1.0;

            // Activation wave propagates through layers
            let activation_wave = fast_sin(t * 2.0 - layer as f32 * 0.8);
            let node_phase = fast_sin(t * 3.0 + i as f32 * 0.5);
            let activation = ((activation_wave + node_phase) * 0.5 + 0.5).clamp(0.0, 1.0);

            nodes.push(NeuralNode {
                x: layer_x + offset_x,
                y: base_y + offset_y,
                activation,
                layer,
            });
        }
    }
    nodes
}

/// Check if position is on a neural connection line
fn neural_connection_intensity(x: u16, y: u16, nodes: &[NeuralNode], frame_index: usize) -> f32 {
    let t = frame_index as f32 * 0.03;
    let px = x as f32;
    let py = y as f32;

    let mut max_intensity = 0.0f32;

    // Check connections between adjacent layers
    for node1 in nodes.iter() {
        for node2 in nodes.iter() {
            // Only connect adjacent layers
            if node2.layer != node1.layer + 1 {
                continue;
            }

            // Distance from point to line segment
            let dx = node2.x - node1.x;
            let dy = node2.y - node1.y;
            let len_sq = dx * dx + dy * dy;

            if len_sq < 0.001 {
                continue;
            }

            let t_param = ((px - node1.x) * dx + (py - node1.y) * dy) / len_sq;
            let t_clamped = t_param.clamp(0.0, 1.0);

            let closest_x = node1.x + t_clamped * dx;
            let closest_y = node1.y + t_clamped * dy;

            let dist = ((px - closest_x).powi(2) + (py - closest_y).powi(2)).sqrt();

            if dist < 1.5 {
                // Signal traveling along the connection
                let signal_pos = (t * 0.5 + node1.layer as f32 * 0.2) % 1.0;
                let signal_strength = 1.0 - (t_param - signal_pos).abs() * 3.0;
                let signal = signal_strength.max(0.0);

                let connection_strength = (node1.activation + node2.activation) * 0.5;
                let line_intensity = (1.0 - dist / 1.5) * connection_strength * 0.3;
                let combined = line_intensity + signal * 0.4;

                max_intensity = max_intensity.max(combined);
            }
        }
    }

    max_intensity
}

// ============================================================================
// WARM GRADIENT WAVES - Flowing orange and amber
// ============================================================================

fn gradient_wave(x: u16, y: u16, width: u16, height: u16, frame_index: usize) -> f32 {
    let t = frame_index as f32 * 0.02;
    let fx = x as f32 / width as f32;
    let fy = y as f32 / height as f32;

    // Multiple overlapping waves
    let wave1 = fast_sin(fx * 3.0 + fy * 2.0 + t);
    let wave2 = fast_sin(fx * 5.0 - fy * 3.0 + t * 1.3);
    let wave3 = fast_sin((fx - 0.5).powi(2) * 8.0 + (fy - 0.5).powi(2) * 8.0 - t * 0.7);

    let combined = (wave1 * 0.4 + wave2 * 0.3 + wave3 * 0.3 + 1.0) / 2.0;
    combined.clamp(0.0, 1.0)
}

// ============================================================================
// FLOATING GEOMETRIC SHAPES - Circles, triangles, hexagons drifting slowly
// ============================================================================

#[derive(Clone, Copy)]
enum ShapeType {
    Circle,
    Triangle,
    Hexagon,
    Diamond,
}

struct FloatingShape {
    x: f32,
    y: f32,
    size: f32,
    rotation: f32,
    shape_type: ShapeType,
    brightness: f32,
}

fn get_floating_shapes(width: u16, height: u16, frame_index: usize, count: usize) -> Vec<FloatingShape> {
    let t = frame_index as f32 * 0.008;
    let mut shapes = Vec::with_capacity(count);

    for i in 0..count {
        let seed = i * 17 + 42;
        let base_x = (simple_hash(seed, 100) % (width as usize * 100)) as f32 / 100.0;
        let base_y = (simple_hash(seed, 200) % (height as usize * 100)) as f32 / 100.0;

        // Slow drifting motion
        let drift_x = fast_sin(t * 0.3 + i as f32 * 0.7) * 4.0;
        let drift_y = fast_cos(t * 0.25 + i as f32 * 0.5) * 3.0 + t * 0.3;

        let x = (base_x + drift_x).rem_euclid(width as f32);
        let y = (base_y + drift_y).rem_euclid(height as f32);

        let size = 1.5 + (simple_hash(seed, 300) % 30) as f32 / 10.0;
        let rotation = t * 0.5 + i as f32 * 1.2;

        let shape_type = match simple_hash(seed, 400) % 4 {
            0 => ShapeType::Circle,
            1 => ShapeType::Triangle,
            2 => ShapeType::Hexagon,
            _ => ShapeType::Diamond,
        };

        // Pulsing brightness
        let brightness = 0.3 + (fast_sin(t * 1.5 + i as f32 * 0.8) + 1.0) * 0.35;

        shapes.push(FloatingShape {
            x,
            y,
            size,
            rotation,
            shape_type,
            brightness,
        });
    }
    shapes
}

fn shape_distance(px: f32, py: f32, shape: &FloatingShape) -> f32 {
    let dx = px - shape.x;
    let dy = (py - shape.y) * 2.0; // Terminal aspect ratio

    // Rotate point
    let cos_r = fast_cos(shape.rotation);
    let sin_r = fast_sin(shape.rotation);
    let rx = dx * cos_r - dy * sin_r;
    let ry = dx * sin_r + dy * cos_r;

    match shape.shape_type {
        ShapeType::Circle => {
            (rx * rx + ry * ry).sqrt() - shape.size
        }
        ShapeType::Triangle => {
            let s = shape.size;
            let k = 3.0f32.sqrt();
            let rx = rx.abs();
            let d = rx - s.min(ry * k + s);
            d.max(-ry - s * 0.5)
        }
        ShapeType::Hexagon => {
            let s = shape.size;
            let rx = rx.abs();
            let ry = ry.abs();
            let k = 3.0f32.sqrt() / 2.0;
            (rx - s).max((rx * 0.5 + ry * k - s).max(ry * k - s * k))
        }
        ShapeType::Diamond => {
            let s = shape.size;
            (rx.abs() + ry.abs()) / 1.414 - s
        }
    }
}

// ============================================================================
// THINKING PULSE - Central glow that breathes
// ============================================================================

fn thinking_pulse(x: u16, y: u16, width: u16, height: u16, frame_index: usize) -> f32 {
    let t = frame_index as f32 * 0.03;

    // Center of the screen
    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;

    let dx = x as f32 - cx;
    let dy = (y as f32 - cy) * 2.0; // Terminal aspect
    let dist = (dx * dx + dy * dy).sqrt();

    // Breathing rhythm (slower, calmer)
    let breath = (fast_sin(t * 0.8) + 1.0) / 2.0;
    let breath_size = 8.0 + breath * 12.0;

    // Smooth falloff from center
    let intensity = smoothstep(breath_size + 5.0, breath_size * 0.3, dist);

    // Add subtle rings
    let rings = fast_sin(dist * 0.5 - t * 2.0) * 0.15;

    (intensity + rings * intensity).clamp(0.0, 1.0)
}

// ============================================================================
// PARTICLE TRAILS - Particles with fading trails
// ============================================================================

struct TrailParticle {
    x: f32,
    y: f32,
    trail_x: [f32; 4],
    trail_y: [f32; 4],
    brightness: f32,
}

fn get_trail_particles(width: u16, height: u16, frame_index: usize, count: usize) -> Vec<TrailParticle> {
    let t = frame_index as f32 * 0.025;
    let mut particles = Vec::with_capacity(count);

    for i in 0..count {
        let seed = i * 31 + 77;
        let base_x = (simple_hash(seed, 100) % (width as usize)) as f32;
        let base_y = (simple_hash(seed, 200) % (height as usize)) as f32;

        let speed_x = (simple_hash(seed, 300) % 100) as f32 / 50.0 - 1.0;
        let speed_y = (simple_hash(seed, 400) % 100) as f32 / 100.0 - 0.5;

        let x = (base_x + t * speed_x * 3.0).rem_euclid(width as f32);
        let y = (base_y + t * speed_y * 2.0).rem_euclid(height as f32);

        // Trail positions (history)
        let trail_x = [
            (x - speed_x * 0.5).rem_euclid(width as f32),
            (x - speed_x * 1.0).rem_euclid(width as f32),
            (x - speed_x * 1.5).rem_euclid(width as f32),
            (x - speed_x * 2.0).rem_euclid(width as f32),
        ];
        let trail_y = [
            (y - speed_y * 0.5).rem_euclid(height as f32),
            (y - speed_y * 1.0).rem_euclid(height as f32),
            (y - speed_y * 1.5).rem_euclid(height as f32),
            (y - speed_y * 2.0).rem_euclid(height as f32),
        ];

        let brightness = 0.5 + (fast_sin(t * 2.0 + i as f32 * 0.7) + 1.0) * 0.25;

        particles.push(TrailParticle {
            x,
            y,
            trail_x,
            trail_y,
            brightness,
        });
    }
    particles
}

// ============================================================================
// CONSTELLATION PATTERNS - Dots that connect briefly
// ============================================================================

fn constellation_pattern(x: u16, y: u16, width: u16, height: u16, frame_index: usize) -> f32 {
    let t = frame_index as f32 * 0.02;
    let num_stars = 12;
    let px = x as f32;
    let py = y as f32;

    // Generate constellation stars
    let mut stars = Vec::with_capacity(num_stars);
    for i in 0..num_stars {
        let seed = i * 23 + 111;
        let star_x = (simple_hash(seed, 100) % width as usize) as f32;
        let star_y = (simple_hash(seed, 200) % height as usize) as f32;

        // Slow drift
        let drift_x = fast_sin(t * 0.2 + i as f32 * 0.5) * 3.0;
        let drift_y = fast_cos(t * 0.15 + i as f32 * 0.3) * 2.0;

        stars.push((star_x + drift_x, star_y + drift_y));
    }

    // Check if on a connection line (connections appear and fade)
    let connection_phase = (t * 0.3) % 1.0;
    let num_connections = ((connection_phase * 6.0) as usize).min(5);

    let mut intensity = 0.0f32;

    for conn_idx in 0..num_connections {
        let i1 = simple_hash(conn_idx + frame_index / 60, 500) % num_stars;
        let i2 = simple_hash(conn_idx + frame_index / 60, 600) % num_stars;

        if i1 == i2 { continue; }

        let (x1, y1) = stars[i1];
        let (x2, y2) = stars[i2];

        // Distance from point to line
        let dx = x2 - x1;
        let dy = y2 - y1;
        let len_sq = dx * dx + dy * dy;

        if len_sq < 1.0 { continue; }

        let t_param = ((px - x1) * dx + (py - y1) * dy) / len_sq;
        if t_param < 0.0 || t_param > 1.0 { continue; }

        let closest_x = x1 + t_param * dx;
        let closest_y = y1 + t_param * dy;
        let dist = ((px - closest_x).powi(2) + (py - closest_y).powi(2)).sqrt();

        if dist < 1.0 {
            // Fade based on connection age
            let age = (conn_idx as f32 + connection_phase) / 6.0;
            let fade = 1.0 - age;
            intensity = intensity.max((1.0 - dist) * fade * 0.3);
        }
    }

    // Check if on a star
    for (sx, sy) in &stars {
        let dist = ((px - sx).powi(2) + (py - sy).powi(2) * 4.0).sqrt();
        if dist < 1.5 {
            intensity = intensity.max(1.0 - dist / 1.5);
        }
    }

    intensity
}

// ============================================================================
// BACKGROUND GRADIENT - Warm dark brown gradient
// ============================================================================

fn background_color(x: u16, y: u16, width: u16, height: u16, frame_index: usize) -> Color {
    let t = frame_index as f32 * 0.01;
    let fx = x as f32 / width as f32;
    let fy = y as f32 / height as f32;

    // Radial gradient from center
    let cx = 0.5;
    let cy = 0.45;
    let dist = ((fx - cx).powi(2) + (fy - cy).powi(2)).sqrt();

    // Subtle wave in the gradient
    let wave = fast_sin(fx * 2.5 + fy * 1.5 + t) * 0.08;

    // Gradient factor (brighter at center)
    let gradient = (1.0 - dist * 0.7).clamp(0.0, 1.0) + wave;

    // Interpolate between dark and warm brown
    let base = lerp_color(BG_DARK, BG_WARM, gradient * 0.6);

    Color::Rgb(base.0, base.1, base.2)
}

// ============================================================================
// COLOR SELECTION - Get warm orange/amber/gold colors
// ============================================================================

fn get_accent_color(intensity: f32, variant: usize, frame_index: usize) -> Color {
    let t = frame_index as f32 * 0.01;
    let phase = (t + variant as f32 * 0.5) % 3.0;

    let base = if phase < 1.0 {
        PRIMARY_ORANGE
    } else if phase < 2.0 {
        SECONDARY_AMBER
    } else {
        ACCENT_GOLD
    };

    let r = (base.0 as f32 * intensity) as u8;
    let g = (base.1 as f32 * intensity) as u8;
    let b = (base.2 as f32 * intensity) as u8;

    Color::Rgb(r, g, b)
}

fn get_glow_color(intensity: f32) -> Color {
    // Warm glow - mix of orange and amber
    let i = intensity.clamp(0.0, 1.0);
    let r = (217.0 * i + 30.0 * (1.0 - i)) as u8;
    let g = (140.0 * i + 20.0 * (1.0 - i)) as u8;
    let b = (10.0 * i + 10.0 * (1.0 - i)) as u8;
    Color::Rgb(r, g, b)
}

// ============================================================================
// CHARACTER SELECTION
// ============================================================================

fn intensity_char(intensity: f32, variant: usize) -> char {
    if intensity > 0.85 {
        match variant % 4 {
            0 => '◉',
            1 => '●',
            2 => '◆',
            _ => '★',
        }
    } else if intensity > 0.65 {
        match variant % 3 {
            0 => '○',
            1 => '◇',
            _ => '⬡',
        }
    } else if intensity > 0.45 {
        '▓'
    } else if intensity > 0.30 {
        '▒'
    } else if intensity > 0.18 {
        '░'
    } else if intensity > 0.08 {
        '·'
    } else {
        ' '
    }
}

fn particle_char(brightness: f32) -> char {
    if brightness > 0.7 {
        '•'
    } else if brightness > 0.4 {
        '·'
    } else {
        '.'
    }
}

fn trail_char(age: usize) -> char {
    match age {
        0 => '●',
        1 => '◉',
        2 => '○',
        3 => '·',
        _ => '.',
    }
}

// ============================================================================
// MAIN RENDER FUNCTION
// ============================================================================

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // First pass: render background gradient
    for y in 0..area.height {
        for x in 0..area.width {
            let color = background_color(x, y, area.width, area.height, frame_index);
            frame.render_widget(
                Paragraph::new(" ").style(Style::default().bg(color)),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }

    // Get pre-computed data for neural network and shapes
    let neural_nodes = get_neural_nodes(area.width, area.height, frame_index);
    let floating_shapes = get_floating_shapes(area.width, area.height, frame_index, 12);
    let trail_particles = get_trail_particles(area.width, area.height, frame_index, 20);

    // Second pass: render all effects
    for y in 0..area.height {
        for x in 0..area.width {
            let mut total_intensity = 0.0f32;
            let mut effect_type = 0usize;

            // 1. Hexagonal grid (subtle background pattern)
            let hex_intensity = hexagon_field(x as f32, y as f32, 8.0, frame_index);
            if hex_intensity > 0.05 {
                total_intensity = total_intensity.max(hex_intensity);
                effect_type = 1;
            }

            // 2. Gradient waves
            let wave = gradient_wave(x, y, area.width, area.height, frame_index);
            let wave_intensity = (wave - 0.3).max(0.0) * 0.3;
            if wave_intensity > total_intensity {
                total_intensity = wave_intensity;
                effect_type = 2;
            }

            // 3. Neural network connections
            let neural = neural_connection_intensity(x, y, &neural_nodes, frame_index);
            if neural > total_intensity {
                total_intensity = neural;
                effect_type = 3;
            }

            // 4. Floating shapes (check each shape)
            for (i, shape) in floating_shapes.iter().enumerate() {
                let dist = shape_distance(x as f32, y as f32, shape);
                if dist < 0.5 {
                    // On the edge of the shape
                    let edge_intensity = (0.5 - dist.abs()) * 2.0 * shape.brightness;
                    if edge_intensity > total_intensity {
                        total_intensity = edge_intensity;
                        effect_type = 4 + (i % 4);
                    }
                }
            }

            // 5. Thinking pulse (central breathing glow)
            let pulse = thinking_pulse(x, y, area.width, area.height, frame_index);
            if pulse > 0.1 && pulse > total_intensity * 0.5 {
                total_intensity = total_intensity.max(pulse * 0.6);
                effect_type = 8;
            }

            // 6. Constellation patterns
            let constellation = constellation_pattern(x, y, area.width, area.height, frame_index);
            if constellation > total_intensity {
                total_intensity = constellation;
                effect_type = 9;
            }

            // Render if there's something to show
            if total_intensity > 0.05 {
                let ch = intensity_char(total_intensity, effect_type);
                let color = get_accent_color(total_intensity, effect_type, frame_index);

                frame.render_widget(
                    Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }

    // Third pass: render neural network nodes (on top)
    for (i, node) in neural_nodes.iter().enumerate() {
        let nx = node.x as u16;
        let ny = node.y as u16;

        if nx < area.width && ny < area.height {
            let brightness = 0.4 + node.activation * 0.6;
            let color = get_accent_color(brightness, i + node.layer * 5, frame_index);
            let ch = if node.activation > 0.7 { '◉' } else if node.activation > 0.4 { '●' } else { '○' };

            frame.render_widget(
                Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                Rect::new(area.x + nx, area.y + ny, 1, 1),
            );
        }
    }

    // Fourth pass: render particle trails
    for (i, particle) in trail_particles.iter().enumerate() {
        // Render trail first (behind particle)
        for (age, (&tx, &ty)) in particle.trail_x.iter().zip(particle.trail_y.iter()).enumerate() {
            let px = tx as u16;
            let py = ty as u16;

            if px < area.width && py < area.height {
                let trail_brightness = particle.brightness * (1.0 - age as f32 * 0.25);
                let color = get_glow_color(trail_brightness * 0.5);
                let ch = trail_char(age + 1);

                frame.render_widget(
                    Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                    Rect::new(area.x + px, area.y + py, 1, 1),
                );
            }
        }

        // Render main particle
        let px = particle.x as u16;
        let py = particle.y as u16;

        if px < area.width && py < area.height {
            let color = get_accent_color(particle.brightness, i, frame_index);
            let ch = particle_char(particle.brightness);

            frame.render_widget(
                Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                Rect::new(area.x + px, area.y + py, 1, 1),
            );
        }
    }

    // Fifth pass: render the central "thinking" orb
    let cx = area.width / 2;
    let cy = area.height / 2;
    let t = frame_index as f32 * 0.03;
    let breath = (fast_sin(t * 0.8) + 1.0) / 2.0;

    // Render a soft glow around center
    for dy in 0..5u16 {
        for dx in 0..9u16 {
            let nx = cx.saturating_sub(4) + dx;
            let ny = cy.saturating_sub(2) + dy;

            if nx < area.width && ny < area.height {
                let dist_x = (dx as f32 - 4.0).abs();
                let dist_y = (dy as f32 - 2.0).abs() * 2.0;
                let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();

                let glow_size = 3.0 + breath * 2.0;
                if dist < glow_size {
                    let intensity = (1.0 - dist / glow_size) * (0.6 + breath * 0.4);
                    let color = get_glow_color(intensity);
                    let ch = if dist < 1.5 { '◉' } else if dist < 2.5 { '○' } else { '·' };

                    frame.render_widget(
                        Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                        Rect::new(area.x + nx, area.y + ny, 1, 1),
                    );
                }
            }
        }
    }
}
