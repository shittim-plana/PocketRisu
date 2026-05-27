<p align="center">
  <img src="../assets/pocketrisu-banner-1024.png" alt="PocketRisu — AI 롤플레이 채팅 (서버 & 안드로이드)" width="900" />
</p>

<h1 align="center">PocketRisu — AI 롤플레이 채팅 (서버 & 안드로이드)</h1>

<p align="center">
  <a href="../README.md">English</a> | <strong>한국어</strong> | <a href="README.de.md">Deutsch</a> | <a href="README.es.md">Español</a> | <a href="README.vi.md">Tiếng Việt</a> | <a href="README.fr.md">Français</a>
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

PocketRisu는 셀프호스팅 웹 서버 **또는** 안드로이드 단독 앱으로 동작하는 AI 롤플레이 채팅 플랫폼입니다. 서버 없이 기기에서 바로 실행할 수 있습니다.

### 안드로이드 APK (NEW)

[Releases](https://github.com/shittim-plana/PocketRisu/releases)에서 APK를 다운로드해 설치하세요. 로컬 SQLite DB로 기기에서 독립 실행됩니다 — AI API 키만 넣으면 바로 채팅할 수 있습니다.

- **Tauri 2.0** + **GeckoView 140 ESR** (Mozilla 브라우저 엔진)
- Rust 네이티브 HTTP 프록시
- arm64-v8a (aarch64) 전용

<p align="center">
  <table>
    <tr>
      <td align="center"><img src="../assets/screenshots/screenshot-pc-chat.png" alt="PC 채팅" height="420" /></td>
      <td align="center"><img src="../assets/screenshots/screenshot-mobile-chat.png" alt="모바일 채팅" height="420" /></td>
    </tr>
    <tr>
      <td align="center"><b>PC</b></td>
      <td align="center"><b>모바일</b></td>
    </tr>
  </table>
</p>


## 문서

- [설치 가이드 (서버)](../docs/ko/install.md)
- [안드로이드 APK 가이드](../docs/en/android-local-apk.md)
- [RisuAI 데이터 이전 가이드](../docs/ko/migration.md)
- [원격 접속 가이드](../docs/ko/remote.md)
- [Termux 설치 가이드](../docs/ko/termux.md)


## RisuAI 호환

PocketRisu는 [RisuAI](https://github.com/kwaroran/RisuAI)에서 파생되어, 셀프호스팅 환경에 맞게 개선한 프로젝트입니다. 기존 데이터를 통째로 마이그레이션할 수 있고, RisuAI 생태계 자산을 그대로 사용할 수 있습니다.

- RisuRealm 캐릭터 다운로드
- 캐릭터 카드 (`.charx`, `.risum`, `.risup` 등)
- 모듈, 로어북, 프리셋
- 백업 파일 (`.bin`) 양방향 호환


## 주요 기능

- **다양한 AI 지원**: OpenAI, Claude, Gemini, DeepInfra, OpenRouter, Ollama 등
- **두 가지 모드**: 셀프호스팅 웹 서버 (PC·태블릿·스마트폰 브라우저) 또는 안드로이드 단독 APK
- **데이터 통합 저장**: 모든 데이터를 SQLite DB 하나에 보관 — 서버 또는 기기 내
- **RisuRealm**: 캐릭터 허브에서 캐릭터 검색 및 다운로드
- **로어북·장기 메모리**: 세계관/메모리 북, HypaMemoryV3 등 컨텍스트 유지
- **자동 번역**: 입력/출력 자동 번역
- **정규식 스크립트·플러그인**: Plugin API 2.0 / 2.1 / 3.0 지원
- **TTS·추가 에셋**: 음성 합성, 채팅 내 이미지·오디오·비디오
- **서버 모드 전용**: 대시보드, 백업/복원, 셀프 업데이트, Quick Tunnel 원격 접속
- **다국어 UI**: 한국어, 영어, 일본어 등


## 아키텍처

### 서버 모드 (PC / 셀프호스팅)

```
브라우저 ──HTTP──▶ Node.js 서버 ──▶ SQLite DB
                       │
                       ├── /api/* (CRUD, 패치, 채팅, 에셋)
                       └── /proxy2 (AI API 중계)
```

### 안드로이드 APK 모드

```
┌─ Android 앱 ────────────────────────────────┐
│                                              │
│  GeckoView 140 ESR                           │
│    └── PocketRisu Svelte UI                  │
│           │                                  │
│           │ invoke()  (IPC WebExtension)      │
│           ▼                                  │
│  Tauri 2.0 / Rust                            │
│    ├── KV Store (SQLite, 기기 내 저장)        │
│    ├── proxy_request (AI API 중계)           │
│    └── streamed_fetch (스트리밍 + 이벤트)     │
│                                              │
└──────────────────────────────────────────────┘
```

### 프로젝트 구조

```
PocketRisu/
├── src/                    Svelte 프론트엔드 (공용)
│   └── ts/storage/
│       ├── autoStorage.ts      모드 분기 (서버 ↔ Tauri)
│       ├── nodeStorage.ts      서버 모드: HTTP fetch
│       └── tauriStorage.ts     안드로이드 모드: Tauri invoke
├── src-tauri/              Tauri Rust 백엔드 (안드로이드)
│   ├── src/
│   │   ├── lib.rs              앱 진입점 + 커맨드 등록
│   │   ├── commands.rs         13개 Tauri 커맨드
│   │   └── kv_store.rs         SQLite KV (rusqlite)
│   └── gen/android/        GeckoView 오버레이
│       └── app/src/main/java/.../
│           ├── MainActivity.kt     GeckoView + IPC 브릿지
│           └── AssetServer.kt      localhost 에셋 서버
├── server/                 Node.js 서버 (서버 모드 전용)
└── dist/                   Vite 빌드 출력 → APK 에셋
```


## 커뮤니티 & 연락처

- 버그 리포트 / 기능 제안: [GitHub Issues](https://github.com/shittim-plana/PocketRisu/issues)


## 라이선스

[GPL-3.0](../LICENSE)
