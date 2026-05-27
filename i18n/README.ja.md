<p align="center">
  <img src="../assets/pocketrisu-banner-1024.png" alt="PocketRisu — AIロールプレイチャット（サーバー & Android）" width="900" />
</p>

<h1 align="center">PocketRisu — AIロールプレイチャット（サーバー & Android）</h1>

<p align="center">
  <a href="../README.md">English</a> | <a href="README.ko.md">한국어</a> | <a href="README.de.md">Deutsch</a> | <a href="README.es.md">Español</a> | <a href="README.fr.md">Français</a> | <a href="README.it.md">Italiano</a> | <strong>日本語</strong>
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

PocketRisuは、セルフホスティングWebサーバー**または**スタンドアロンAndroidアプリとして動作するAIロールプレイチャットプラットフォームです。サーバー不要で端末単体で実行できます。

### Android APK（NEW）

[Releases](https://github.com/shittim-plana/PocketRisu/releases)からAPKをダウンロードし、Androidデバイスに直接インストールしてください。ローカルSQLiteデータベースで端末上で完全に動作します — AI APIキーを設定するだけでチャットを開始できます。

- **Tauri 2.0** + **GeckoView 140 ESR**（Mozillaブラウザエンジン）
- Rust製ネイティブHTTPプロキシ
- arm64-v8a（aarch64）専用

<p align="center">
  <table>
    <tr>
      <td align="center"><img src="../assets/screenshots/screenshot-pc-chat.png" alt="PCチャット" height="420" /></td>
      <td align="center"><img src="../assets/screenshots/screenshot-mobile-chat.png" alt="モバイルチャット" height="420" /></td>
    </tr>
    <tr>
      <td align="center"><b>PC</b></td>
      <td align="center"><b>モバイル</b></td>
    </tr>
  </table>
</p>


## ドキュメント

- [インストールガイド（サーバー）](../docs/en/install.md)
- [Android APKガイド](../docs/en/android-local-apk.md)
- [RisuAI移行ガイド](../docs/en/migration.md)


## RisuAI互換性

PocketRisuは[RisuAI](https://github.com/kwaroran/RisuAI)から派生し、セルフホスティング環境向けに最適化されたプロジェクトです。既存のRisuAIデータはそのまま移行でき、RisuAIエコシステムのすべてのアセットが利用可能です。

- RisuRealmキャラクターダウンロード
- キャラクターカード（`.charx`、`.risum`、`.risup`など）
- モジュール、ロアブック、プリセット
- バックアップファイル（`.bin`）双方向互換


## 主な機能

- **多数のAIプロバイダー**: OpenAI、Claude、Gemini、DeepInfra、OpenRouter、Ollamaなど
- **2つのモード**: セルフホスティングWebサーバー（PC/タブレット/スマートフォンのブラウザ経由）またはAndroid単体APK
- **統合データストレージ**: すべてのデータをSQLiteデータベース1つに保存 — サーバー上または端末内
- **RisuRealm**: キャラクターハブからキャラクターを検索・ダウンロード
- **ロアブック・長期記憶**: World Info / Memory Book、HypaMemoryV3などのコンテキスト保持機能
- **自動翻訳**: 入出力の自動翻訳
- **正規表現スクリプト・プラグイン**: Plugin API 2.0 / 2.1 / 3.0対応
- **TTS・追加アセット**: 音声合成、チャット内の画像・音声・動画埋め込み
- **サーバーモード限定**: ダッシュボード、バックアップ/復元、自動更新、Quick Tunnelリモートアクセス
- **多言語UI**: 韓国語、英語、日本語など


## アーキテクチャ

### サーバーモード（PC / セルフホスティング）

```
ブラウザ ──HTTP──▶ Node.jsサーバー ──▶ SQLite DB
                       │
                       ├── /api/*（CRUD、パッチ同期、チャット、アセット）
                       └── /proxy2（AI APIリレー）
```

### Android APKモード

```
┌─ Androidアプリ ─────────────────────────────┐
│                                              │
│  GeckoView 140 ESR                           │
│    └── PocketRisu Svelte UI                  │
│           │                                  │
│           │ invoke()（IPC WebExtension）       │
│           ▼                                  │
│  Tauri 2.0 / Rust                            │
│    ├── KV Store（SQLite、端末内保存）          │
│    ├── proxy_request（AI APIリレー）          │
│    └── streamed_fetch（ストリーミング＋イベント）│
│                                              │
└──────────────────────────────────────────────┘
```

### データフロー — チャットメッセージ

```
ユーザー入力
  → Svelte UI
    → invoke('streamed_fetch', { url, headers, body })
      → Rust: reqwest → AIプロバイダー（OpenAI、Claudeなど）
        ← SSEチャンク → emit("streamed_fetch")イベント
      ← ReadableStream → チャットUI描画
    → invoke('chat_content_save', { cha_id, chat_index, data })
      → SQLite KVストア（端末内）
```

### プロジェクト構造

```
PocketRisu/
├── src/                    Svelteフロントエンド（共通）
│   └── ts/storage/
│       ├── autoStorage.ts      モード分岐（サーバー ↔ Tauri）
│       ├── nodeStorage.ts      サーバーモード: HTTP fetch
│       └── tauriStorage.ts     Androidモード: Tauri invoke
├── src-tauri/              Tauri Rustバックエンド（Android）
│   ├── src/
│   │   ├── lib.rs              アプリエントリ＋コマンド登録
│   │   ├── commands.rs         13個のTauriコマンド
│   │   └── kv_store.rs         SQLite KV（rusqlite）
│   └── gen/android/        GeckoViewオーバーレイ
│       └── app/src/main/java/.../
│           ├── MainActivity.kt     GeckoView + IPCブリッジ
│           └── AssetServer.kt      localhostアセットサーバー
├── server/                 Node.jsサーバー（サーバーモード専用）
└── dist/                   Viteビルド出力 → APKアセット
```


ベーステンプレート: [tauri-geckoview-template](https://github.com/shittim-plana/tauri-geckoview-template)


## コミュニティ & お問い合わせ

- バグ報告 / 機能リクエスト: [GitHub Issues](https://github.com/shittim-plana/PocketRisu/issues)


## ライセンス

[GPL-3.0](../LICENSE)
