# Pomowise

A beautiful animated Pomodoro timer for your terminal with 24 themes, system tray support, and one-command install.

```
    ██████╗  ██████╗ ███╗   ███╗ ██████╗ ██╗    ██╗██╗███████╗███████╗
    ██╔══██╗██╔═══██╗████╗ ████║██╔═══██╗██║    ██║██║██╔════╝██╔════╝
    ██████╔╝██║   ██║██╔████╔██║██║   ██║██║ █╗ ██║██║███████╗█████╗
    ██╔═══╝ ██║   ██║██║╚██╔╝██║██║   ██║██║███╗██║██║╚════██║██╔══╝
    ██║     ╚██████╔╝██║ ╚═╝ ██║╚██████╔╝╚███╔███╔╝██║███████║███████╗
    ╚═╝      ╚═════╝ ╚═╝     ╚═╝ ╚═════╝  ╚══╝╚══╝ ╚═╝╚══════╝╚══════╝
```

## Install

```bash
pnpm install -g pomowise
```

That's it. Pre-compiled binaries are downloaded automatically for your platform.

Also works with npm:

```bash
npm install -g pomowise
```

## Uninstall

```bash
npm uninstall -g pomowise
rm -rf ~/.pomowise
```

### Supported platforms

| Platform | Architecture |
|----------|-------------|
| Linux    | x64         |
| macOS    | x64, ARM64 (Apple Silicon) |
| Windows  | x64         |

## Usage

```bash
pomo          # Start the timer
pomo-tray     # Start the system tray icon (runs in background)
```

### System Tray

Run `pomo-tray` to get a persistent icon in your system tray:

- Icon color changes by session: red (work), green (short break), blue (long break), orange (paused)
- Tooltip shows current session and time remaining
- Click the icon to open the TUI in a new terminal

### Keybindings

#### Menu

| Key | Action |
|-----|--------|
| `j` / `Down` | Move down |
| `k` / `Up` | Move up |
| `Enter` | Select |
| `q` | Quit |

#### Timer

| Key | Action |
|-----|--------|
| `Space` | Pause / Resume |
| `Tab` | Skip to next session |
| `r` | Reset current session |
| `t` | Open theme selector |
| `f` | Cycle font style |
| `F` | Toggle adaptive font |
| `a` | Toggle auto-rotation |
| `h` | Toggle hints |
| `q` | Back to menu |

#### Theme Selector

| Key | Action |
|-----|--------|
| `j` / `Down` | Next theme (live preview) |
| `k` / `Up` | Previous theme (live preview) |
| `Enter` | Confirm |
| `Esc` | Cancel |

## Themes

24 animated ASCII themes with unique color palettes:

Matrix Rain, Fire, Starfield, Plasma, Rain Drops, Radio Waves, Spinning Shapes, Fireworks, Aurora Borealis, Ocean Waves, DNA Helix, Bubbles, Electric Storm, Snowfall, Forest Nature, Geometric Patterns, Glitch Cyberpunk, Minimal Zen, Seasonal, Landscape, Claude, GitHub, Medieval, Synthwave

Themes auto-rotate between sessions, or pick one with `t`.

## Build from Source

Requires Rust toolchain.

```bash
git clone https://github.com/renan-pagani/pomowise.git
cd pomowise
cargo build --release
```

Binaries: `target/release/pomowise` and `target/release/pomowise-tray`

Linux dependencies for the tray binary:

```bash
# Arch
sudo pacman -S gtk3 libayatana-appindicator xdotool

# Ubuntu/Debian
sudo apt install libgtk-3-dev libayatana-appindicator3-dev libxdo-dev libdbus-1-dev pkg-config
```

## License

MIT
