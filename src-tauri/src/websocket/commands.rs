use std::sync::Arc;
use tauri::State;
use serde_json::Value;

use super::client::{WebSocketClient, WebSocketMessage};

#[tauri::command]
pub async fn ws_connect(
    ws_client: State<'_, Arc<WebSocketClient>>,
    uri: String,
) -> Result<(), String> {
    ws_client.connect(&uri).await.map_err(|e| e.to_string())?;
    
    // Start listening for messages in a background task
    let client_clone = ws_client.inner().clone();
    tokio::spawn(async move {
        if let Err(e) = client_clone.listen_for_messages().await {
            eprintln!("WebSocket listening error: {}", e);
        }
    });
    
    Ok(())
}

#[tauri::command]
pub async fn ws_send_message(
    ws_client: State<'_, Arc<WebSocketClient>>,
    verb: String,
    path: String,
    payload: Value,
) -> Result<(), String> {
    eprintln!("[WebSocket] Sending message: verb={}, path={}", verb, path);

    let message = WebSocketMessage {
        verb: verb.clone(),
        path: path.clone(),
        payload,
    };

    match ws_client.send_message(message).await {
        Ok(_) => {
            eprintln!("[WebSocket] Message sent successfully: verb={}", verb);
            Ok(())
        }
        Err(e) => {
            eprintln!("[WebSocket] Failed to send message: verb={}, error={}", verb, e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn ws_disconnect(
    ws_client: State<'_, Arc<WebSocketClient>>,
) -> Result<(), String> {
    ws_client.disconnect().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ws_is_connected(
    ws_client: State<'_, Arc<WebSocketClient>>,
) -> Result<bool, String> {
    Ok(ws_client.is_connected())
}

