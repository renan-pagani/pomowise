# Pomowise Installer

> A retro terminal setup wizard for the Pomowise pomodoro timer.

```
    ██████╗  ██████╗ ███╗   ███╗ ██████╗ ██╗    ██╗██╗███████╗███████╗
    ██╔══██╗██╔═══██╗████╗ ████║██╔═══██╗██║    ██║██║██╔════╝██╔════╝
    ██████╔╝██║   ██║██╔████╔██║██║   ██║██║ █╗ ██║██║███████╗█████╗
    ██╔═══╝ ██║   ██║██║╚██╔╝██║██║   ██║██║███╗██║██║╚════██║██╔══╝
    ██║     ╚██████╔╝██║ ╚═╝ ██║╚██████╔╝╚███╔███╔╝██║███████║███████╗
    ╚═╝      ╚═════╝ ╚═╝     ╚═╝ ╚═════╝  ╚══╝╚══╝ ╚═╝╚══════╝╚══════╝
```

## Installation

```bash
npm install -g pomowise
pomowise setup
```

## What it does

The setup wizard will:

1. **Detect your system** - OS, shell (bash/zsh/fish), architecture
2. **Check for Rust** - Required to build the timer
3. **Install Rust** - If needed (via rustup)
4. **Build from source** - Compiles the release binary
5. **Configure shell** - Adds `pomo` alias to your shell config

## After Installation

Restart your terminal and run:

```bash
pomo --help
pomo start
pomo
```

## Requirements

- Node.js 18+
- Git
- Internet connection (to download Rust if needed)

## Features

- Matrix rain splash screen
- Typing effect text output
- Green/cyan terminal color scheme
- Progress bars with block characters
- Box-drawing character panels
- Auto-detection of shell configuration

## Development

```bash
cd installer
npm install
npm test
```

## License

MIT
