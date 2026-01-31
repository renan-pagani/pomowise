# Pomowise Redesign

## Overview

Transform tui-ansy-pomo into "pomowise" with npm installer, 10 new themes, 10 new fonts, and UX improvements.

## Changes

### 1. Rename to Pomowise
- Update `Cargo.toml`: name = "pomowise"
- Update `src/ui/menu.rs`: title = "pomowise"

### 2. npm Installer Package

**Package:** `pomowise` on npm

**Command flow:**
```bash
npm install -g pomowise
pomowise setup
```

**Setup wizard:**
1. ASCII splash with matrix rain
2. Detect OS/shell/Rust
3. Install Rust if needed
4. Build from source (cargo build --release)
5. Add `pomo` alias to shell profile
6. Theme preview & selection
7. Success animation

**Style:** Retro terminal aesthetic - typing effects, matrix rain, green/cyan colors, box-drawing chars

**Structure:**
```
installer/
├── package.json
├── bin/pomowise.js
├── lib/
│   ├── setup.js
│   ├── ui.js
│   ├── detect.js
│   └── rust.js
└── assets/logo.txt
```

### 3. New Themes (10)

| Theme | Visual Description |
|-------|-------------------|
| Landscape | Rolling hills, sun/moon cycle, clouds |
| Claude | Warm orange/tan gradients, subtle particles |
| GitHub | Dark charcoal, green contribution dots |
| Medieval | Castle silhouettes, torch flicker |
| Synthwave | Neon grid, sunset gradient |
| Nature | Falling leaves, forest breeze |
| Geometric | Rotating fractals, tessellations |
| Glitch | Scanlines, RGB split, noise |
| Minimal | Subtle gradient pulse, zen |
| Seasonal | Changes with real-world month |

### 4. New Fonts (10)

| Font | Style |
|------|-------|
| Organic | Rounded, hill-shaped curves |
| Claude | Clean geometric, Anthropic feel |
| Terminal | Sharp monospace edges |
| Gothic | Blackletter, medieval |
| Neon | 80s outline with chrome |
| Bamboo | Organic brush strokes |
| Angular | Tessellated blocks |
| Fragmented | Broken/corrupted segments |
| Hairline | Thin, minimalist |
| Seasonal | Adapts to current season |

### 5. Theme Auto-Rotation Toggle
- Hotkey: `a`
- Default: ON (rotates every 2.5 min)
- When OFF: shows `[theme locked]` indicator
- State: `App.auto_rotate: bool`

### 6. Hint Bar Toggle
- Hotkey: `h`
- Default: visible
- When hidden: flash "Press h for hints" for 2 seconds
- State: `App.hints_visible: bool`, `App.hint_flash_frames: u32`

### 7. Updated Hint Bar
```
Space: Pause  r: Reset  Tab: Skip  t: Themes  f: Font  a: Auto  h: Hide  q: Menu
```

## Implementation Streams

**Stream A: Core Changes** (sequential)
- Rename to pomowise
- Auto-rotate toggle
- Hint bar toggle
- Update hint text

**Stream B: npm Installer** (independent)
- Full setup wizard with retro UI

**Stream C: Themes** (10 parallel tasks)
- One subagent per theme

**Stream D: Fonts** (10 parallel tasks)
- One subagent per font

## Templates

Subagents will use existing files as templates:
- Theme template: `src/animation/themes/aurora.rs`
- Font template: `src/animation/digit_fonts.rs` (any font section)
