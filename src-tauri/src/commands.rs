use base64::{engine::general_purpose, Engine as _};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

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

    let client = reqwest::Client::new();
    let builder = match method.as_str() {
        "POST" => {
            let decoded = general_purpose::STANDARD
                .decode(body.as_bytes())
                .unwrap_or_default();
            client.post(&url).headers(parsed_headers).body(decoded)
        }
        "PUT" => {
            let decoded = general_purpose::STANDARD
                .decode(body.as_bytes())
                .unwrap_or_default();
            client.put(&url).headers(parsed_headers).body(decoded)
        }
        "DELETE" => {
            let decoded = general_purpose::STANDARD
                .decode(body.as_bytes())
                .unwrap_or_default();
            client.delete(&url).headers(parsed_headers).body(decoded)
        }
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
