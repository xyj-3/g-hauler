use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream};
use tokio_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub msg_id: String,
    pub verb: String,
    pub path: String,
    pub payload: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketResponse {
    pub msg_id: String,
    pub status: String,
    pub payload: Value,
}

pub struct WebSocketClient {
    pub ws_stream: Arc<Mutex<Option<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>>>,
    pub app_handle: AppHandle,
}

impl WebSocketClient {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            ws_stream: Arc::new(Mutex::new(None)),
            app_handle,
        }
    }

    pub async fn connect(&self, uri: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let request = tokio_tungstenite::tungstenite::handshake::client::Request::builder()
            .uri(uri)
            .header("Sec-WebSocket-Protocol", "json")
            .body(())?;

        let (ws_stream, _) = connect_async(request).await?;
        
        let mut stream_guard = self.ws_stream.lock().await;
        *stream_guard = Some(ws_stream);
        
        Ok(())
    }

    pub async fn send_message(&self, message: WebSocketMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut stream_guard = self.ws_stream.lock().await;
        
        if let Some(ref mut stream) = *stream_guard {
            let json_message = serde_json::to_string(&message)?;
            stream.send(Message::Text(json_message)).await?;
        } else {
            return Err("WebSocket not connected".into());
        }
        
        Ok(())
    }

    pub async fn listen_for_messages(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        loop {
            let message = {
                let mut stream_guard = self.ws_stream.lock().await;
                if let Some(ref mut stream) = *stream_guard {
                    match stream.next().await {
                        Some(msg) => msg?,
                        None => break,
                    }
                } else {
                    return Err("WebSocket not connected".into());
                }
            };

            match message {
                Message::Text(text) => {
                    // Emit the message to the frontend
                    let _ = self.app_handle.emit("websocket-message", text);
                }
                Message::Close(_) => {
                    let _ = self.app_handle.emit("websocket-closed", "Connection closed");
                    break;
                }
                _ => {}
            }
        }
        
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut stream_guard = self.ws_stream.lock().await;
        if let Some(ref mut stream) = *stream_guard {
            stream.send(Message::Close(None)).await?;
        }
        *stream_guard = None;
        Ok(())
    }
}