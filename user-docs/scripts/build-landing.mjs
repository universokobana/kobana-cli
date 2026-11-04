#!/usr/bin/env node
/**
 * Materializes the locale-picker landing page as a static dir.
 *
 * Default target: build-landing/ (used by the per-locale dev preview).
 * With `--target <dir>` (e.g. --target build), writes the landing as the
 * root index.html of the production build, alongside the per-locale
 * subdirs the main `build` script produces.
 */

import { mkdirSync, copyFileSync, existsSync, rmSync, cpSync } from 'node:fs';
import { resolve, dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(__dirname, '..');
const SRC = resolve(ROOT, 'static/index.html');
const STATIC_DIR = resolve(ROOT, 'static');

function parseTarget() {
  const idx = process.argv.indexOf('--target');
  if (idx >= 0 && process.argv[idx + 1]) return process.argv[idx + 1];
  return 'build-landing';
}

const TARGET = parseTarget();
const OUT = resolve(ROOT, TARGET);
const STANDALONE = TARGET === 'build-landing';

if (!existsSync(SRC)) {
  console.error(`landing source not found: ${SRC}`);
  process.exit(1);
}

if (STANDALONE && existsSync(OUT)) rmSync(OUT, { recursive: true, force: true });
mkdirSync(OUT, { recursive: true });

copyFileSync(SRC, join(OUT, 'index.html'));

// The landing sits at the site root, outside every locale build, so it has
// no /pt, /en or /es prefix to borrow assets from. Drop the /img directory
// and the favicon pack it references next to the picker itself.
const imgSrc = join(STATIC_DIR, 'img');
if (existsSync(imgSrc)) {
  cpSync(imgSrc, join(OUT, 'img'), { recursive: true });
}

const faviconDir = join(STATIC_DIR, 'favicon');
if (existsSync(faviconDir)) {
  cpSync(faviconDir, join(OUT, 'favicon'), { recursive: true });
}

const faviconIco = join(STATIC_DIR, 'favicon.ico');
if (existsSync(faviconIco)) {
  copyFileSync(faviconIco, join(OUT, 'favicon.ico'));
}

console.log(`landing built at ${OUT}`);
