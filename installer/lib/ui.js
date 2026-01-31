import chalk from 'chalk';
import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Matrix rain characters
const MATRIX_CHARS = 'アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン0123456789';

// Terminal colors
export const colors = {
  primary: chalk.green,
  secondary: chalk.cyan,
  accent: chalk.greenBright,
  dim: chalk.gray,
  error: chalk.red,
  warning: chalk.yellow,
  success: chalk.greenBright,
  highlight: chalk.bgGreen.black,
};

// Box drawing characters
export const box = {
  topLeft: '╔',
  topRight: '╗',
  bottomLeft: '╚',
  bottomRight: '╝',
  horizontal: '═',
  vertical: '║',
  teeLeft: '╠',
  teeRight: '╣',
  cross: '╬',
};

// Sleep utility
export const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

// Typing effect
export async function typeText(text, options = {}) {
  const { speed = 30, color = colors.primary, newLine = true } = options;

  for (const char of text) {
    process.stdout.write(color(char));
    await sleep(speed);
  }

  if (newLine) {
    console.log();
  }
}

// Fast typing for longer text
export async function typeTextFast(text, options = {}) {
  return typeText(text, { ...options, speed: 10 });
}

// Matrix rain effect
export async function matrixRain(duration = 2000, width = 60, height = 8) {
  const columns = Array(width).fill(0);
  const startTime = Date.now();

  // Hide cursor
  process.stdout.write('\x1B[?25l');

  while (Date.now() - startTime < duration) {
    let frame = '';

    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        if (Math.random() > 0.95) {
          columns[x] = y;
        }

        if (columns[x] === y) {
          frame += colors.accent(MATRIX_CHARS[Math.floor(Math.random() * MATRIX_CHARS.length)]);
        } else if (columns[x] > y && columns[x] - y < 3) {
          frame += colors.primary(MATRIX_CHARS[Math.floor(Math.random() * MATRIX_CHARS.length)]);
        } else if (Math.random() > 0.97) {
          frame += colors.dim(MATRIX_CHARS[Math.floor(Math.random() * MATRIX_CHARS.length)]);
        } else {
          frame += ' ';
        }
      }
      frame += '\n';
    }

    process.stdout.write('\x1B[' + height + 'A'); // Move cursor up
    process.stdout.write(frame);
    await sleep(50);
  }

  // Clear the rain area
  for (let i = 0; i < height; i++) {
    process.stdout.write('\x1B[2K\n'); // Clear line
  }
  process.stdout.write('\x1B[' + height + 'A'); // Move cursor back up

  // Show cursor
  process.stdout.write('\x1B[?25h');
}

// Load and display ASCII logo
export async function showLogo() {
  try {
    const logoPath = join(__dirname, '..', 'assets', 'logo.txt');
    const logo = readFileSync(logoPath, 'utf-8');

    const lines = logo.split('\n');
    for (const line of lines) {
      console.log(colors.primary(line));
      await sleep(50);
    }
  } catch (err) {
    // Fallback inline logo
    const fallbackLogo = `
    ${colors.primary('██████╗  ██████╗ ███╗   ███╗ ██████╗ ██╗    ██╗██╗███████╗███████╗')}
    ${colors.primary('██╔══██╗██╔═══██╗████╗ ████║██╔═══██╗██║    ██║██║██╔════╝██╔════╝')}
    ${colors.secondary('██████╔╝██║   ██║██╔████╔██║██║   ██║██║ █╗ ██║██║███████╗█████╗')}
    ${colors.secondary('██╔═══╝ ██║   ██║██║╚██╔╝██║██║   ██║██║███╗██║██║╚════██║██╔══╝')}
    ${colors.accent('██║     ╚██████╔╝██║ ╚═╝ ██║╚██████╔╝╚███╔███╔╝██║███████║███████╗')}
    ${colors.accent('╚═╝      ╚═════╝ ╚═╝     ╚═╝ ╚═════╝  ╚══╝╚══╝ ╚═╝╚══════╝╚══════╝')}
    `;
    console.log(fallbackLogo);
  }
}

// Progress bar
export function progressBar(progress, total, width = 40) {
  const percentage = Math.round((progress / total) * 100);
  const filled = Math.round((progress / total) * width);
  const empty = width - filled;

  const filledBar = colors.primary('█'.repeat(filled));
  const emptyBar = colors.dim('░'.repeat(empty));
  const percentText = colors.accent(`${percentage.toString().padStart(3)}%`);

  return `[${filledBar}${emptyBar}] ${percentText}`;
}

// Animated progress bar
export async function animatedProgress(label, duration = 3000, width = 40) {
  const steps = 100;
  const stepDuration = duration / steps;

  process.stdout.write('\x1B[?25l'); // Hide cursor

  for (let i = 0; i <= steps; i++) {
    process.stdout.write('\r');
    process.stdout.write(colors.secondary(label.padEnd(20)));
    process.stdout.write(progressBar(i, steps, width));
    await sleep(stepDuration);
  }

  console.log();
  process.stdout.write('\x1B[?25h'); // Show cursor
}

// Draw a box around text
export function drawBox(lines, options = {}) {
  const { padding = 2, color = colors.secondary } = options;
  const maxLength = Math.max(...lines.map((l) => l.length));
  const width = maxLength + padding * 2;

  let output = '';

  // Top border
  output += color(box.topLeft + box.horizontal.repeat(width) + box.topRight) + '\n';

  // Content
  for (const line of lines) {
    const paddedLine = ' '.repeat(padding) + line + ' '.repeat(width - line.length - padding);
    output += color(box.vertical) + paddedLine + color(box.vertical) + '\n';
  }

  // Bottom border
  output += color(box.bottomLeft + box.horizontal.repeat(width) + box.bottomRight);

  return output;
}

// Section header
export function sectionHeader(title) {
  const line = '─'.repeat(50);
  console.log();
  console.log(colors.dim(line));
  console.log(colors.accent(`  ${title}`));
  console.log(colors.dim(line));
  console.log();
}

// Status message with icon
export function status(type, message) {
  const icons = {
    info: colors.secondary('ℹ'),
    success: colors.success('✓'),
    error: colors.error('✗'),
    warning: colors.warning('⚠'),
    arrow: colors.primary('→'),
  };

  console.log(`  ${icons[type] || icons.info} ${message}`);
}

// Clear screen
export function clearScreen() {
  process.stdout.write('\x1B[2J\x1B[3J\x1B[H');
}

// Show splash screen
export async function showSplash() {
  clearScreen();
  await matrixRain(1500, 70, 6);
  await showLogo();
  console.log();
  await typeText('  Initializing setup wizard...', { speed: 40, color: colors.dim });
  console.log();
  await sleep(500);
}

// Success screen
export async function showSuccess(binaryPath, alias) {
  console.log();
  const successBox = drawBox([
    '',
    '  INSTALLATION COMPLETE!  ',
    '',
    `  Binary: ${binaryPath}`,
    `  Alias:  ${alias}`,
    '',
    '  Restart your terminal and run:',
    '',
    '    pomo --help',
    '',
  ], { color: colors.success });

  console.log(successBox);
  console.log();

  await typeText('  Thank you for installing Pomowise!', { color: colors.accent });
  await typeText('  Focus. Flow. Finish.', { color: colors.dim, speed: 50 });
  console.log();
}

export default {
  colors,
  box,
  sleep,
  typeText,
  typeTextFast,
  matrixRain,
  showLogo,
  progressBar,
  animatedProgress,
  drawBox,
  sectionHeader,
  status,
  clearScreen,
  showSplash,
  showSuccess,
};
