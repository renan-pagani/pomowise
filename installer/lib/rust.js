import { execSync, spawn } from 'child_process';
import { existsSync, mkdirSync, appendFileSync, readFileSync, writeFileSync, copyFileSync, chmodSync, rmSync } from 'fs';
import { homedir } from 'os';
import { join } from 'path';
import ora from 'ora';
import { colors, status, animatedProgress, sleep } from './ui.js';

const REPO_URL = 'https://github.com/renan-pagani/pomowise.git';
export const INSTALL_DIR = join(homedir(), '.pomowise');
export const BINARY_NAME = process.platform === 'win32' ? 'pomowise.exe' : 'pomowise';

// Install Rust via rustup
export async function installRust() {
  const spinner = ora({
    text: 'Installing Rust toolchain...',
    color: 'green',
  }).start();

  return new Promise((resolve, reject) => {
    if (process.platform === 'win32') {
      spinner.fail('Automatic Rust installation not supported on Windows');
      console.log();
      status('info', 'Please install Rust manually from: https://rustup.rs');
      resolve(false);
      return;
    }

    const rustupInstall = spawn('sh', ['-c', 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'], {
      stdio: ['inherit', 'pipe', 'pipe'],
    });

    let output = '';

    rustupInstall.stdout.on('data', (data) => {
      output += data.toString();
    });

    rustupInstall.stderr.on('data', (data) => {
      output += data.toString();
    });

    rustupInstall.on('close', (code) => {
      if (code === 0) {
        spinner.succeed('Rust toolchain installed successfully');

        // Source cargo env
        const cargoEnv = join(homedir(), '.cargo', 'env');
        if (existsSync(cargoEnv)) {
          try {
            execSync(`. ${cargoEnv}`, { shell: '/bin/bash' });
          } catch (e) {
            // Ignore sourcing errors
          }
        }

        // Update PATH for current process
        const cargoPath = join(homedir(), '.cargo', 'bin');
        process.env.PATH = `${cargoPath}:${process.env.PATH}`;

        resolve(true);
      } else {
        spinner.fail('Failed to install Rust');
        console.log(colors.dim(output));
        resolve(false);
      }
    });

    rustupInstall.on('error', (err) => {
      spinner.fail(`Failed to install Rust: ${err.message}`);
      resolve(false);
    });
  });
}

// Clone the repository
export async function cloneRepo(repoUrl = REPO_URL) {
  const spinner = ora({
    text: 'Cloning repository...',
    color: 'green',
  }).start();

  return new Promise((resolve, reject) => {
    // Create install directory
    if (!existsSync(INSTALL_DIR)) {
      mkdirSync(INSTALL_DIR, { recursive: true });
    }

    const srcDir = join(INSTALL_DIR, 'src');

    // Remove existing source if present
    if (existsSync(srcDir)) {
      if (!srcDir.startsWith(INSTALL_DIR)) {
        throw new Error('Refusing to delete path outside install directory');
      }
      rmSync(srcDir, { recursive: true, force: true });
    }

    const git = spawn('git', ['clone', '--depth', '1', repoUrl, srcDir], {
      stdio: ['inherit', 'pipe', 'pipe'],
    });

    let output = '';

    git.stdout.on('data', (data) => {
      output += data.toString();
    });

    git.stderr.on('data', (data) => {
      output += data.toString();
    });

    git.on('close', (code) => {
      if (code === 0) {
        spinner.succeed('Repository cloned');
        resolve(srcDir);
      } else {
        spinner.fail('Failed to clone repository');
        console.log(colors.dim(output));
        resolve(null);
      }
    });

    git.on('error', (err) => {
      spinner.fail(`Git error: ${err.message}`);
      resolve(null);
    });
  });
}

// Build the project with cargo
export async function buildProject(srcDir) {
  const spinner = ora({
    text: 'Building project (this may take a few minutes)...',
    color: 'green',
  }).start();

  return new Promise((resolve, reject) => {
    // Ensure cargo is in PATH
    const cargoPath = join(homedir(), '.cargo', 'bin');
    const env = {
      ...process.env,
      PATH: `${cargoPath}:${process.env.PATH}`,
    };

    const cargo = spawn('cargo', ['build', '--release'], {
      cwd: srcDir,
      stdio: ['inherit', 'pipe', 'pipe'],
      env,
    });

    let output = '';
    let lastLine = '';

    cargo.stdout.on('data', (data) => {
      output += data.toString();
      const lines = data.toString().split('\n').filter(Boolean);
      if (lines.length > 0) {
        lastLine = lines[lines.length - 1];
        spinner.text = `Building: ${lastLine.substring(0, 50)}...`;
      }
    });

    cargo.stderr.on('data', (data) => {
      output += data.toString();
      const lines = data.toString().split('\n').filter(Boolean);
      if (lines.length > 0) {
        lastLine = lines[lines.length - 1];
        // Filter out progress lines and show compilation status
        if (lastLine.includes('Compiling') || lastLine.includes('Downloading')) {
          spinner.text = lastLine.substring(0, 60);
        }
      }
    });

    cargo.on('close', (code) => {
      if (code === 0) {
        spinner.succeed('Build completed successfully');

        const binaryPath = join(srcDir, 'target', 'release', BINARY_NAME);
        if (existsSync(binaryPath)) {
          resolve(binaryPath);
        } else {
          // Try alternative binary name
          const altBinary = join(srcDir, 'target', 'release', 'tui-ansy-pomo');
          if (existsSync(altBinary)) {
            resolve(altBinary);
          } else {
            spinner.fail('Binary not found after build');
            resolve(null);
          }
        }
      } else {
        spinner.fail('Build failed');
        console.log(colors.error('\nBuild output:'));
        console.log(colors.dim(output.slice(-2000))); // Last 2000 chars
        resolve(null);
      }
    });

    cargo.on('error', (err) => {
      spinner.fail(`Cargo error: ${err.message}`);
      resolve(null);
    });
  });
}

// Copy binary to install location
export function installBinary(binaryPath) {
  const binDir = join(INSTALL_DIR, 'bin');

  if (!existsSync(binDir)) {
    mkdirSync(binDir, { recursive: true });
  }

  const destPath = join(binDir, BINARY_NAME);

  try {
    copyFileSync(binaryPath, destPath);
    chmodSync(destPath, 0o755);
    return destPath;
  } catch (err) {
    console.error(colors.error(`Failed to install binary: ${err.message}`));
    return null;
  }
}

// Add alias to shell config
export function addShellAlias(shellConfig, binaryPath) {
  const aliasLine = `\n# Pomowise - Retro Pomodoro Timer\nalias pomo="${binaryPath}"\n`;

  try {
    // Check if alias already exists
    if (existsSync(shellConfig)) {
      const content = readFileSync(shellConfig, 'utf-8');
      if (content.includes('alias pomo=')) {
        // Update existing alias
        const updated = content.replace(/alias pomo="[^"]*"/, `alias pomo="${binaryPath}"`);
        if (updated !== content) {
          writeFileSync(shellConfig, updated);
          return { action: 'updated', file: shellConfig };
        }
        return { action: 'exists', file: shellConfig };
      }
    }

    // Append new alias
    appendFileSync(shellConfig, aliasLine);
    return { action: 'added', file: shellConfig };
  } catch (err) {
    return { action: 'error', file: shellConfig, error: err.message };
  }
}

// Add to PATH (alternative to alias)
export function addToPath(shellConfig, binDir) {
  const pathLine = `\n# Pomowise - Add to PATH\nexport PATH="$PATH:${binDir}"\n`;

  try {
    if (existsSync(shellConfig)) {
      const content = readFileSync(shellConfig, 'utf-8');
      if (content.includes(binDir)) {
        return { action: 'exists', file: shellConfig };
      }
    }

    appendFileSync(shellConfig, pathLine);
    return { action: 'added', file: shellConfig };
  } catch (err) {
    return { action: 'error', file: shellConfig, error: err.message };
  }
}

// Build from local directory (for development)
export async function buildLocal(projectDir) {
  const spinner = ora({
    text: 'Building from local source...',
    color: 'green',
  }).start();

  return new Promise((resolve) => {
    const cargoPath = join(homedir(), '.cargo', 'bin');
    const env = {
      ...process.env,
      PATH: `${cargoPath}:${process.env.PATH}`,
    };

    const cargo = spawn('cargo', ['build', '--release'], {
      cwd: projectDir,
      stdio: ['inherit', 'pipe', 'pipe'],
      env,
    });

    let output = '';

    cargo.stderr.on('data', (data) => {
      output += data.toString();
      const lines = data.toString().split('\n').filter(Boolean);
      if (lines.length > 0) {
        const lastLine = lines[lines.length - 1];
        if (lastLine.includes('Compiling')) {
          spinner.text = lastLine.substring(0, 60);
        }
      }
    });

    cargo.on('close', (code) => {
      if (code === 0) {
        spinner.succeed('Local build completed');

        // Find the binary
        const possibleNames = ['pomowise', 'pomo', 'tui-ansy-pomo', BINARY_NAME];
        for (const name of possibleNames) {
          const binaryPath = join(projectDir, 'target', 'release', name);
          if (existsSync(binaryPath)) {
            resolve(binaryPath);
            return;
          }
        }

        spinner.warn('Binary not found, checking target directory...');
        resolve(null);
      } else {
        spinner.fail('Local build failed');
        resolve(null);
      }
    });

    cargo.on('error', (err) => {
      spinner.fail(`Build error: ${err.message}`);
      resolve(null);
    });
  });
}

export default {
  installRust,
  cloneRepo,
  buildProject,
  installBinary,
  addShellAlias,
  addToPath,
  buildLocal,
  INSTALL_DIR,
  BINARY_NAME,
};
