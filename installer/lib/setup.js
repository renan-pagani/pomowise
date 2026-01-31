import inquirer from 'inquirer';
import ora from 'ora';
import { existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

import {
  colors,
  showSplash,
  showSuccess,
  sectionHeader,
  status,
  typeText,
  typeTextFast,
  drawBox,
  sleep,
  clearScreen,
} from './ui.js';

import {
  detectSystem,
  detectShell,
  checkRust,
} from './detect.js';

import {
  installRust,
  cloneRepo,
  buildProject,
  buildLocal,
  installBinary,
  addShellAlias,
  INSTALL_DIR,
} from './rust.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Show help message
export function showHelp() {
  console.log(`
${colors.primary('POMOWISE')} - Retro Terminal Pomodoro Timer

${colors.secondary('USAGE:')}
  pomowise setup     Run the installation wizard
  pomowise --help    Show this help message

${colors.secondary('DESCRIPTION:')}
  Pomowise is a beautiful retro-style pomodoro timer for your terminal.
  This installer will:

  1. Check for Rust toolchain (and optionally install it)
  2. Build the application from source
  3. Install the binary to ~/.pomowise/bin
  4. Add a 'pomo' alias to your shell

${colors.secondary('AFTER INSTALLATION:')}
  Restart your terminal and run:

    ${colors.accent('pomo --help')}
    ${colors.accent('pomo start')}
    ${colors.accent('pomo')}

${colors.secondary('REPOSITORY:')}
  https://github.com/yourusername/pomowise

${colors.dim('Built with love and caffeine.')}
`);
}

// Main setup function
export async function runSetup() {
  try {
    // Show splash screen
    await showSplash();

    // Step 1: System Detection
    sectionHeader('SYSTEM DETECTION');

    const system = detectSystem();

    await sleep(300);
    status('info', `Operating System: ${colors.accent(system.os.name)} (${system.arch})`);
    await sleep(200);
    status('info', `Shell: ${colors.accent(system.shell.name)} ${system.shell.detected ? '' : colors.dim('(assumed)')}`);
    await sleep(200);
    status('info', `Config file: ${colors.dim(system.shell.configFile)}`);
    await sleep(200);

    if (system.rust.installed) {
      status('success', `Rust: ${colors.accent('Installed')}`);
      status('info', `  ${colors.dim(system.rust.rustcVersion)}`);
      status('info', `  ${colors.dim(system.rust.cargoVersion)}`);
    } else {
      status('warning', `Rust: ${colors.warning('Not found')}`);
    }

    await sleep(200);

    if (system.git.installed) {
      status('success', `Git: ${colors.accent('Installed')}`);
    } else {
      status('error', `Git: ${colors.error('Not found')}`);
      console.log();
      console.log(colors.error('  Git is required for installation.'));
      console.log(colors.dim('  Please install Git and try again.'));
      process.exit(1);
    }

    console.log();

    // Step 2: Rust Installation (if needed)
    if (!system.rust.installed) {
      sectionHeader('RUST INSTALLATION');

      const { installRustChoice } = await inquirer.prompt([
        {
          type: 'confirm',
          name: 'installRustChoice',
          message: 'Rust is not installed. Would you like to install it now?',
          default: true,
        },
      ]);

      if (installRustChoice) {
        console.log();
        const rustInstalled = await installRust();

        if (!rustInstalled) {
          console.log();
          status('error', 'Failed to install Rust. Please install manually:');
          console.log(colors.secondary('  https://rustup.rs'));
          process.exit(1);
        }

        // Re-check Rust
        const rustCheck = checkRust();
        if (!rustCheck.installed) {
          console.log();
          status('warning', 'Rust installed but not in PATH.');
          status('info', 'Please restart your terminal and run the installer again.');
          process.exit(0);
        }
      } else {
        console.log();
        status('info', 'Skipping Rust installation.');
        status('warning', 'You will need Rust to build Pomowise.');
        console.log(colors.dim('  Install from: https://rustup.rs'));
        process.exit(0);
      }
    }

    // Step 3: Installation Method
    sectionHeader('INSTALLATION');

    // Check if we're in the project directory
    const parentDir = join(__dirname, '..', '..');
    const cargoToml = join(parentDir, 'Cargo.toml');
    const isLocalInstall = existsSync(cargoToml);

    let buildMethod = 'clone';

    if (isLocalInstall) {
      const { useLocal } = await inquirer.prompt([
        {
          type: 'confirm',
          name: 'useLocal',
          message: 'Local source detected. Build from local directory?',
          default: true,
        },
      ]);

      if (useLocal) {
        buildMethod = 'local';
      }
    }

    console.log();

    let binaryPath = null;

    if (buildMethod === 'local') {
      // Build from local source
      await typeTextFast('Building from local source...', { color: colors.dim });
      console.log();

      binaryPath = await buildLocal(parentDir);
    } else {
      // Clone and build
      await typeTextFast('Cloning repository...', { color: colors.dim });
      console.log();

      const { repoUrl } = await inquirer.prompt([
        {
          type: 'input',
          name: 'repoUrl',
          message: 'Repository URL:',
          default: 'https://github.com/yourusername/pomowise.git',
        },
      ]);

      console.log();

      const srcDir = await cloneRepo(repoUrl);

      if (!srcDir) {
        status('error', 'Failed to clone repository.');
        process.exit(1);
      }

      binaryPath = await buildProject(srcDir);
    }

    if (!binaryPath) {
      status('error', 'Build failed. Please check the error messages above.');
      process.exit(1);
    }

    // Step 4: Install Binary
    sectionHeader('INSTALLING');

    const installedPath = installBinary(binaryPath);

    if (!installedPath) {
      status('error', 'Failed to install binary.');
      process.exit(1);
    }

    status('success', `Binary installed to: ${colors.dim(installedPath)}`);

    // Step 5: Shell Configuration
    sectionHeader('SHELL CONFIGURATION');

    const { configureShell } = await inquirer.prompt([
      {
        type: 'confirm',
        name: 'configureShell',
        message: `Add 'pomo' alias to ${system.shell.configFile}?`,
        default: true,
      },
    ]);

    if (configureShell) {
      const result = addShellAlias(system.shell.configFile, installedPath);

      switch (result.action) {
        case 'added':
          status('success', `Alias added to ${colors.dim(result.file)}`);
          break;
        case 'updated':
          status('success', `Alias updated in ${colors.dim(result.file)}`);
          break;
        case 'exists':
          status('info', `Alias already exists in ${colors.dim(result.file)}`);
          break;
        case 'error':
          status('error', `Failed to update ${result.file}: ${result.error}`);
          break;
      }
    } else {
      console.log();
      status('info', 'To manually add the alias, add this to your shell config:');
      console.log();
      console.log(colors.secondary(`  alias pomo="${installedPath}"`));
    }

    // Step 6: Success!
    sectionHeader('COMPLETE');

    await showSuccess(installedPath, 'pomo');

    // Final instructions
    const instructions = drawBox([
      '',
      '  NEXT STEPS:',
      '',
      '  1. Restart your terminal (or run: source ' + system.shell.configFile + ')',
      '  2. Run: pomo --help',
      '  3. Start your first session: pomo start',
      '',
    ], { color: colors.secondary });

    console.log(instructions);
    console.log();

  } catch (err) {
    if (err.name === 'ExitPromptError') {
      // User cancelled
      console.log();
      console.log(colors.dim('Installation cancelled.'));
      process.exit(0);
    }

    console.error(colors.error(`\nSetup error: ${err.message}`));
    console.error(colors.dim(err.stack));
    process.exit(1);
  }
}

export default {
  showHelp,
  runSetup,
};
