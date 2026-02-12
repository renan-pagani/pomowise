#!/usr/bin/env node

import { createWriteStream, existsSync, mkdirSync, chmodSync, unlinkSync, renameSync } from 'node:fs';
import { join } from 'node:path';
import { homedir, platform, arch } from 'node:os';
import { createHash } from 'node:crypto';
import { get } from 'node:https';

const VERSION = '1.0.0';
const GITHUB_REPO = 'renan-pagani/pomowise';
const INSTALL_DIR = join(homedir(), '.pomowise', 'bin');

function getTarget() {
  const os = platform();
  const cpuArch = arch();

  const targets = {
    'linux-x64': 'x86_64-unknown-linux-gnu',
    'darwin-x64': 'aarch64-apple-darwin', // ARM64 works on Intel via Rosetta 2
    'darwin-arm64': 'aarch64-apple-darwin',
    'win32-x64': 'x86_64-pc-windows-msvc',
  };

  const key = `${os}-${cpuArch}`;
  const target = targets[key];

  if (!target) {
    console.error(`Unsupported platform: ${key}`);
    console.error(`Supported: ${Object.keys(targets).join(', ')}`);
    process.exit(1);
  }

  return { target, os, isWindows: os === 'win32' };
}

function downloadFile(url) {
  return new Promise((resolve, reject) => {
    const follow = (url, redirects = 0) => {
      if (redirects > 5) return reject(new Error('Too many redirects'));

      get(url, (res) => {
        if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
          return follow(res.headers.location, redirects + 1);
        }
        if (res.statusCode !== 200) {
          return reject(new Error(`HTTP ${res.statusCode} for ${url}`));
        }
        resolve(res);
      }).on('error', reject);
    };

    follow(url);
  });
}

async function downloadAndVerify(url, checksumUrl, destPath) {
  console.log(`Downloading from ${url}...`);

  // Download checksum first
  const checksumStream = await downloadFile(checksumUrl);
  let checksumData = '';
  for await (const chunk of checksumStream) {
    checksumData += chunk;
  }

  // Parse expected hash (format: "hash  filename")
  const expectedHash = checksumData.trim().split(/\s+/)[0];
  if (!expectedHash || expectedHash.length !== 64) {
    throw new Error('Invalid checksum file format');
  }

  // Download binary
  const fileStream = await downloadFile(url);
  const tmpPath = destPath + '.tmp';
  const writeStream = createWriteStream(tmpPath);
  const hash = createHash('sha256');

  await new Promise((resolve, reject) => {
    fileStream.on('data', (chunk) => {
      hash.update(chunk);
      writeStream.write(chunk);
    });
    fileStream.on('end', () => {
      writeStream.end();
      resolve();
    });
    fileStream.on('error', reject);
    writeStream.on('error', reject);
  });

  // Verify checksum
  const actualHash = hash.digest('hex');
  if (actualHash !== expectedHash) {
    unlinkSync(tmpPath);
    throw new Error(`Checksum mismatch!\n  Expected: ${expectedHash}\n  Got:      ${actualHash}`);
  }

  console.log('Checksum verified.');
  renameSync(tmpPath, destPath);
}

async function main() {
  const { target, isWindows } = getTarget();

  // Ensure install directory exists
  if (!existsSync(INSTALL_DIR)) {
    mkdirSync(INSTALL_DIR, { recursive: true });
  }

  const ext = isWindows ? '.zip' : '.tar.gz';
  const assetName = `pomowise-${target}${ext}`;
  const baseUrl = `https://github.com/${GITHUB_REPO}/releases/download/v${VERSION}`;
  const assetUrl = `${baseUrl}/${assetName}`;
  const checksumUrl = `${baseUrl}/${assetName}.sha256`;

  const archivePath = join(INSTALL_DIR, assetName);

  try {
    await downloadAndVerify(assetUrl, checksumUrl, archivePath);
  } catch (e) {
    console.error(`Failed to download binary: ${e.message}`);
    console.error('You may need to build from source instead.');
    process.exit(1);
  }

  // Extract archive
  console.log('Extracting...');

  if (isWindows) {
    const { execSync } = await import('node:child_process');
    execSync(`powershell -Command "Expand-Archive -Force '${archivePath}' '${INSTALL_DIR}'"`, {
      stdio: 'inherit',
    });
  } else {
    const { execSync } = await import('node:child_process');
    execSync(`tar -xzf "${archivePath}" -C "${INSTALL_DIR}"`, {
      stdio: 'inherit',
    });

    const binaries = ['pomowise', 'pomowise-tray'];
    for (const bin of binaries) {
      const binPath = join(INSTALL_DIR, bin);
      if (existsSync(binPath)) {
        chmodSync(binPath, 0o755);
      }
    }
  }

  // Cleanup archive
  unlinkSync(archivePath);

  console.log('');
  console.log('Pomowise installed successfully!');
  console.log('');
  console.log('Commands:');
  console.log('  pomo        - Start the Pomodoro timer');
  console.log('  pomo-tray   - Start the system tray icon');
}

main().catch((err) => {
  console.error('Installation failed:', err.message);
  process.exit(1);
});
