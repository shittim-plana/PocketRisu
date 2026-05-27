<p align="center">
  <img src="assets/pocketrisu-banner-1024.png" alt="PocketRisu — Self-hosted AI Roleplay Chat Platform" width="900" />
</p>

<h1 align="center">PocketRisu — AI Roleplay Chat (Server & Android)</h1>

<p align="center">
  <strong>English</strong> | <a href="i18n/README.ko.md">한국어</a> | <a href="i18n/README.de.md">Deutsch</a> | <a href="i18n/README.cn.md">简体中文</a> | <a href="i18n/README.es.md">Español</a> | <a href="i18n/README.vi.md">Tiếng Việt</a> | <a href="i18n/README.zh-Hant.md">繁體中文</a>
</p>

<p align="center">
  <a href="https://github.com/shittim-plana/PocketRisu/releases">
    <img alt="Latest Release" src="https://img.shields.io/github/v/release/shittim-plana/PocketRisu?label=latest" />
  </a>
  <a href="LICENSE">
    <img alt="License: GPL-3.0" src="https://img.shields.io/github/license/shittim-plana/PocketRisu" />
  </a>
  <a href="https://github.com/shittim-plana/PocketRisu/releases">
    <img alt="Android APK" src="https://img.shields.io/badge/Android-APK-3DDC84?logo=android&logoColor=white" />
  </a>
</p>

PocketRisu is an AI roleplay chat platform that runs as a self-hosted web server **or** as a standalone Android app — no server required.

### Android APK (NEW)

Download the APK from [Releases](https://github.com/shittim-plana/PocketRisu/releases) and install directly on your Android device. The app runs entirely on-device with a local SQLite database — just add your AI API key and start chatting.

- **Tauri 2.0** + **GeckoView 140 ESR** (Mozilla's browser engine)
- Rust native HTTP proxy for AI provider calls
- arm64-v8a (aarch64) only

<p align="center">
  <table>
    <tr>
      <td align="center"><img src="assets/screenshots/screenshot-pc-chat.png" alt="PC chat" height="420" /></td>
      <td align="center"><img src="assets/screenshots/screenshot-mobile-chat.png" alt="Mobile chat" height="420" /></td>
    </tr>
    <tr>
      <td align="center"><b>PC</b></td>
      <td align="center"><b>Mobile</b></td>
    </tr>
  </table>
</p>


## Documentation

- [Installation guide (Server)](docs/en/install.md)
- [Android APK guide](docs/en/android-local-apk.md)
- [RisuAI migration guide](docs/en/migration.md)
- [Remote access guide](docs/en/remote.md)
- [Termux installation guide](docs/en/termux.md)


## RisuAI Compatibility

PocketRisu is derived from [RisuAI](https://github.com/kwaroran/RisuAI) and refined for self-hosted environments. Existing RisuAI data can be migrated wholesale, and all RisuAI ecosystem assets remain usable as-is.

- RisuRealm character downloads
- Character cards (`.charx`, `.risum`, `.risup`, etc.)
- Modules, lorebooks, presets
- Backup files (`.bin`) with two-way compatibility

For migration from an existing RisuAI installation, see the [migration guide](docs/en/migration.md).


## Features

- **Multiple AI providers**: OpenAI, Claude, Gemini, DeepInfra, OpenRouter, Ollama, and more
- **Two modes**: Self-hosted web server (PC/tablet/smartphone via browser) or standalone Android APK
- **Unified data storage**: All data in a single SQLite database — on your server or on-device
- **RisuRealm**: Browse and download characters from the Risu character hub
- **Lorebook & long-term memory**: World info / memory book, HypaMemoryV3, and other context retention features
- **Automatic translation**: Auto-translate input and output for cross-language roleplay
- **Regex scripts & plugins**: Plugin API 2.0 / 2.1 / 3.0 supported
- **TTS & additional assets**: Voice synthesis, embedded images / audio / video in chat
- **Server mode extras**: Dashboard, backup/restore, self-update, Quick Tunnel remote access
- **Multilingual UI**: Korean, English, Japanese, Chinese, and more


## Architecture

PocketRisu runs in two modes. The frontend is shared; the backend differs.

### Server Mode (PC / Self-hosted)

```
Browser ──HTTP──▶ Node.js Server ──▶ SQLite DB
                       │
                       ├── /api/* (CRUD, patch sync, chat, assets)
                       └── /proxy2 (AI provider relay)
```

### Android APK Mode

```
┌─ Android App ───────────────────────────────┐
│                                              │
│  GeckoView 140 ESR                           │
│    └── PocketRisu Svelte UI                  │
│           │                                  │
│           │ invoke()  (IPC WebExtension)      │
│           ▼                                  │
│  Tauri 2.0 / Rust                            │
│    ├── KV Store (SQLite, on-device)          │
│    ├── proxy_request (AI API relay)          │
│    └── streamed_fetch (streaming + events)   │
│                                              │
└──────────────────────────────────────────────┘
```

### Data Flow — Chat Message

```
User input
  → Svelte UI
    → invoke('streamed_fetch', { url, headers, body })
      → Rust: reqwest → AI Provider (OpenAI, Claude, etc.)
        ← SSE chunks → emit("streamed_fetch") events
      ← ReadableStream → Chat UI rendering
    → invoke('chat_content_save', { cha_id, chat_index, data })
      → SQLite KV store (on-device)
```

### Project Structure

```
PocketRisu/
├── src/                    Svelte frontend (shared)
│   └── ts/storage/
│       ├── autoStorage.ts      mode switch (Server ↔ Tauri)
│       ├── nodeStorage.ts      Server mode: HTTP fetch
│       └── tauriStorage.ts     Android mode: Tauri invoke
├── src-tauri/              Tauri Rust backend (Android)
│   ├── src/
│   │   ├── lib.rs              app entry + command registration
│   │   ├── commands.rs         13 Tauri commands
│   │   └── kv_store.rs         SQLite KV (rusqlite)
│   └── gen/android/        GeckoView overlay
│       └── app/src/main/java/.../
│           ├── MainActivity.kt     GeckoView + IPC bridge
│           └── AssetServer.kt      localhost asset server
├── server/                 Node.js server (Server mode only)
└── dist/                   Vite build output → APK assets
```


## Community & Contact

- Bug reports / feature requests: [GitHub Issues](https://github.com/shittim-plana/PocketRisu/issues)


## License

[GPL-3.0](LICENSE)
