//! Bitget HTTP 客户端实现
//!
//! 该模块提供了与 Bitget 交易所 API 交互的核心客户端实现
//! 负责处理 API 请求、签名验证和错误处理

use anyhow::{Result, anyhow};
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::BTreeMap;
use std::str::FromStr;
use tracing::{debug, error};

use crate::consts;
use crate::utils;

/// Bitget 交易所客户端
///
/// 提供与 Bitget API 交互的核心功能，包括请求签名、发送请求等
#[derive(Debug, Clone)]
pub struct BitgetClient {
    /// API 密钥
    pub api_key: String,
    /// API 密钥对应的秘钥
    pub api_secret_key: String,
    /// API 密码短语
    pub passphrase: String,
    /// 是否使用服务器时间
    pub use_server_time: bool,
    /// 是否为首次请求（用于调试）
    pub first: bool,
    /// HTTP 客户端
    http_client: Client,
    /// 基础 URL
    base_url: String,
}

impl BitgetClient {
    /// 创建新的 Bitget 客户端实例
    ///
    /// # 参数
    /// * `api_key` - API 密钥
    /// * `api_secret_key` - API 密钥对应的秘钥
    /// * `passphrase` - API 密码短语
    /// * `use_server_time` - 是否使用服务器时间
    /// * `first` - 是否为首次请求（用于调试）
    ///
    /// # 返回
    /// 返回 BitgetClient 实例
    pub fn new(
        api_key: String,
        api_secret_key: String,
        passphrase: String,
        use_server_time: bool,
        first: bool,
    ) -> Self {
        Self {
            api_key,
            api_secret_key,
            passphrase,
            use_server_time,
            first,
            http_client: Client::new(),
            base_url: consts::API_URL.to_string(),
        }
    }

    /// 发送同步请求（支持 GET/POST），自动签名、构造 header
    ///
    /// # 参数
    /// * `method` - 请求方法（GET/POST）
    /// * `request_path` - 请求路径
    /// * `params` - 请求参数（BTreeMap，自动按 key 排序）
    /// * `cursor` - 是否为分页请求
    ///
    /// # 返回
    /// 返回请求结果字符串或错误
    pub fn request(
        &self,
        method: &str,
        request_path: &str,
        params: &BTreeMap<String, String>,
        cursor: bool,
    ) -> Result<String> {
        let _cursor = cursor;
        // 1. 构造 url
        let mut full_path = request_path.to_string();
        if method == consts::GET {
            full_path.push('?');
            full_path.push_str(&utils::build_query(params));
        }

        let url = format!("{}{}", self.base_url, full_path);

        // 2. 构造 headers
        let headers = self.build_headers(method, &full_path, params)?;

        // 3. 构造 body (POST 请求)
        let body = if method == consts::POST {
            Some(serde_json::to_string(params)?)
        } else {
            None
        };

        // 4. 发送请求
        let response = match method {
            consts::GET => self.http_client.get(&url).headers(headers).send()?,
            consts::POST => self
                .http_client
                .post(&url)
                .headers(headers)
                .body(body.unwrap())
                .send()?,
            _ => return Err(anyhow!("不支持的 HTTP 方法: {}", method)),
        };

        // 5. 处理响应
        let status = response.status();
        let text = response.text()?;

        if !status.is_success() {
            return Err(anyhow!("请求失败，状态码: {}, 响应: {}", status, text));
        }

        Ok(text)
    }

    fn build_headers(
        &self,
        method: &str,
        full_path: &str,
        params: &BTreeMap<String, String>,
    ) -> Result<HeaderMap> {
        let timestamp = if self.use_server_time {
            // TODO: 实现获取服务器时间接口
            utils::get_timestamp()
        } else {
            utils::get_timestamp()
        };

        let body = if method == consts::POST {
            serde_json::to_string(params).map_err(|e| anyhow!("序列化参数失败: {}", e))?
        } else {
            String::new()
        };

        let pre_hash = utils::pre_hash(&timestamp, method, full_path, &body);
        let sign = match consts::SIGN_TYPE {
            "RSA" => {
                // TODO: 实现 RSA 签名
                return Err(anyhow!("RSA 签名暂未实现"));
            }
            _ => utils::sign(&pre_hash, &self.api_secret_key)?,
        };

        let headers = utils::get_header(&self.api_key, &sign, &timestamp, &self.passphrase);

        let mut header_map = HeaderMap::new();
        for (k, v) in headers {
            header_map.insert(
                HeaderName::from_str(&k).map_err(|e| anyhow!("无效的 header 名称: {}", e))?,
                HeaderValue::from_str(&v).map_err(|e| anyhow!("无效的 header 值: {}", e))?,
            );
        }

        Ok(header_map)
    }
}
