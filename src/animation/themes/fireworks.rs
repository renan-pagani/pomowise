use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Firework burst particle
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    color_idx: usize,
}

/// A single firework burst
struct Firework {
    center_x: f32,
    center_y: f32,
    birth_frame: usize,
    color_scheme: usize,
    num_particles: usize,
}

impl Firework {
    fn new(seed: usize, width: u16, height: u16) -> Self {
        let h1 = simple_hash(seed, 1);
        let h2 = simple_hash(seed, 2);
        let h3 = simple_hash(seed, 3);
        let h4 = simple_hash(seed, 4);

        Self {
            center_x: (h1 % (width as usize * 8 / 10) + width as usize / 10) as f32,
            center_y: (h2 % (height as usize * 6 / 10) + height as usize / 10) as f32,
            birth_frame: h3 % 150,
            color_scheme: h4 % 5,
            num_particles: (h4 % 15) + 20,
        }
    }

    fn get_particles(&self, frame_index: usize) -> Vec<Particle> {
        let age = (frame_index as i32 - self.birth_frame as i32) % 150;
        if age < 0 || age > 40 {
            return vec![];
        }

        let t = age as f32 / 10.0; // Time factor
        let mut particles = Vec::new();

        for i in 0..self.num_particles {
            // Angle for this particle
            let angle = (i as f32 / self.num_particles as f32) * 2.0 * std::f32::consts::PI;
            // Add some randomness to angle
            let h = simple_hash(self.birth_frame + i, 5);
            let angle_jitter = (h % 100) as f32 / 100.0 - 0.5;
            let angle = angle + angle_jitter * 0.3;

            // Speed varies per particle
            let speed_h = simple_hash(self.birth_frame + i, 6);
            let speed = 0.5 + (speed_h % 100) as f32 / 100.0;

            // Position with gravity
            let vx = angle.cos() * speed * 3.0;
            let vy = angle.sin() * speed * 1.5;

            let x = self.center_x + vx * t;
            let y = self.center_y + vy * t + 0.3 * t * t; // Gravity

            // Life fades over time
            let life = 1.0 - (t / 4.0);

            if life > 0.0 {
                particles.push(Particle {
                    x,
                    y,
                    vx,
                    vy,
                    life,
                    color_idx: self.color_scheme,
                });
            }
        }

        particles
    }
}

fn simple_hash(seed: usize, salt: usize) -> usize {
    let mut h = seed.wrapping_mul(2654435761);
    h ^= salt.wrapping_mul(1597334677);
    h = h.wrapping_mul(2654435761);
    h ^ (h >> 16)
}

/// Get firework color based on scheme and brightness
fn firework_color(scheme: usize, brightness: f32) -> Color {
    let b = (brightness * 255.0) as u8;
    match scheme {
        0 => Color::Rgb(b, b / 3, b / 5),          // Red-orange
        1 => Color::Rgb(b / 3, b, b / 3),          // Green
        2 => Color::Rgb(b / 3, b / 2, b),          // Blue
        3 => Color::Rgb(b, b, b / 5),              // Yellow-gold
        _ => Color::Rgb(b, b / 3, b),              // Magenta
    }
}

/// Particle character based on life
fn particle_char(life: f32) -> char {
    if life > 0.8 {
        '★'
    } else if life > 0.6 {
        '✦'
    } else if life > 0.4 {
        '✧'
    } else if life > 0.2 {
        '·'
    } else {
        '.'
    }
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Dark night sky
    let bg = Block::default().style(Style::default().bg(Color::Rgb(5, 5, 15)));
    frame.render_widget(bg, area);

    // Create multiple fireworks
    let num_fireworks = 6;

    for fw_idx in 0..num_fireworks {
        // Each firework repeats on a cycle
        let cycle_offset = fw_idx * 25;
        let firework = Firework::new(
            fw_idx * 7919 + (frame_index / 150) * 1000,
            area.width,
            area.height,
        );

        let adjusted_frame = frame_index.wrapping_add(cycle_offset);
        for particle in firework.get_particles(adjusted_frame) {
            let x = particle.x as u16;
            let y = particle.y as u16;

            if x < area.width && y < area.height {
                let color = firework_color(particle.color_idx, particle.life);
                let ch = particle_char(particle.life);

                frame.render_widget(
                    Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );

                // Trail for fast-moving particles
                if particle.life > 0.5 {
                    let trail_x = (particle.x - particle.vx * 0.3) as u16;
                    let trail_y = (particle.y - particle.vy * 0.3) as u16;
                    if trail_x < area.width && trail_y < area.height {
                        let trail_color = firework_color(particle.color_idx, particle.life * 0.5);
                        frame.render_widget(
                            Paragraph::new("·").style(Style::default().fg(trail_color)),
                            Rect::new(area.x + trail_x, area.y + trail_y, 1, 1),
                        );
                    }
                }
            }
        }
    }

    // Add some twinkling stars in background
    for i in 0..30 {
        let h1 = simple_hash(i + 5000, 1);
        let h2 = simple_hash(i + 5000, 2);
        let x = (h1 % area.width as usize) as u16;
        let y = (h2 % area.height as usize) as u16;

        let twinkle = (frame_index + i * 7) % 20 < 15;
        if twinkle && x < area.width && y < area.height {
            let brightness = 40 + (simple_hash(i, 3) % 40) as u8;
            frame.render_widget(
                Paragraph::new(".").style(Style::default().fg(Color::Rgb(
                    brightness,
                    brightness,
                    brightness + 20,
                ))),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
}
