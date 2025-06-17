//! Bitget 交易所 API 异常处理
//!
//! 该模块定义了 Bitget API 交互过程中的特定异常类型

use std::fmt;
use serde::{Deserialize, Serialize};

/// Bitget API 响应错误结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitgetApiError {
    /// 错误代码
    pub code: String,
    /// 错误消息
    pub msg: String,
    /// 请求跟踪 ID
    #[serde(default)]
    pub request_id: String,
}

impl fmt::Display for BitgetApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bitget API 错误 - 代码: {}, 消息: {}", self.code, self.msg)
    }
}

impl std::error::Error for BitgetApiError {}

/// 解析 API 响应中的错误
///
/// # 参数
/// * `response_text` - API 响应文本
///
/// # 返回
/// 如果响应包含错误，返回解析后的错误对象，否则返回 None
pub fn parse_error_response(response_text: &str) -> Option<BitgetApiError> {
    match serde_json::from_str::<serde_json::Value>(response_text) {
        Ok(value) => {
            // 检查是否包含错误代码和消息
            if let (Some(code), Some(msg)) = (value.get("code"), value.get("msg")) {
                // 检查是否为错误响应
                if code.as_str().map_or(false, |c| c != "00000") {
                    let request_id = value.get("requestId")
                        .and_then(|id| id.as_str())
                        .unwrap_or("").to_string();
                    
                    return Some(BitgetApiError {
                        code: code.as_str().unwrap_or("").to_string(),
                        msg: msg.as_str().unwrap_or("").to_string(),
                        request_id,
                    });
                }
            }
            None
        },
        Err(_) => None,
    }
}
