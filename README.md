# Pomowise

A retro terminal pomodoro timer with beautiful animated themes.

```
    ██████╗  ██████╗ ███╗   ███╗ ██████╗ ██╗    ██╗██╗███████╗███████╗
    ██╔══██╗██╔═══██╗████╗ ████║██╔═══██╗██║    ██║██║██╔════╝██╔════╝
    ██████╔╝██║   ██║██╔████╔██║██║   ██║██║ █╗ ██║██║███████╗█████╗
    ██╔═══╝ ██║   ██║██║╚██╔╝██║██║   ██║██║███╗██║██║╚════██║██╔══╝
    ██║     ╚██████╔╝██║ ╚═╝ ██║╚██████╔╝╚███╔███╔╝██║███████║███████╗
    ╚═╝      ╚═════╝ ╚═╝     ╚═╝ ╚═════╝  ╚══╝╚══╝ ╚═╝╚══════╝╚══════╝
```

## Quick Install

### Option 1: Node Installer (Recommended)

Requires Node.js 18+ and Git.

```bash
git clone https://github.com/yourusername/tui-ansy-pomo.git
cd tui-ansy-pomo/installer
npm install
node bin/pomowise.js setup
```

The setup wizard will:
- Detect your system and shell
- Install Rust if needed
- Build the binary
- Add the `pomo` alias to your shell

### Option 2: Build from Source

Requires Rust toolchain.

```bash
git clone https://github.com/yourusername/tui-ansy-pomo.git
cd tui-ansy-pomo
cargo build --release
```

Binary will be at `target/release/pomowise`.

## Usage

After installation, restart your terminal and run:

```bash
pomo --help    # Show available commands
pomo start     # Start a pomodoro session
pomo           # Launch the timer
```

## Features

- Animated ASCII art display
- Multiple color themes
- Desktop notifications
- Keyboard controls
- Works in any terminal

## Requirements

- macOS, Linux, or Windows
- Terminal with Unicode support
- Node.js 18+ (for installer) OR Rust (for manual build)

## License

MIT
