<p align="center">
  <img src="../assets/pocketrisu-banner-1024.png" alt="PocketRisu — KI-Rollenspiel-Chat (Server & Android)" width="900" />
</p>

<h1 align="center">PocketRisu — KI-Rollenspiel-Chat (Server & Android)</h1>

<p align="center">
  <a href="../README.md">English</a> | <a href="README.ko.md">한국어</a> | <strong>Deutsch</strong> | <a href="README.es.md">Español</a> | <a href="README.fr.md">Français</a> | <a href="README.it.md">Italiano</a> | <a href="README.ja.md">日本語</a>
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

PocketRisu ist eine KI-Rollenspiel-Chat-Plattform, die als selbstgehosteter Webserver **oder** als eigenstaendige Android-App laeuft — kein Server erforderlich.

### Android APK (NEU)

Laden Sie die APK von den [Releases](https://github.com/shittim-plana/PocketRisu/releases) herunter und installieren Sie sie direkt auf Ihrem Android-Geraet. Die App laeuft vollstaendig lokal mit einer SQLite-Datenbank — fuegen Sie einfach Ihren KI-API-Schluessel hinzu und starten Sie den Chat.

- **Tauri 2.0** + **GeckoView 140 ESR** (Mozillas Browser-Engine)
- Nativer Rust-HTTP-Proxy fuer KI-Anbieter-Aufrufe
- Nur arm64-v8a (aarch64)

<p align="center">
  <table>
    <tr>
      <td align="center"><img src="../assets/screenshots/screenshot-pc-chat.png" alt="PC-Chat" height="420" /></td>
      <td align="center"><img src="../assets/screenshots/screenshot-mobile-chat.png" alt="Mobiler Chat" height="420" /></td>
    </tr>
    <tr>
      <td align="center"><b>PC</b></td>
      <td align="center"><b>Mobil</b></td>
    </tr>
  </table>
</p>


## Dokumentation

- [Installationsanleitung (Server)](../docs/de/install.md)
- [Android APK Anleitung](../docs/en/android-local-apk.md)
- [RisuAI-Migrationsleitfaden](../docs/de/migration.md)
- [Fernzugriff](../docs/de/remote.md)
- [Termux-Installationsanleitung](../docs/de/termux.md)


## RisuAI-Kompatibilitaet

PocketRisu ist von [RisuAI](https://github.com/kwaroran/RisuAI) abgeleitet und fuer selbstgehostete Umgebungen optimiert. Bestehende RisuAI-Daten koennen vollstaendig migriert werden, und alle RisuAI-Oekosystem-Assets bleiben unveraendert nutzbar.

- RisuRealm-Charakter-Downloads
- Charakterkarten (`.charx`, `.risum`, `.risup` usw.)
- Module, Lorebooks, Presets
- Backup-Dateien (`.bin`) mit bidirektionaler Kompatibilitaet

Fuer die Migration von einer bestehenden RisuAI-Installation siehe den [Migrationsleitfaden](../docs/de/migration.md).


## Funktionen

- **Mehrere KI-Anbieter**: OpenAI, Claude, Gemini, DeepInfra, OpenRouter, Ollama und mehr
- **Zwei Modi**: Selbstgehosteter Webserver (PC/Tablet/Smartphone ueber Browser) oder eigenstaendige Android-APK
- **Vereinheitlichte Datenspeicherung**: Alle Daten in einer einzigen SQLite-Datenbank — auf Ihrem Server oder auf dem Geraet
- **RisuRealm**: Charaktere im Risu-Charakter-Hub durchsuchen und herunterladen
- **Lorebook & Langzeitgedaechtnis**: World Info / Memory Book, HypaMemoryV3 und andere Kontext-Erhaltungsfunktionen
- **Automatische Uebersetzung**: Automatische Uebersetzung von Ein- und Ausgabe fuer sprachuebergreifendes Rollenspiel
- **Regex-Skripte & Plugins**: Plugin API 2.0 / 2.1 / 3.0 unterstuetzt
- **TTS & zusaetzliche Assets**: Sprachsynthese, eingebettete Bilder / Audio / Video im Chat
- **Server-Modus-Extras**: Dashboard, Backup/Wiederherstellung, Selbstaktualisierung, Quick Tunnel Fernzugriff
- **Mehrsprachige UI**: Koreanisch, Englisch, Japanisch, Chinesisch und mehr


## Architektur

PocketRisu laeuft in zwei Modi. Das Frontend ist gemeinsam; das Backend unterscheidet sich.

### Server-Modus (PC / Selbstgehostet)

```
Browser ──HTTP──▶ Node.js Server ──▶ SQLite DB
                       │
                       ├── /api/* (CRUD, Patch-Sync, Chat, Assets)
                       └── /proxy2 (KI-Anbieter-Relay)
```

### Android APK Modus

```
┌─ Android-App ──────────────────────────────────┐
│                                                │
│  GeckoView 140 ESR                             │
│    └── PocketRisu Svelte UI                    │
│           │                                    │
│           │ invoke()  (IPC WebExtension)        │
│           ▼                                    │
│  Tauri 2.0 / Rust                              │
│    ├── KV Store (SQLite, auf dem Geraet)       │
│    ├── proxy_request (KI-API-Relay)            │
│    └── streamed_fetch (Streaming + Events)     │
│                                                │
└────────────────────────────────────────────────┘
```

### Datenfluss — Chat-Nachricht

```
Benutzereingabe
  → Svelte UI
    → invoke('streamed_fetch', { url, headers, body })
      → Rust: reqwest → KI-Anbieter (OpenAI, Claude, usw.)
        ← SSE-Chunks → emit("streamed_fetch") Events
      ← ReadableStream → Chat-UI-Rendering
    → invoke('chat_content_save', { cha_id, chat_index, data })
      → SQLite KV Store (auf dem Geraet)
```

### Projektstruktur

```
PocketRisu/
├── src/                    Svelte Frontend (gemeinsam)
│   └── ts/storage/
│       ├── autoStorage.ts      Modus-Weiche (Server ↔ Tauri)
│       ├── nodeStorage.ts      Server-Modus: HTTP fetch
│       └── tauriStorage.ts     Android-Modus: Tauri invoke
├── src-tauri/              Tauri Rust Backend (Android)
│   ├── src/
│   │   ├── lib.rs              App-Einstiegspunkt + Kommando-Registrierung
│   │   ├── commands.rs         13 Tauri-Kommandos
│   │   └── kv_store.rs         SQLite KV (rusqlite)
│   └── gen/android/        GeckoView-Overlay
│       └── app/src/main/java/.../
│           ├── MainActivity.kt     GeckoView + IPC-Bruecke
│           └── AssetServer.kt      Localhost-Asset-Server
├── server/                 Node.js Server (nur Server-Modus)
└── dist/                   Vite Build-Ausgabe → APK-Assets
```

Basierend auf [tauri-geckoview-template](https://github.com/shittim-plana/tauri-geckoview-template)


## Community & Kontakt

- Fehlerberichte / Funktionswuensche: [GitHub Issues](https://github.com/shittim-plana/PocketRisu/issues)


## Lizenz

[GPL-3.0](../LICENSE)
