import { execSync } from 'child_process';
import { existsSync } from 'fs';
import { homedir } from 'os';
import { join } from 'path';
import shell from 'shelljs';

// Detect operating system
export function detectOS() {
  const platform = process.platform;

  switch (platform) {
    case 'darwin':
      return {
        name: 'macOS',
        platform: 'darwin',
        icon: '',
      };
    case 'linux':
      return {
        name: 'Linux',
        platform: 'linux',
        icon: '',
      };
    case 'win32':
      return {
        name: 'Windows',
        platform: 'win32',
        icon: '',
      };
    default:
      return {
        name: platform,
        platform,
        icon: '?',
      };
  }
}

// Detect current shell
export function detectShell() {
  const shellEnv = process.env.SHELL || '';
  const home = homedir();

  if (shellEnv.includes('zsh')) {
    return {
      name: 'zsh',
      configFile: join(home, '.zshrc'),
      detected: true,
    };
  }

  if (shellEnv.includes('bash')) {
    // Check for .bashrc or .bash_profile
    const bashrc = join(home, '.bashrc');
    const bashProfile = join(home, '.bash_profile');

    return {
      name: 'bash',
      configFile: existsSync(bashrc) ? bashrc : bashProfile,
      detected: true,
    };
  }

  if (shellEnv.includes('fish')) {
    return {
      name: 'fish',
      configFile: join(home, '.config', 'fish', 'config.fish'),
      detected: true,
    };
  }

  // Fallback - try to detect from running process
  if (process.platform === 'win32') {
    return {
      name: 'powershell',
      configFile: join(home, 'Documents', 'WindowsPowerShell', 'Microsoft.PowerShell_profile.ps1'),
      detected: false,
    };
  }

  // Default to bash
  return {
    name: 'bash',
    configFile: join(home, '.bashrc'),
    detected: false,
  };
}

// Check if Rust is installed
export function checkRust() {
  try {
    const rustcVersion = execSync('rustc --version', { encoding: 'utf-8', stdio: ['pipe', 'pipe', 'pipe'] }).trim();
    const cargoVersion = execSync('cargo --version', { encoding: 'utf-8', stdio: ['pipe', 'pipe', 'pipe'] }).trim();

    return {
      installed: true,
      rustcVersion,
      cargoVersion,
      path: shell.which('rustc'),
    };
  } catch (err) {
    return {
      installed: false,
      rustcVersion: null,
      cargoVersion: null,
      path: null,
    };
  }
}

// Check if git is installed
export function checkGit() {
  try {
    const version = execSync('git --version', { encoding: 'utf-8', stdio: ['pipe', 'pipe', 'pipe'] }).trim();
    return {
      installed: true,
      version,
      path: shell.which('git'),
    };
  } catch (err) {
    return {
      installed: false,
      version: null,
      path: null,
    };
  }
}

// Get system architecture
export function getArchitecture() {
  return process.arch;
}

// Check if command exists
export function commandExists(cmd) {
  return shell.which(cmd) !== null;
}

// Get all shell config files that might exist
export function getAllShellConfigs() {
  const home = homedir();

  const configs = [
    { name: 'zsh', file: join(home, '.zshrc') },
    { name: 'bash', file: join(home, '.bashrc') },
    { name: 'bash_profile', file: join(home, '.bash_profile') },
    { name: 'fish', file: join(home, '.config', 'fish', 'config.fish') },
    { name: 'profile', file: join(home, '.profile') },
  ];

  return configs.filter((c) => existsSync(c.file));
}

// Full system detection
export function detectSystem() {
  return {
    os: detectOS(),
    shell: detectShell(),
    rust: checkRust(),
    git: checkGit(),
    arch: getArchitecture(),
    home: homedir(),
    shellConfigs: getAllShellConfigs(),
  };
}

export default {
  detectOS,
  detectShell,
  checkRust,
  checkGit,
  getArchitecture,
  commandExists,
  getAllShellConfigs,
  detectSystem,
};
