//! Bitget 历史成交相关接口
//! 示例：GET /api/mix/v1/order/fills
use crate::client::BitgetClient;
use anyhow::Result;
use serde::Deserialize;
use std::collections::BTreeMap;
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct FillResp {
    pub trade_id: Option<String>,
    pub order_id: Option<String>,
    pub price: Option<String>,
    pub size: Option<String>,
    // TODO: 按需补充更多字段
}

impl BitgetClient {
    /// 查询历史成交
    pub fn get_fills(&self, symbol: &str, order_id: &str) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("orderId".to_string(), order_id.to_string());
        self.request(
            crate::consts::GET,
            "/api/mix/v1/order/fills",
            &params,
            false,
        )
    }
}
