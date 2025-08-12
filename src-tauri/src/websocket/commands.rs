use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{AppHandle, State};
use serde_json::Value;

use super::client::{WebSocketClient, WebSocketMessage};

type WebSocketClients = Arc<Mutex<HashMap<String, Arc<WebSocketClient>>>>;

#[tauri::command]
pub async fn websocket_connect(
    app_handle: AppHandle,
    clients: State<'_, WebSocketClients>,
    connection_id: String,
    uri: String,
) -> Result<(), String> {
    let client = Arc::new(WebSocketClient::new(app_handle.clone()));
    
    client.connect(&uri).await.map_err(|e| e.to_string())?;
    
    let mut clients_guard = clients.lock().await;
    clients_guard.insert(connection_id.clone(), client.clone());
    
    // Start listening for messages in a background task
    let client_clone = client.clone();
    let connection_id_clone = connection_id.clone();
    let clients_clone = clients.inner().clone();
    
    tokio::spawn(async move {
        if let Err(e) = client_clone.listen_for_messages().await {
            eprintln!("WebSocket listening error for {}: {}", connection_id_clone, e);
            // Remove the client from the map when connection fails
            let mut clients_guard = clients_clone.lock().await;
            clients_guard.remove(&connection_id_clone);
        }
    });
    
    Ok(())
}

#[tauri::command]
pub async fn websocket_send_message(
    clients: State<'_, WebSocketClients>,
    connection_id: String,
    msg_id: String,
    verb: String,
    path: String,
    payload: Value,
) -> Result<(), String> {
    let clients_guard = clients.lock().await;
    
    if let Some(client) = clients_guard.get(&connection_id) {
        let message = WebSocketMessage {
            msg_id,
            verb,
            path,
            payload,
        };
        
        client.send_message(message).await.map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("WebSocket connection not found".to_string())
    }
}

#[tauri::command]
pub async fn websocket_disconnect(
    clients: State<'_, WebSocketClients>,
    connection_id: String,
) -> Result<(), String> {
    let mut clients_guard = clients.lock().await;
    
    if let Some(client) = clients_guard.remove(&connection_id) {
        client.disconnect().await.map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("WebSocket connection not found".to_string())
    }
}

#[tauri::command]
pub async fn websocket_list_connections(
    clients: State<'_, WebSocketClients>,
) -> Result<Vec<String>, String> {
    let clients_guard = clients.lock().await;
    Ok(clients_guard.keys().cloned().collect())
}

/// Initialize the WebSocket clients state
pub fn init_websocket_state() -> WebSocketClients {
    Arc::new(Mutex::new(HashMap::new()))
}