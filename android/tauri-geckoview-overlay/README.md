# PocketRisu GeckoView Overlay

This directory contains files to overlay onto a local `tauri-geckoview-template` Android project.

- `app/src/main/assets/www/` is populated by `pnpm android:prepare:geckoview`
- `app/src/main/assets/pocketrisu-runtime.json` is generated alongside the web assets

Run from repository root:

```bash
pnpm build:android-local
pnpm android:prepare:geckoview
```
