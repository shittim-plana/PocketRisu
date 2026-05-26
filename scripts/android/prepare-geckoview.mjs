#!/usr/bin/env node
import { cp, mkdir, readFile, rm, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, '..', '..');

const distDir = path.join(repoRoot, 'dist');
const overlayRoot = path.join(repoRoot, 'android', 'tauri-geckoview-overlay');
const webAssetsDir = path.join(overlayRoot, 'app', 'src', 'main', 'assets', 'www');
const metaDir = path.join(overlayRoot, 'app', 'src', 'main', 'assets');
const packageJsonPath = path.join(repoRoot, 'package.json');
const packageJson = JSON.parse(await readFile(packageJsonPath, 'utf-8'));

async function ensureDistExists() {
    try {
        const info = await stat(distDir);
        if (!info.isDirectory()) {
            throw new Error();
        }
    } catch {
        console.error('[android:prepare:geckoview] Missing dist/ output.');
        console.error('Run `pnpm build:android-local` first.');
        process.exit(1);
    }
}

await ensureDistExists();

await rm(webAssetsDir, { recursive: true, force: true });
await mkdir(webAssetsDir, { recursive: true });
await cp(distDir, webAssetsDir, { recursive: true });
await mkdir(metaDir, { recursive: true });

const runtimeManifest = {
    app: 'PocketRisu',
    version: packageJson.version,
    generatedAt: new Date().toISOString(),
    target: 'android-local-apk',
    architecture: 'embedded-frontend',
    unsupportedServerFeatures: [
        'Node server APIs (/api/*)',
        'Cloudflare Quick Tunnel',
        'Server-side SQLite backup dashboard',
    ],
    notes: [
        'This package is intended for tauri-geckoview-template Android wrappers.',
        'Local browser storage is used unless wrapper-native persistence is added.',
    ],
};

await writeFile(
    path.join(metaDir, 'pocketrisu-runtime.json'),
    `${JSON.stringify(runtimeManifest, null, 2)}\n`,
    'utf-8',
);

console.log(`[android:prepare:geckoview] Copied dist -> ${webAssetsDir}`);
console.log('[android:prepare:geckoview] Wrote runtime manifest.');
