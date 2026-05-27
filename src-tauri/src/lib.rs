mod commands;
mod kv_store;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .setup(|app| {
            let data_dir = app.path().app_data_dir().expect("failed to resolve app data dir");
            let store = kv_store::KvStore::open(&data_dir)
                .expect("failed to open KV store");
            app.manage(store);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::proxy_request,
            commands::streamed_fetch,
            commands::kv_read,
            commands::kv_write,
            commands::kv_list,
            commands::kv_remove,
            commands::chat_content_load,
            commands::chat_content_save,
            commands::asset_get,
            commands::assets_bulk_read,
            commands::assets_bulk_write,
            commands::db_flush,
            commands::crypto_hash,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
