<p align="center">
  <strong>English</strong> | <a href="../ko/install.md">한국어</a> | <a href="../de/install.md">Deutsch</a> | <a href="../cn/install.md">简体中文</a> | <a href="../es/install.md">Español</a> | <a href="../vi/install.md">Tiếng Việt</a> | <a href="../zh-Hant/install.md">繁體中文</a>
</p>

# Android Local APK (tauri-geckoview-template)

This guide explains how to package PocketRisu as a local Android APK with `tauri-geckoview-template`.

Scope of this integration:

- `verso` is intentionally excluded.
- Architecture is **embedded frontend assets** (`dist/` inside APK), not Node server-in-APK.
- This path is for local app packaging and testing on Android devices.

---

## 1. Build PocketRisu web assets for Android target

From repository root:

```bash
pnpm install
pnpm build:android-local
pnpm android:prepare:geckoview
```

After this, web assets are staged at:

`android/tauri-geckoview-overlay/app/src/main/assets/www`

Runtime metadata is generated at:

`android/tauri-geckoview-overlay/app/src/main/assets/pocketrisu-runtime.json`

---

## 2. Apply the overlay to your tauri-geckoview-template app

1. Prepare a local `tauri-geckoview-template` Android project.
2. Copy the overlay contents from:
   - `android/tauri-geckoview-overlay/app/src/main/assets/*`
3. Ensure your GeckoView app serves `assets/www/index.html` as startup content.

This repository intentionally ships only the PocketRisu overlay layer (not the full external template source tree).

---

## 3. Runtime scope and feature mapping

Because Android local APK mode does not run `server/node/server.cjs`, server-only features are unavailable by default.

- **Works in APK mode**
  - Frontend UI and chat flows backed by browser-local storage
  - Provider API usage from the frontend (depending on provider/network policy)
  - Import/export flows that are browser API based

- **Not available without extra native/server integration**
  - Node server APIs (`/api/*`)
  - Cloudflare Quick Tunnel
  - Node-side SQLite admin/backup endpoints

If you need server features, run PocketRisu server separately and connect to it over network instead of pure local-asset mode.

---

## 4. Permissions and security checks

Recommended Android checks for GeckoView wrapper app:

- Internet permission only if model/provider calls require network.
- File/media permissions only if your wrapper adds native file pickers.
- Keep WebView/GeckoView JavaScript bridge surface minimal.
- Restrict external navigation and untrusted URL handling.
- Keep mixed-content disabled unless explicitly needed.

PocketRisu frontend itself distinguishes Android-local build with `__ANDROID_LOCAL_APK__` define.

---

## 5. Storage path expectations

In this local APK path, data persistence is browser/engine local storage (IndexedDB/local storage), unless your wrapper adds custom native persistence bridges.

Before release, verify:

- Data survives app restart
- Backup/export/import flow works on your target Android versions
- Clear-data/uninstall behavior is acceptable for your users

---

## 6. Debug and release checklist

- Build debug APK and install on real device
- Validate core scenarios (chat, save/restore, provider call, long session)
- Configure release signing and ABI targets
- Optimize size (ABI split / R8 / resource shrinking as applicable)
- Document unsupported server-only features in release notes

---

← [Back to README](../../README.md)
