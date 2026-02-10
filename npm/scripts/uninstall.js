#!/usr/bin/env node

import { rmSync, existsSync } from 'node:fs';
import { join } from 'node:path';
import { homedir } from 'node:os';
import { execSync } from 'node:child_process';

const INSTALL_DIR = join(homedir(), '.pomowise', 'bin');

// Kill running tray process
try {
  if (process.platform !== 'win32') {
    execSync('pkill -f pomowise-tray', { stdio: 'ignore' });
  }
} catch {
  // Process not running, that's fine
}

// Remove binaries
const binaries = process.platform === 'win32'
  ? ['pomowise.exe', 'pomowise-tray.exe']
  : ['pomowise', 'pomowise-tray'];

for (const bin of binaries) {
  const binPath = join(INSTALL_DIR, bin);
  if (existsSync(binPath)) {
    rmSync(binPath);
    console.log(`Removed ${binPath}`);
  }
}

console.log('Pomowise uninstalled.');
