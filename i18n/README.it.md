<p align="center">
  <img src="../assets/pocketrisu-banner-1024.png" alt="PocketRisu — Chat di Roleplay IA (Server & Android)" width="900" />
</p>

<h1 align="center">PocketRisu — Chat di Roleplay IA (Server & Android)</h1>

<p align="center">
  <a href="../README.md">English</a> | <a href="README.ko.md">한국어</a> | <a href="README.de.md">Deutsch</a> | <a href="README.es.md">Español</a> | <a href="README.fr.md">Français</a> | <strong>Italiano</strong> | <a href="README.ja.md">日本語</a>
</p>

<p align="center">
  <a href="https://github.com/shittim-plana/PocketRisu/releases">
    <img alt="Latest Release" src="https://img.shields.io/github/v/release/shittim-plana/PocketRisu?label=latest" />
  </a>
  <a href="../LICENSE">
    <img alt="License: GPL-3.0" src="https://img.shields.io/github/license/shittim-plana/PocketRisu" />
  </a>
  <a href="https://github.com/shittim-plana/PocketRisu/releases">
    <img alt="Android APK" src="https://img.shields.io/badge/Android-APK-3DDC84?logo=android&logoColor=white" />
  </a>
</p>

PocketRisu e una piattaforma di chat di roleplay IA che funziona come web server self-hosted **o** come app Android autonoma — nessun server necessario.

### APK Android (NUOVO)

Scarica l'APK dalle [Releases](https://github.com/shittim-plana/PocketRisu/releases) e installalo direttamente sul tuo dispositivo Android. L'app funziona interamente sul dispositivo con un database SQLite locale — basta aggiungere la tua chiave API IA e iniziare a chattare.

- **Tauri 2.0** + **GeckoView 140 ESR** (motore browser di Mozilla)
- Proxy HTTP nativo Rust per le chiamate ai provider IA
- Solo arm64-v8a (aarch64)

<p align="center">
  <table>
    <tr>
      <td align="center"><img src="../assets/screenshots/screenshot-pc-chat.png" alt="Chat su PC" height="420" /></td>
      <td align="center"><img src="../assets/screenshots/screenshot-mobile-chat.png" alt="Chat mobile" height="420" /></td>
    </tr>
    <tr>
      <td align="center"><b>PC</b></td>
      <td align="center"><b>Mobile</b></td>
    </tr>
  </table>
</p>


## Documentazione

- [Guida all'installazione (Server)](../docs/en/install.md)
- [Guida APK Android](../docs/en/android-local-apk.md)
- [Guida alla migrazione da RisuAI](../docs/en/migration.md)
- [Accesso remoto](../docs/en/remote.md)
- [Guida all'installazione Termux](../docs/en/termux.md)


## Compatibilita con RisuAI

PocketRisu deriva da [RisuAI](https://github.com/kwaroran/RisuAI) ed e ottimizzato per ambienti self-hosted. I dati RisuAI esistenti possono essere migrati completamente, e tutte le risorse dell'ecosistema RisuAI rimangono utilizzabili cosi come sono.

- Download di personaggi RisuRealm
- Schede personaggio (`.charx`, `.risum`, `.risup`, ecc.)
- Moduli, lorebook, preset
- File di backup (`.bin`) con compatibilita bidirezionale

Per la migrazione da un'installazione RisuAI esistente, consulta la [guida alla migrazione](../docs/en/migration.md).


## Funzionalita

- **Molteplici provider IA**: OpenAI, Claude, Gemini, DeepInfra, OpenRouter, Ollama e altri
- **Due modalita**: Web server self-hosted (PC/tablet/smartphone via browser) o APK Android autonomo
- **Archiviazione unificata**: Tutti i dati in un unico database SQLite — sul tuo server o sul dispositivo
- **RisuRealm**: Sfoglia e scarica personaggi dall'hub personaggi di Risu
- **Lorebook e memoria a lungo termine**: World info / memory book, HypaMemoryV3 e altre funzionalita di mantenimento del contesto
- **Traduzione automatica**: Traduzione automatica di input e output per roleplay multilingue
- **Script regex e plugin**: Plugin API 2.0 / 2.1 / 3.0 supportati
- **TTS e risorse aggiuntive**: Sintesi vocale, immagini / audio / video integrati nella chat
- **Extra modalita server**: Dashboard, backup/ripristino, aggiornamento automatico, accesso remoto Quick Tunnel
- **Interfaccia multilingue**: Coreano, inglese, giapponese, cinese e altro


## Architettura

PocketRisu funziona in due modalita. Il frontend e condiviso; il backend differisce.

### Modalita Server (PC / Self-hosted)

```
Browser ──HTTP──▶ Server Node.js ──▶ SQLite DB
                       │
                       ├── /api/* (CRUD, sincronizzazione patch, chat, assets)
                       └── /proxy2 (relay provider IA)
```

### Modalita APK Android

```
┌─ App Android ──────────────────────────────────┐
│                                                │
│  GeckoView 140 ESR                             │
│    └── PocketRisu Svelte UI                    │
│           │                                    │
│           │ invoke()  (IPC WebExtension)        │
│           ▼                                    │
│  Tauri 2.0 / Rust                              │
│    ├── KV Store (SQLite, sul dispositivo)      │
│    ├── proxy_request (relay API IA)            │
│    └── streamed_fetch (streaming + eventi)     │
│                                                │
└────────────────────────────────────────────────┘
```

### Flusso dati — Messaggio chat

```
Input utente
  → Svelte UI
    → invoke('streamed_fetch', { url, headers, body })
      → Rust: reqwest → Provider IA (OpenAI, Claude, ecc.)
        ← Chunk SSE → emit("streamed_fetch") eventi
      ← ReadableStream → Rendering UI chat
    → invoke('chat_content_save', { cha_id, chat_index, data })
      → SQLite KV Store (sul dispositivo)
```

### Struttura del progetto

```
PocketRisu/
├── src/                    Frontend Svelte (condiviso)
│   └── ts/storage/
│       ├── autoStorage.ts      Selettore modalita (Server ↔ Tauri)
│       ├── nodeStorage.ts      Modalita server: HTTP fetch
│       └── tauriStorage.ts     Modalita Android: Tauri invoke
├── src-tauri/              Backend Rust Tauri (Android)
│   ├── src/
│   │   ├── lib.rs              Punto di ingresso app + registrazione comandi
│   │   ├── commands.rs         13 comandi Tauri
│   │   └── kv_store.rs         SQLite KV (rusqlite)
│   └── gen/android/        Overlay GeckoView
│       └── app/src/main/java/.../
│           ├── MainActivity.kt     GeckoView + bridge IPC
│           └── AssetServer.kt      Server assets localhost
├── server/                 Server Node.js (solo modalita server)
└── dist/                   Output build Vite → Assets APK
```

Basato su [tauri-geckoview-template](https://github.com/shittim-plana/tauri-geckoview-template)


## Comunita e contatti

- Segnalazione bug / richieste funzionalita: [GitHub Issues](https://github.com/shittim-plana/PocketRisu/issues)


## Licenza

[GPL-3.0](../LICENSE)
