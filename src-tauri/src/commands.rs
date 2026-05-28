use base64::{engine::general_purpose, Engine as _};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::Duration;
use tauri::{AppHandle, Emitter, State};

use crate::kv_store::KvStore;

fn parse_headers(header_json: &Value) -> Result<HeaderMap, String> {
    let obj = header_json
        .as_object()
        .ok_or_else(|| "invalid header JSON".to_string())?;
    let mut headers = HeaderMap::new();
    for (key, value) in obj {
        let name = HeaderName::from_bytes(key.as_bytes()).map_err(|e| e.to_string())?;
        let val = HeaderValue::from_str(value.as_str().unwrap_or("")).map_err(|e| e.to_string())?;
        headers.insert(name, val);
    }
    Ok(headers)
}

fn header_map_to_json(header_map: &HeaderMap) -> Value {
    let mut map = HashMap::new();
    for (key, value) in header_map {
        map.insert(
            key.as_str().to_string(),
            value.to_str().unwrap_or("").to_string(),
        );
    }
    json!(map)
}

// ── Proxy commands ──────────────────────────────────────────────────────────

#[tauri::command]
pub async fn proxy_request(
    url: String,
    body: String,
    header: String,
    method: String,
) -> String {
    let headers_json: Value = match serde_json::from_str(&header) {
        Ok(h) => h,
        Err(e) => return json!({"success": false, "body": e.to_string()}).to_string(),
    };

    let headers = match parse_headers(&headers_json) {
        Ok(h) => h,
        Err(e) => return json!({"success": false, "body": e}).to_string(),
    };

    let client = reqwest::Client::new();
    let builder = match method.as_str() {
        "POST" => client.post(&url).headers(headers).body(body),
        "PUT" => client.put(&url).headers(headers).body(body),
        "DELETE" => client.delete(&url).headers(headers).body(body),
        _ => client.get(&url).headers(headers),
    };

    match builder.timeout(Duration::from_secs(120)).send().await {
        Ok(resp) => {
            let resp_headers = header_map_to_json(resp.headers());
            let status = resp.status().as_u16();
            match resp.bytes().await {
                Ok(bytes) => {
                    let encoded = general_purpose::STANDARD.encode(&bytes);
                    json!({
                        "success": true,
                        "body": encoded,
                        "headers": resp_headers,
                        "status": status
                    })
                    .to_string()
                }
                Err(e) => json!({"success": false, "body": e.to_string()}).to_string(),
            }
        }
        Err(e) => json!({"success": false, "body": e.to_string(), "status": 400}).to_string(),
    }
}

#[tauri::command]
pub async fn streamed_fetch(
    id: String,
    url: String,
    headers: String,
    body: String,
    method: String,
    app: AppHandle,
) -> String {
    let headers_json: Value = match serde_json::from_str(&headers) {
        Ok(h) => h,
        Err(e) => return json!({"success": false, "body": e.to_string()}).to_string(),
    };

    let parsed_headers = match parse_headers(&headers_json) {
        Ok(h) => h,
        Err(e) => return json!({"success": false, "body": e}).to_string(),
    };

    let decoded_body = if !body.is_empty() {
        match general_purpose::STANDARD.decode(body.as_bytes()) {
            Ok(b) => Some(b),
            Err(e) => return json!({"success": false, "body": format!("base64 decode error: {}", e)}).to_string(),
        }
    } else {
        None
    };

    let client = reqwest::Client::new();
    let builder = match method.as_str() {
        "POST" => client.post(&url).headers(parsed_headers).body(decoded_body.unwrap_or_default()),
        "PUT" => client.put(&url).headers(parsed_headers).body(decoded_body.unwrap_or_default()),
        "DELETE" => client.delete(&url).headers(parsed_headers).body(decoded_body.unwrap_or_default()),
        _ => client.get(&url).headers(parsed_headers),
    };

    match builder.timeout(Duration::from_secs(240)).send().await {
        Ok(mut resp) => {
            let resp_headers = header_map_to_json(resp.headers());
            let status = resp.status().as_u16();

            let _ = app.emit(
                "streamed_fetch",
                json!({
                    "type": "headers",
                    "body": resp_headers,
                    "id": id,
                    "status": status
                })
                .to_string(),
            );

            loop {
                match resp.chunk().await {
                    Ok(Some(chunk)) => {
                        let encoded = general_purpose::STANDARD.encode(&chunk);
                        let _ = app.emit(
                            "streamed_fetch",
                            json!({
                                "type": "chunk",
                                "body": encoded,
                                "id": id
                            })
                            .to_string(),
                        );
                    }
                    Ok(None) => break,
                    Err(e) => {
                        return json!({"success": false, "body": e.to_string()}).to_string();
                    }
                }
            }

            let _ = app.emit(
                "streamed_fetch",
                json!({"type": "end", "id": id}).to_string(),
            );

            json!({"success": true}).to_string()
        }
        Err(e) => json!({"success": false, "body": e.to_string()}).to_string(),
    }
}

// ── KV CRUD commands ────────────────────────────────────────────────────────

#[tauri::command]
pub fn kv_read(store: State<'_, KvStore>, file_path: String) -> Result<Option<String>, String> {
    match store.get(&file_path)? {
        Some(data) => Ok(Some(general_purpose::STANDARD.encode(&data))),
        None => Ok(None),
    }
}

#[tauri::command]
pub fn kv_write(store: State<'_, KvStore>, file_path: String, data: String) -> Result<(), String> {
    let bytes = general_purpose::STANDARD
        .decode(data.as_bytes())
        .map_err(|e| format!("base64 decode error: {}", e))?;
    store.set(&file_path, &bytes)
}

#[tauri::command]
pub fn kv_list(store: State<'_, KvStore>, key_prefix: Option<String>) -> Result<Vec<String>, String> {
    store.list(key_prefix.as_deref())
}

#[tauri::command]
pub fn kv_remove(store: State<'_, KvStore>, file_path: String) -> Result<(), String> {
    store.delete(&file_path)
}

// ── Chat content commands ───────────────────────────────────────────────────

#[tauri::command]
pub fn chat_content_load(
    store: State<'_, KvStore>,
    cha_id: String,
    chat_index: String,
) -> Result<Option<String>, String> {
    let key = format!("chat-content/{}/{}", cha_id, chat_index);
    match store.get(&key)? {
        Some(data) => Ok(Some(general_purpose::STANDARD.encode(&data))),
        None => Ok(None),
    }
}

#[tauri::command]
pub fn chat_content_save(
    store: State<'_, KvStore>,
    cha_id: String,
    chat_index: String,
    data: String,
) -> Result<(), String> {
    let key = format!("chat-content/{}/{}", cha_id, chat_index);
    let bytes = general_purpose::STANDARD
        .decode(data.as_bytes())
        .map_err(|e| format!("base64 decode error: {}", e))?;
    store.set(&key, &bytes)
}

// ── Asset commands ──────────────────────────────────────────────────────────

#[tauri::command]
pub fn asset_get(store: State<'_, KvStore>, hex_key: String) -> Result<Option<String>, String> {
    match store.get(&hex_key)? {
        Some(data) => Ok(Some(general_purpose::STANDARD.encode(&data))),
        None => Ok(None),
    }
}

#[tauri::command]
pub fn assets_bulk_read(
    store: State<'_, KvStore>,
    keys: Vec<String>,
) -> Result<Vec<Option<String>>, String> {
    let mut results = Vec::with_capacity(keys.len());
    for key in &keys {
        match store.get(key)? {
            Some(data) => results.push(Some(general_purpose::STANDARD.encode(&data))),
            None => results.push(None),
        }
    }
    Ok(results)
}

#[derive(Deserialize)]
pub struct BulkWriteItem {
    pub key: String,
    pub value: String,
}

#[tauri::command]
pub fn assets_bulk_write(
    store: State<'_, KvStore>,
    items: Vec<BulkWriteItem>,
) -> Result<(), String> {
    for item in &items {
        let bytes = general_purpose::STANDARD
            .decode(item.value.as_bytes())
            .map_err(|e| format!("base64 decode error for key {}: {}", item.key, e))?;
        store.set(&item.key, &bytes)?;
    }
    Ok(())
}

// ── DB operations ───────────────────────────────────────────────────────────

#[tauri::command]
pub fn db_flush(store: State<'_, KvStore>) -> Result<(), String> {
    store.checkpoint_wal()
}

#[tauri::command]
pub fn crypto_hash(data: String) -> Result<String, String> {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    Ok(format!("{:x}", hasher.finalize()))
}
