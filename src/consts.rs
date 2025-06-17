//! Bitget 常量定义
//!
//! 该模块定义了 Bitget API 使用的各种常量

/// API 基础 URL
pub const API_URL: &str = "https://api.bitget.com";

/// WebSocket API URL
pub const WS_URL: &str = "wss://ws.bitget.com/spot/v1/stream";

/// 请求方法: GET
pub const GET: &str = "GET";

/// 请求方法: POST
pub const POST: &str = "POST";

/// 签名类型
pub const SIGN_TYPE: &str = "HmacSHA256";

/// 内容类型
pub const CONTENT_TYPE: &str = "application/json";
