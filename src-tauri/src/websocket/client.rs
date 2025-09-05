use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};
use tokio::time::sleep;
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

#[derive(Debug, Clone)]
pub struct ReconnectionConfig {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

impl Default for ReconnectionConfig {
    fn default() -> Self {
        Self {
            max_retries: 10,
            initial_delay: Duration::from_millis(1000),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 1.5,
        }
    }
}

pub struct WebSocketClient {
    pub ws_stream: Arc<Mutex<Option<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>>>,
    pub app_handle: AppHandle,
    pub uri: Arc<RwLock<Option<String>>>,
    pub reconnection_config: ReconnectionConfig,
    pub reconnecting: Arc<Mutex<bool>>,
}

impl WebSocketClient {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            ws_stream: Arc::new(Mutex::new(None)),
            app_handle,
            uri: Arc::new(RwLock::new(None)),
            reconnection_config: ReconnectionConfig::default(),
            reconnecting: Arc::new(Mutex::new(false)),
        }
    }

    pub fn with_reconnection_config(mut self, config: ReconnectionConfig) -> Self {
        self.reconnection_config = config;
        self
    }

    pub async fn connect(&self, uri: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Store URI for reconnection
        {
            let mut uri_guard = self.uri.write().await;
            *uri_guard = Some(uri.to_string());
        }

        self.connect_internal(uri).await
    }

    async fn connect_internal(&self, uri: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let request = tokio_tungstenite::tungstenite::handshake::client::Request::builder()
            .uri(uri)
            .header("Sec-WebSocket-Protocol", "json")
            .body(())?;

        let (ws_stream, _) = connect_async(request).await?;
        
        let mut stream_guard = self.ws_stream.lock().await;
        *stream_guard = Some(ws_stream);
        
        let _ = self.app_handle.emit("websocket-connected", "Connected");
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
        
        // Connection ended - try to reconnect if we have a URI
        let _ = self.app_handle.emit("websocket-disconnected", "Connection lost");
        self.start_reconnection().await;
        
        Ok(())
    }

    async fn start_reconnection(&self) {
        let mut reconnecting_guard = self.reconnecting.lock().await;
        if *reconnecting_guard {
            return; // Already reconnecting
        }
        *reconnecting_guard = true;
        drop(reconnecting_guard);

        let uri = {
            let uri_guard = self.uri.read().await;
            uri_guard.clone()
        };

        if let Some(uri_str) = uri {
            let mut delay = self.reconnection_config.initial_delay;
            
            for attempt in 1..=self.reconnection_config.max_retries {
                eprintln!("Reconnection attempt {} of {}", attempt, self.reconnection_config.max_retries);
                let _ = self.app_handle.emit("websocket-reconnecting", format!("Attempt {}", attempt));

                match self.connect_internal(&uri_str).await {
                    Ok(()) => {
                        eprintln!("Reconnected successfully");
                        let _ = self.app_handle.emit("websocket-reconnected", "Reconnected");
                        let mut reconnecting_guard = self.reconnecting.lock().await;
                        *reconnecting_guard = false;
                        return;
                    }
                    Err(e) => {
                        eprintln!("Reconnection attempt {} failed: {}", attempt, e);
                        if attempt < self.reconnection_config.max_retries {
                            sleep(delay).await;
                            delay = Duration::from_millis(
                                (delay.as_millis() as f64 * self.reconnection_config.backoff_multiplier) as u64
                            ).min(self.reconnection_config.max_delay);
                        }
                    }
                }
            }

            eprintln!("All reconnection attempts failed");
            let _ = self.app_handle.emit("websocket-reconnection-failed", "All attempts failed");
        }

        let mut reconnecting_guard = self.reconnecting.lock().await;
        *reconnecting_guard = false;
    }

    pub async fn disconnect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Clear stored URI to prevent reconnection
        {
            let mut uri_guard = self.uri.write().await;
            *uri_guard = None;
        }

        let mut stream_guard = self.ws_stream.lock().await;
        if let Some(ref mut stream) = *stream_guard {
            stream.send(Message::Close(None)).await?;
        }
        *stream_guard = None;
        
        let _ = self.app_handle.emit("websocket-disconnected", "Manually disconnected");
        Ok(())
    }

    pub async fn is_connected(&self) -> bool {
        let stream_guard = self.ws_stream.lock().await;
        stream_guard.is_some()
    }

    pub async fn is_reconnecting(&self) -> bool {
        let reconnecting_guard = self.reconnecting.lock().await;
        *reconnecting_guard
    }
}