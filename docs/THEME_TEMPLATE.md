# Theme Creation Template

This document explains how to create a new theme for pomowise.

## File Location

Create your theme at: `src/animation/themes/{theme_name}.rs`

## Required Function

Every theme must export this function:

```rust
pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize)
```

**Parameters:**
- `frame` - Ratatui Frame to render to
- `area` - The full terminal Rect to fill
- `frame_index` - Animation frame counter (increments ~10x/sec)

## Template Structure

```rust
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Brief description of your theme

// Optional: simple hash function for randomness
fn simple_hash(x: usize, seed: usize) -> usize {
    let mut h = x.wrapping_mul(2654435761);
    h ^= seed;
    h = h.wrapping_mul(2654435761);
    h ^ (h >> 16)
}

// Optional: fast sine approximation for wave effects
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

pub fn render_background(frame: &mut Frame, area: Rect, frame_index: usize) {
    // 1. Clear with background color
    let bg = Block::default().style(Style::default().bg(Color::Rgb(R, G, B)));
    frame.render_widget(bg, area);

    // 2. Render animated elements
    for y in 0..area.height {
        for x in 0..area.width {
            // Calculate what to draw at (x, y) based on frame_index
            // ...

            // Render single character
            frame.render_widget(
                Paragraph::new("█").style(Style::default().fg(color)),
                Rect::new(area.x + x, area.y + y, 1, 1),
            );
        }
    }
}
```

## Common ASCII Characters for Effects

```
Blocks:    █ ▓ ▒ ░ ▄ ▀ ▌ ▐
Dots:      · • ● ○ ◦ ◘ ◙
Stars:     ★ ☆ ✦ ✧ * ·
Lines:     │ ─ ┌ ┐ └ ┘ ╱ ╲
Arrows:    ↑ ↓ ← → ↖ ↗ ↘ ↙
Nature:    ♠ ♣ ❀ ✿ ☘ 🌿
Weather:   ☀ ☁ ☂ ❄ ⁂ ✺
Misc:      ◆ ◇ ∎ ∴ ∵ ≋ ⌇
```

## Registration

After creating your theme file, register it in `src/animation/themes/mod.rs`:

1. Add module declaration at top:
   ```rust
   pub mod your_theme;
   ```

2. Add to `ThemeType` enum:
   ```rust
   pub enum ThemeType {
       // ... existing
       YourTheme,
   }
   ```

3. Add to `all()` function

4. Add to `name()` match:
   ```rust
   ThemeType::YourTheme => "Your Theme Display Name",
   ```

5. Add to `render_background()` match:
   ```rust
   ThemeType::YourTheme => your_theme::render_background(frame, area, frame_index),
   ```

6. Add colors in `primary_color()`, `secondary_color()`, `background_color()`

## Tips

- Use `frame_index` for animation timing (multiply by 0.01-0.1 for speed)
- Keep calculations simple - this runs every frame
- Test at different terminal sizes
- Colors should complement each other
- Background should be dark enough for timer digits to be readable
