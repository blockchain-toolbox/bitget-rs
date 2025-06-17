//! Bitget 工具函数
//!
//! 该模块提供了 Bitget API 使用的各种工具函数
//! 包括签名、时间戳、header 构造等

use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::BTreeMap;

/// 将参数序列化为 URL 查询字符串（按 key 排序）
///
/// # 参数
/// * `params` - 要序列化的参数映射
///
/// # 返回
/// 返回序列化后的 URL 查询字符串，如 "?key1=value1&key2=value2"
/// 如果参数为空，返回空字符串
pub fn parse_params_to_str(params: &BTreeMap<String, String>) -> String {
    if params.is_empty() {
        return "".to_string();
    }
    let mut s = String::from("?");
    for (i, (k, v)) in params.iter().enumerate() {
        if i > 0 {
            s.push('&');
        }
        s.push_str(&format!("{k}={v}"));
    }
    s
}

/// 获取当前 UTC 毫秒时间戳字符串
///
/// # 返回
/// 返回当前 UTC 时间的毫秒时间戳字符串
pub fn get_timestamp() -> String {
    Utc::now().timestamp_millis().to_string()
}

/// 构造待签名字符串
///
/// # 参数
/// * `timestamp` - 时间戳字符串
/// * `method` - 请求方法（GET/POST）
/// * `request_path` - 请求路径
/// * `body` - 请求体内容
///
/// # 返回
/// 返回待签名的字符串
pub fn pre_hash(timestamp: &str, method: &str, request_path: &str, body: &str) -> String {
    format!("{timestamp}{method}{request_path}{body}")
}

/// HMAC-SHA256 签名并 base64 编码
///
/// # 参数
/// * `pre_hash` - 待签名字符串
/// * `secret` - 密钥
///
/// # 返回
/// 返回签名结果字符串或错误
pub fn sign(pre_hash: &str, secret: &str) -> Result<String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(|e| anyhow!("HMAC 初始化失败: {:?}", e))?;
    mac.update(pre_hash.as_bytes());
    let result = mac.finalize().into_bytes();
    Ok(general_purpose::STANDARD.encode(result))
}

/// 构造请求头
///
/// # 参数
/// * `api_key` - API 密钥
/// * `sign` - 签名结果
/// * `timestamp` - 时间戳
/// * `passphrase` - API 密码短语
///
/// # 返回
/// 返回请求头键值对列表
pub fn get_header(
    api_key: &str,
    sign: &str,
    timestamp: &str,
    passphrase: &str,
) -> Vec<(String, String)> {
    vec![
        ("Content-Type".to_string(), "application/json".to_string()),
        ("ACCESS-KEY".to_string(), api_key.to_string()),
        ("ACCESS-SIGN".to_string(), sign.to_string()),
        ("ACCESS-TIMESTAMP".to_string(), timestamp.to_string()),
        ("ACCESS-PASSPHRASE".to_string(), passphrase.to_string()),
    ]
}

/// 构建查询字符串
pub fn build_query(params: &BTreeMap<String, String>) -> String {
    params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&")
}
