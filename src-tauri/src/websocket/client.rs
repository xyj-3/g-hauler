use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use tokio_tungstenite::{client_async, WebSocketStream};
use tokio_tungstenite::tungstenite::{Message, handshake::client::generate_key};
use tokio_tungstenite::tungstenite::http::Request;
use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub verb: String,
    pub path: String,
    pub payload: Value,
}

pub struct WebSocketClient {
    pub write_stream: Arc<Mutex<Option<SplitSink<WebSocketStream<TcpStream>, Message>>>>,
    pub read_stream: Arc<Mutex<Option<SplitStream<WebSocketStream<TcpStream>>>>>,
    pub app_handle: AppHandle,
    pub is_connected: Arc<AtomicBool>,
}

impl WebSocketClient {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            write_stream: Arc::new(Mutex::new(None)),
            read_stream: Arc::new(Mutex::new(None)),
            app_handle,
            is_connected: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn connect(&self, uri: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Parse the WebSocket URI to extract host and port
        let uri_parsed = uri.parse::<tokio_tungstenite::tungstenite::http::Uri>()?;
        let host = uri_parsed.host().ok_or("Invalid URI: missing host")?;
        let port = uri_parsed.port_u16().unwrap_or(9010);
        let addr = format!("{}:{}", host, port);

        // Establish TCP connection
        let stream = TcpStream::connect(&addr).await?;

        // Build the WebSocket handshake request
        let request = Request::builder()
            .uri(uri)
            .header("Host", &addr)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Key", generate_key())
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Protocol", "json")
            .body(())?;

        // Perform WebSocket handshake
        let (ws_stream, _) = client_async(request, stream).await?;
        
        // Split the stream into read and write halves
        let (write, read) = ws_stream.split();

        // Store the split streams
        {
            let mut write_guard = self.write_stream.lock().await;
            *write_guard = Some(write);
        }
        {
            let mut read_guard = self.read_stream.lock().await;
            *read_guard = Some(read);
        }

        self.is_connected.store(true, Ordering::SeqCst);
        let _ = self.app_handle.emit("websocket-connected", "Connected");
        Ok(())
    }

    pub async fn send_message(&self, message: WebSocketMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut write_guard = self.write_stream.lock().await;

        if let Some(ref mut write_stream) = *write_guard {
            let json_message = json!({
                "verb": message.verb.to_uppercase(),
                "path": message.path,
                "payload": message.payload
            });
            let message_str = json_message.to_string();
            eprintln!("[WebSocket Client] Sending JSON: {}", message_str);
            write_stream.send(Message::Text(message_str)).await?;
            eprintln!("[WebSocket Client] Message sent to server");
        } else {
            eprintln!("[WebSocket Client] Cannot send message: not connected");
            return Err("WebSocket not connected".into());
        }

        Ok(())
    }

    pub async fn listen_for_messages(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        loop {
            // Acquire the read lock, get the next message, and immediately release the lock
            let message_result = {
                let mut read_guard = self.read_stream.lock().await;
                if let Some(ref mut read_stream) = *read_guard {
                    read_stream.next().await
                } else {
                    return Err("WebSocket not connected".into());
                }
            }; // Lock is released here

            // Process the message outside the lock
            match message_result {
                Some(Ok(message)) => {
                    match message {
                        Message::Text(text) => {
                            eprintln!("[WebSocket Client] Received message: {}", text);
                            // Emit the message to the frontend
                            let _ = self.app_handle.emit("websocket-message", text);
                        }
                        Message::Close(_) => {
                            eprintln!("[WebSocket Client] Connection closed by server");
                            self.is_connected.store(false, Ordering::SeqCst);
                            let _ = self.app_handle.emit("websocket-closed", "Connection closed");
                            break;
                        }
                        Message::Ping(data) => {
                            eprintln!("[WebSocket Client] Received ping");
                            // Send pong response using write stream
                            let mut write_guard = self.write_stream.lock().await;
                            if let Some(ref mut write_stream) = *write_guard {
                                let _ = write_stream.send(Message::Pong(data)).await;
                            }
                        }
                        Message::Pong(_) => {
                            eprintln!("[WebSocket Client] Received pong");
                        }
                        _ => {
                            eprintln!("[WebSocket Client] Received other message type: {:?}", message);
                        }
                    }
                }
                Some(Err(e)) => {
                    self.is_connected.store(false, Ordering::SeqCst);
                    return Err(e.into());
                },
                None => {
                    self.is_connected.store(false, Ordering::SeqCst);
                    break;
                }
            }
        }

        let _ = self.app_handle.emit("websocket-disconnected", "Connection lost");
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Send close message via write stream
        {
            let mut write_guard = self.write_stream.lock().await;
            if let Some(ref mut write_stream) = *write_guard {
                let _ = write_stream.send(Message::Close(None)).await;
            }
            *write_guard = None;
        }
        
        // Clear read stream
        {
            let mut read_guard = self.read_stream.lock().await;
            *read_guard = None;
        }

        self.is_connected.store(false, Ordering::SeqCst);
        let _ = self.app_handle.emit("websocket-disconnected", "Manually disconnected");
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected.load(Ordering::SeqCst)
    }
}