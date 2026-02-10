#!/usr/bin/env node

import { spawn } from 'node:child_process';
import { join } from 'node:path';
import { homedir } from 'node:os';
import { existsSync } from 'node:fs';

const BINARY = process.platform === 'win32' ? 'pomowise-tray.exe' : 'pomowise-tray';
const binaryPath = join(homedir(), '.pomowise', 'bin', BINARY);

if (!existsSync(binaryPath)) {
  console.error('Pomowise tray binary not found. Run: pnpm install -g pomowise');
  process.exit(1);
}

// Run tray in background (detached)
const child = spawn(binaryPath, [], {
  detached: true,
  stdio: 'ignore',
});
child.unref();

console.log('Pomowise tray started in background.');
