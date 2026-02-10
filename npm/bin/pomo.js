#!/usr/bin/env node

import { execFileSync } from 'node:child_process';
import { join } from 'node:path';
import { homedir } from 'node:os';
import { existsSync } from 'node:fs';

const BINARY = process.platform === 'win32' ? 'pomowise.exe' : 'pomowise';
const binaryPath = join(homedir(), '.pomowise', 'bin', BINARY);

if (!existsSync(binaryPath)) {
  console.error('Pomowise binary not found. Run: pnpm install -g pomowise');
  process.exit(1);
}

try {
  execFileSync(binaryPath, process.argv.slice(2), { stdio: 'inherit' });
} catch (e) {
  process.exit(e.status || 1);
}
