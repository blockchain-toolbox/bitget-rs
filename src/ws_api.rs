//! Bitget WebSocket API
//! Bitget WebSocket 行情与推送模块

use anyhow::{Result, anyhow};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, info};

#[derive(Debug)]
pub struct BitgetWebSocketClient {
    stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    subscriptions: HashSet<String>,
}

impl BitgetWebSocketClient {
    pub async fn new(url: &str) -> Result<Self> {
        let (ws_stream, _) = connect_async(url).await?;
        Ok(Self {
            stream: Arc::new(Mutex::new(ws_stream)),
            subscriptions: HashSet::new(),
        })
    }

    pub async fn subscribe(&mut self, topic: &str) -> Result<()> {
        let msg = serde_json::json!({
            "op": "subscribe",
            "args": [topic]
        });
        self.send_message(msg).await
    }

    pub async fn send_message(&self, msg: serde_json::Value) -> Result<()> {
        let mut guard = self.stream.lock().await;
        guard.send(WsMessage::Text(msg.to_string())).await?;
        Ok(())
    }

    // ...（其他方法实现）...
}
