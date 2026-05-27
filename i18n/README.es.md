<p align="center">
  <img src="../assets/pocketrisu-banner-1024.png" alt="PocketRisu — Chat de Roleplay IA (Servidor & Android)" width="900" />
</p>

<h1 align="center">PocketRisu — Chat de Roleplay IA (Servidor & Android)</h1>

<p align="center">
  <a href="../README.md">English</a> | <a href="README.ko.md">한국어</a> | <a href="README.de.md">Deutsch</a> | <strong>Español</strong> | <a href="README.fr.md">Français</a> | <a href="README.it.md">Italiano</a> | <a href="README.ja.md">日本語</a>
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

PocketRisu es una plataforma de chat de roleplay con IA que funciona como servidor web autoalojado **o** como aplicacion Android independiente — sin necesidad de servidor.

### APK Android (NUEVO)

Descarga el APK desde [Releases](https://github.com/shittim-plana/PocketRisu/releases) e instalalo directamente en tu dispositivo Android. La aplicacion funciona completamente en el dispositivo con una base de datos SQLite local — solo agrega tu clave de API de IA y comienza a chatear.

- **Tauri 2.0** + **GeckoView 140 ESR** (motor de navegador de Mozilla)
- Proxy HTTP nativo en Rust para llamadas a proveedores de IA
- Solo arm64-v8a (aarch64)

<p align="center">
  <table>
    <tr>
      <td align="center"><img src="../assets/screenshots/screenshot-pc-chat.png" alt="Chat en PC" height="420" /></td>
      <td align="center"><img src="../assets/screenshots/screenshot-mobile-chat.png" alt="Chat movil" height="420" /></td>
    </tr>
    <tr>
      <td align="center"><b>PC</b></td>
      <td align="center"><b>Movil</b></td>
    </tr>
  </table>
</p>


## Documentacion

- [Guia de instalacion (Servidor)](../docs/es/install.md)
- [Guia de APK Android](../docs/en/android-local-apk.md)
- [Guia de migracion desde RisuAI](../docs/es/migration.md)
- [Acceso remoto](../docs/es/remote.md)
- [Guia de instalacion de Termux](../docs/es/termux.md)


## Compatibilidad con RisuAI

PocketRisu deriva de [RisuAI](https://github.com/kwaroran/RisuAI) y esta optimizado para entornos autoalojados. Los datos existentes de RisuAI se pueden migrar completamente, y todos los recursos del ecosistema de RisuAI siguen siendo utilizables tal cual.

- Descargas de personajes de RisuRealm
- Tarjetas de personaje (`.charx`, `.risum`, `.risup`, etc.)
- Modulos, lorebooks, presets
- Archivos de copia de seguridad (`.bin`) con compatibilidad bidireccional

Para migrar desde una instalacion existente de RisuAI, consulta la [guia de migracion](../docs/es/migration.md).


## Funciones

- **Multiples proveedores de IA**: OpenAI, Claude, Gemini, DeepInfra, OpenRouter, Ollama y mas
- **Dos modos**: Servidor web autoalojado (PC/tablet/smartphone via navegador) o APK Android independiente
- **Almacenamiento unificado**: Todos los datos en una sola base de datos SQLite — en tu servidor o en el dispositivo
- **RisuRealm**: Explorar y descargar personajes desde el hub de personajes de Risu
- **Lorebook y memoria a largo plazo**: World info / memory book, HypaMemoryV3 y otras funciones de retencion de contexto
- **Traduccion automatica**: Traduccion automatica de entrada y salida para roleplay multilingue
- **Scripts regex y plugins**: Plugin API 2.0 / 2.1 / 3.0 soportados
- **TTS y recursos adicionales**: Sintesis de voz, imagenes / audio / video integrados en el chat
- **Extras del modo servidor**: Panel de control, copia de seguridad/restauracion, actualizacion automatica, acceso remoto Quick Tunnel
- **Interfaz multilingue**: Coreano, ingles, japones, chino y mas


## Arquitectura

PocketRisu funciona en dos modos. El frontend es compartido; el backend difiere.

### Modo Servidor (PC / Autoalojado)

```
Navegador ──HTTP──▶ Servidor Node.js ──▶ SQLite DB
                          │
                          ├── /api/* (CRUD, sincronizacion de parches, chat, assets)
                          └── /proxy2 (relay de proveedor de IA)
```

### Modo APK Android

```
┌─ Aplicacion Android ───────────────────────────┐
│                                                │
│  GeckoView 140 ESR                             │
│    └── PocketRisu Svelte UI                    │
│           │                                    │
│           │ invoke()  (IPC WebExtension)        │
│           ▼                                    │
│  Tauri 2.0 / Rust                              │
│    ├── KV Store (SQLite, en el dispositivo)    │
│    ├── proxy_request (relay de API IA)         │
│    └── streamed_fetch (streaming + eventos)    │
│                                                │
└────────────────────────────────────────────────┘
```

### Flujo de datos — Mensaje de chat

```
Entrada del usuario
  → Svelte UI
    → invoke('streamed_fetch', { url, headers, body })
      → Rust: reqwest → Proveedor de IA (OpenAI, Claude, etc.)
        ← Fragmentos SSE → emit("streamed_fetch") eventos
      ← ReadableStream → Renderizado del chat
    → invoke('chat_content_save', { cha_id, chat_index, data })
      → SQLite KV Store (en el dispositivo)
```

### Estructura del proyecto

```
PocketRisu/
├── src/                    Frontend Svelte (compartido)
│   └── ts/storage/
│       ├── autoStorage.ts      Selector de modo (Servidor ↔ Tauri)
│       ├── nodeStorage.ts      Modo servidor: HTTP fetch
│       └── tauriStorage.ts     Modo Android: Tauri invoke
├── src-tauri/              Backend Rust Tauri (Android)
│   ├── src/
│   │   ├── lib.rs              Entrada de la app + registro de comandos
│   │   ├── commands.rs         13 comandos Tauri
│   │   └── kv_store.rs         SQLite KV (rusqlite)
│   └── gen/android/        Overlay GeckoView
│       └── app/src/main/java/.../
│           ├── MainActivity.kt     GeckoView + puente IPC
│           └── AssetServer.kt      Servidor de assets localhost
├── server/                 Servidor Node.js (solo modo servidor)
└── dist/                   Salida de build Vite → Assets APK
```

Basado en [tauri-geckoview-template](https://github.com/shittim-plana/tauri-geckoview-template)


## Comunidad y contacto

- Reportes de errores / solicitudes de funciones: [GitHub Issues](https://github.com/shittim-plana/PocketRisu/issues)


## Licencia

[GPL-3.0](../LICENSE)
