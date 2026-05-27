#!/usr/bin/env node
import { cp, rm, mkdir } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');
const distDir = path.join(repoRoot, 'dist');
const assetsDir = path.join(repoRoot, 'src-tauri', 'gen', 'android', 'app', 'src', 'main', 'assets');

await rm(path.join(assetsDir, 'assets'), { recursive: true, force: true });
await rm(path.join(assetsDir, 'index.html'), { force: true });
await mkdir(assetsDir, { recursive: true });
await cp(path.join(distDir, 'index.html'), path.join(assetsDir, 'index.html'));
await cp(path.join(distDir, 'assets'), path.join(assetsDir, 'assets'), { recursive: true });

const count = (await import('node:fs')).readdirSync(path.join(assetsDir, 'assets')).length;
console.log(`[sync-tauri-assets] Copied dist → assets/ (${count} items)`);
