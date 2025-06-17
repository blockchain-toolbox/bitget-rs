//! Bitget 行情相关接口
//!
//! 该模块提供了与 Bitget 行情相关的 API 接口
//! 包括查询行情、K线、深度等功能

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

use crate::client::BitgetClient;
use crate::consts;

/// 行情响应结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TickerResp {
    /// 交易对
    pub symbol: Option<String>,

    /// 最新成交价
    pub last: Option<String>,

    /// 买一价
    pub buy: Option<String>,

    /// 卖一价
    pub sell: Option<String>,

    /// 高价
    pub high: Option<String>,

    /// 低价
    pub low: Option<String>,

    /// 成交量
    pub volume: Option<String>,

    /// 成交额
    pub quote_volume: Option<String>,

    /// 时间戳
    pub timestamp: Option<String>,
}

/// K线数据响应结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CandleResp {
    /// 时间戳
    pub timestamp: Option<String>,

    /// 开盘价
    pub open: Option<String>,

    /// 最高价
    pub high: Option<String>,

    /// 最低价
    pub low: Option<String>,

    /// 收盘价
    pub close: Option<String>,

    /// 成交量
    pub volume: Option<String>,
}

impl BitgetClient {
    /// 查询现货行情（v1，已废弃，不推荐）
    ///
    /// # 参数
    /// * `symbol` - 交易对
    ///
    /// # 返回
    /// 返回行情响应字符串或错误
    pub fn get_ticker(&self, symbol: &str) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());

        tracing::debug!("查询现货行情(v1) - 交易对: {}", symbol);

        self.request(consts::GET, "/api/spot/v1/market/ticker", &params, false)
    }

    /// 查询现货行情（v2，推荐）
    ///
    /// # 参数
    /// * `symbol` - 交易对
    ///
    /// # 返回
    /// 返回行情响应字符串或错误
    pub fn get_ticker_v2(&self, symbol: &str) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());

        tracing::debug!("查询现货行情(v2) - 交易对: {}", symbol);

        let resp = self.request(consts::GET, "/api/v2/spot/market/tickers", &params, false)?;

        // 解析 data 字段，返回第一个 ticker
        let v: Value = serde_json::from_str(&resp)?;
        if v["code"] == "00000" {
            if let Some(arr) = v["data"].as_array() {
                if let Some(ticker) = arr.get(0) {
                    return Ok(ticker.to_string());
                }
            }
            Err(anyhow::anyhow!("未找到行情数据: {}", resp))
        } else {
            Err(anyhow::anyhow!("bitget v2 ticker 错误: {}", resp))
        }
    }

    /// 查询现货行情（v2，异步版本）
    ///
    /// # 参数
    /// * `symbol` - 交易对
    ///
    /// # 返回
    /// 返回行情响应字符串或错误
    pub async fn get_ticker_v2_async(&self, symbol: &str) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());

        tracing::debug!("查询现货行情(v2) - 交易对: {}", symbol);

        let resp = self.request(consts::GET, "/api/v2/spot/market/tickers", &params, false)?;

        // 解析 data 字段，返回第一个 ticker
        let v: Value = serde_json::from_str(&resp)?;
        if v["code"] == "00000" {
            if let Some(arr) = v["data"].as_array() {
                if let Some(ticker) = arr.get(0) {
                    return Ok(ticker.to_string());
                }
            }
            Err(anyhow::anyhow!("未找到行情数据: {}", resp))
        } else {
            Err(anyhow::anyhow!("bitget v2 ticker 错误: {}", resp))
        }
    }

    /// 查询K线数据
    ///
    /// # 参数
    /// * `symbol` - 交易对
    /// * `period` - 周期（如 "1m", "5m", "15m", "30m", "1h", "4h", "1d", "1w", "1M"）
    /// * `limit` - 返回数量，默认100，最大500
    ///
    /// # 返回
    /// 返回K线数据响应字符串或错误
    pub fn get_candles(&self, symbol: &str, period: &str, limit: Option<u32>) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("period".to_string(), period.to_string());
        if let Some(limit_val) = limit {
            params.insert("limit".to_string(), limit_val.to_string());
        }

        tracing::debug!("查询K线数据 - 交易对: {}, 周期: {}", symbol, period);

        self.request(consts::GET, "/api/v2/spot/market/candles", &params, false)
    }

    /// 查询K线数据（异步版本）
    ///
    /// # 参数
    /// * `symbol` - 交易对
    /// * `period` - 周期（如 "1m", "5m", "15m", "30m", "1h", "4h", "1d", "1w", "1M"）
    /// * `limit` - 返回数量，默认100，最大500
    ///
    /// # 返回
    /// 返回K线数据响应字符串或错误
    pub async fn get_candles_async(
        &self,
        symbol: &str,
        period: &str,
        limit: Option<u32>,
    ) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("period".to_string(), period.to_string());
        if let Some(limit_val) = limit {
            params.insert("limit".to_string(), limit_val.to_string());
        }

        tracing::debug!("查询K线数据 - 交易对: {}, 周期: {}", symbol, period);

        self.request(consts::GET, "/api/v2/spot/market/candles", &params, false)
    }

    /// 查询深度数据
    ///
    /// # 参数
    /// * `symbol` - 交易对
    /// * `limit` - 返回数量，默认50，最大200
    ///
    /// # 返回
    /// 返回深度数据响应字符串或错误
    pub fn get_depth(&self, symbol: &str, limit: Option<u32>) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        if let Some(limit_val) = limit {
            params.insert("limit".to_string(), limit_val.to_string());
        }

        tracing::debug!("查询深度数据 - 交易对: {}", symbol);

        self.request(consts::GET, "/api/v2/spot/market/orderbook", &params, false)
    }

    /// 查询深度数据（异步版本）
    ///
    /// # 参数
    /// * `symbol` - 交易对
    /// * `limit` - 返回数量，默认50，最大200
    ///
    /// # 返回
    /// 返回深度数据响应字符串或错误
    pub async fn get_depth_async(&self, symbol: &str, limit: Option<u32>) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        if let Some(limit_val) = limit {
            params.insert("limit".to_string(), limit_val.to_string());
        }

        tracing::debug!("查询深度数据 - 交易对: {}", symbol);

        self.request(consts::GET, "/api/v2/spot/market/orderbook", &params, false)
    }
}
