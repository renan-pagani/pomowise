use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Matrix rain characters (katakana-inspired and symbols)
const CHARS: &[char] = &[
    'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    '@', '#', '$', '%', '&', '*', '+', '=', '<', '>',
];

/// State for a single falling column
struct Column {
    y: i32,           // Current head position
    speed: u8,        // Frames between moves (1-3)
    length: u8,       // Trail length
    char_offset: u8,  // For character variation
}

impl Column {
    fn new(x: usize, seed: usize) -> Self {
        let hash = simple_hash(x, seed);
        Self {
            y: -((hash % 20) as i32),
            speed: ((hash >> 4) % 3) as u8 + 1,
            length: ((hash >> 8) % 10) as u8 + 5,
            char_offset: (hash >> 12) as u8,
        }
    }

    fn get_char(&self, y: u16, frame_index: usize) -> char {
        let idx = (y as usize + self.char_offset as usize + frame_index / 7) % CHARS.len();
        CHARS[idx]
    }

    fn get_brightness(&self, y: i32) -> Option<u8> {
        let dist = self.y - y;
        if dist < 0 || dist >= self.length as i32 {
            None
        } else if dist == 0 {
            Some(255) // Head is brightest
        } else {
            // Fade out along trail
            let fade = 255 - (dist as u8 * (200 / self.length));
            Some(fade.max(30))
        }
    }

    fn advance(&mut self, height: u16) {
        self.y += 1;
        if self.y > height as i32 + self.length as i32 {
            self.y = -(self.length as i32);
        }
    }
}

fn simple_hash(x: usize, seed: usize) -> usize {
    let mut h = x.wrapping_mul(2654435761);
    h ^= seed;
    h = h.wrapping_mul(2654435761);
    h ^ (h >> 16)
}

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // Dark background
    let bg = Block::default().style(Style::default().bg(Color::Rgb(0, 10, 0)));
    frame.render_widget(bg, area);

    // Create columns for the width
    for x in 0..area.width {
        let mut col = Column::new(x as usize, 42);

        // Advance column based on frame and speed
        let advances = frame_index / col.speed as usize;
        for _ in 0..advances {
            col.advance(area.height);
        }
        col.y = col.y % ((area.height as i32) + col.length as i32 + 20);
        if col.y < 0 {
            col.y += (area.height as i32) + col.length as i32 + 20;
        }

        // Render this column
        for y in 0..area.height {
            let screen_y = y as i32;
            if let Some(brightness) = col.get_brightness(screen_y) {
                let ch = col.get_char(y, frame_index);
                let green = brightness;
                let color = Color::Rgb(0, green, green / 4);

                frame.render_widget(
                    Paragraph::new(ch.to_string()).style(Style::default().fg(color)),
                    Rect::new(area.x + x, area.y + y, 1, 1),
                );
            }
        }
    }
}
