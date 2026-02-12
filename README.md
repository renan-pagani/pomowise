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

One command to install:

```bash
npm install -g pomowise
```

Or with pnpm:

```bash
pnpm install -g pomowise
```

**What happens during install:**
- Pre-compiled binaries are downloaded automatically for your platform
- Checksum verification ensures security
- Binaries are extracted to `~/.pomowise/bin`
- Commands `pomo` and `pomo-tray` become available in your PATH

### Supported Platforms

| Platform | Architecture | Notes |
|----------|-------------|-------|
| Linux    | x64         | Tested on Ubuntu, Arch, Debian |
| macOS    | ARM64 (M1/M2/M3) | Native Apple Silicon |
| macOS    | x64 (Intel) | Runs via Rosetta 2 |
| Windows  | x64         | Windows 10/11 |

## Uninstall

**Linux / macOS:**
```bash
npm uninstall -g pomowise
rm -rf ~/.pomowise
```

**Windows (PowerShell):**
```powershell
npm uninstall -g pomowise
Remove-Item -Recurse -Force $env:USERPROFILE\.pomowise
```

**Windows (CMD):**
```cmd
npm uninstall -g pomowise
rmdir /s /q %USERPROFILE%\.pomowise
```

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

## Troubleshooting

### Installation fails with "checksum mismatch" or "tar: Unexpected EOF"

This was fixed in v1.0.1. Make sure you're installing the latest version:

```bash
npm install -g pomowise@latest
```

### Command not found: pomo

Make sure npm's global bin directory is in your PATH:

**Linux/macOS:**
```bash
npm config get prefix  # Should show path like /usr/local or ~/.npm-global
# Add to ~/.bashrc or ~/.zshrc:
export PATH="$PATH:$(npm config get prefix)/bin"
```

**Windows:**
```powershell
npm config get prefix  # Should show AppData\Roaming\npm
# Already in PATH by default, try restarting terminal
```

### System tray not working on Linux

Install required dependencies:

```bash
# Arch
sudo pacman -S gtk3 libayatana-appindicator xdotool

# Ubuntu/Debian
sudo apt install libgtk-3-dev libayatana-appindicator3-dev libxdo-dev
```

### Still having issues?

1. Check [GitHub Issues](https://github.com/renan-pagani/pomowise/issues)
2. Try building from source (see below)

## Build from Source

Requires Rust toolchain.

```bash
git clone https://github.com/renan-pagani/pomowise.git
cd pomowise
cargo build --release
```

Binaries will be in: `target/release/pomowise` and `target/release/pomowise-tray`

**Linux dependencies for the tray binary:**

```bash
# Arch
sudo pacman -S gtk3 libayatana-appindicator xdotool

# Ubuntu/Debian
sudo apt install libgtk-3-dev libayatana-appindicator3-dev libxdo-dev libdbus-1-dev pkg-config
```

## Links

- [npm package](https://www.npmjs.com/package/pomowise)
- [GitHub repository](https://github.com/renan-pagani/pomowise)
- [Report issues](https://github.com/renan-pagani/pomowise/issues)

## License

MIT
