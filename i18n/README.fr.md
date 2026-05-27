<p align="center">
  <img src="../assets/pocketrisu-banner-1024.png" alt="PocketRisu — Chat RP IA (Serveur & Android)" width="900" />
</p>

<h1 align="center">PocketRisu — Chat RP IA (Serveur & Android)</h1>

<p align="center">
  <a href="../README.md">English</a> | <a href="README.ko.md">한국어</a> | <a href="README.de.md">Deutsch</a> | <a href="README.es.md">Español</a> | <strong>Français</strong> | <a href="README.it.md">Italiano</a> | <a href="README.ja.md">日本語</a>
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

PocketRisu est une plateforme de chat RP IA qui fonctionne en tant que serveur web auto-heberge **ou** en tant qu'application Android autonome — aucun serveur requis.

### APK Android (NOUVEAU)

Telechargez l'APK depuis les [Releases](https://github.com/shittim-plana/PocketRisu/releases) et installez-le directement sur votre appareil Android. L'application fonctionne entierement en local avec une base SQLite — ajoutez simplement votre cle API IA et commencez a discuter.

- **Tauri 2.0** + **GeckoView 140 ESR** (moteur de navigateur Mozilla)
- Proxy HTTP natif Rust pour les appels API IA
- arm64-v8a (aarch64) uniquement

<p align="center">
  <table>
    <tr>
      <td align="center"><img src="../assets/screenshots/screenshot-pc-chat.png" alt="Chat PC" height="420" /></td>
      <td align="center"><img src="../assets/screenshots/screenshot-mobile-chat.png" alt="Chat mobile" height="420" /></td>
    </tr>
    <tr>
      <td align="center"><b>PC</b></td>
      <td align="center"><b>Mobile</b></td>
    </tr>
  </table>
</p>


## Documentation

- [Guide d'installation (Serveur)](../docs/en/install.md)
- [Guide APK Android](../docs/en/android-local-apk.md)
- [Guide de migration RisuAI](../docs/en/migration.md)


## Compatibilite RisuAI

PocketRisu est derive de [RisuAI](https://github.com/kwaroran/RisuAI) et optimise pour l'auto-hebergement. Les donnees RisuAI existantes peuvent etre migrees integralement.

- Telechargement de personnages RisuRealm
- Cartes de personnage (`.charx`, `.risum`, `.risup`, etc.)
- Modules, lorebooks, presets
- Fichiers de sauvegarde (`.bin`) avec compatibilite bidirectionnelle


## Fonctionnalites

- **Multiples fournisseurs IA** : OpenAI, Claude, Gemini, DeepInfra, OpenRouter, Ollama, etc.
- **Deux modes** : Serveur web auto-heberge (PC/tablette/smartphone via navigateur) ou APK Android autonome
- **Stockage unifie** : Toutes les donnees dans une seule base SQLite — sur votre serveur ou sur l'appareil
- **RisuRealm** : Parcourir et telecharger des personnages depuis le hub
- **Lorebook & memoire long-terme** : World info, HypaMemoryV3, et autres fonctions de retention de contexte
- **Traduction automatique** : Traduction auto des entrees/sorties
- **Scripts regex & plugins** : Plugin API 2.0 / 2.1 / 3.0 supportes
- **TTS & medias** : Synthese vocale, images/audio/video integres dans le chat
- **Extras mode serveur** : Tableau de bord, sauvegarde/restauration, mise a jour automatique, acces distant Quick Tunnel
- **Interface multilingue** : Coreen, anglais, japonais, et plus


## Architecture

PocketRisu fonctionne en deux modes. Le frontend est partage ; le backend differe.

### Mode Serveur (PC / Auto-heberge)

```
Navigateur ──HTTP──▶ Serveur Node.js ──▶ SQLite DB
                          │
                          ├── /api/* (CRUD, patch, chat, assets)
                          └── /proxy2 (relais API IA)
```

### Mode APK Android

```
┌─ Application Android ───────────────────────┐
│                                              │
│  GeckoView 140 ESR                           │
│    └── PocketRisu Svelte UI                  │
│           │                                  │
│           │ invoke()  (IPC WebExtension)      │
│           ▼                                  │
│  Tauri 2.0 / Rust                            │
│    ├── KV Store (SQLite, sur l'appareil)     │
│    ├── proxy_request (relais API IA)         │
│    └── streamed_fetch (streaming + events)   │
│                                              │
└──────────────────────────────────────────────┘
```

### Flux de donnees — Message de chat

```
Saisie utilisateur
  → Svelte UI
    → invoke('streamed_fetch', { url, headers, body })
      → Rust: reqwest → Fournisseur IA (OpenAI, Claude, etc.)
        ← Fragments SSE → emit("streamed_fetch") evenements
      ← ReadableStream → Rendu de l'interface chat
    → invoke('chat_content_save', { cha_id, chat_index, data })
      → SQLite KV Store (sur l'appareil)
```

### Structure du projet

```
PocketRisu/
├── src/                    Frontend Svelte (partage)
│   └── ts/storage/
│       ├── autoStorage.ts      Selecteur de mode (Serveur ↔ Tauri)
│       ├── nodeStorage.ts      Mode serveur : HTTP fetch
│       └── tauriStorage.ts     Mode Android : Tauri invoke
├── src-tauri/              Backend Rust Tauri (Android)
│   ├── src/
│   │   ├── lib.rs              Point d'entree + enregistrement des commandes
│   │   ├── commands.rs         13 commandes Tauri
│   │   └── kv_store.rs         SQLite KV (rusqlite)
│   └── gen/android/        Overlay GeckoView
│       └── app/src/main/java/.../
│           ├── MainActivity.kt     GeckoView + pont IPC
│           └── AssetServer.kt      Serveur d'assets localhost
├── server/                 Serveur Node.js (mode serveur uniquement)
└── dist/                   Sortie build Vite → Assets APK
```

Base sur [tauri-geckoview-template](https://github.com/shittim-plana/tauri-geckoview-template)


## Communaute & Contact

- Rapports de bugs / demandes de fonctionnalites : [GitHub Issues](https://github.com/shittim-plana/PocketRisu/issues)


## Licence

[GPL-3.0](../LICENSE)
