//! Bitget 订单查询相关接口
//! 示例：GET /api/mix/v1/order/detail
use crate::client::BitgetClient;
use anyhow::Result;
use serde::Deserialize;
use std::collections::BTreeMap;
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct OrderDetailResp {
    pub order_id: Option<String>,
    pub status: Option<String>,
    // TODO: 按需补充更多字段
}

impl BitgetClient {
    /// 查询订单详情（合约）
    pub fn get_order_detail(&self, symbol: &str, order_id: &str) -> Result<String> {
        info!("查询订单详情 symbol={}, order_id={}", symbol, order_id);
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("orderId".to_string(), order_id.to_string());
        self.request(
            crate::consts::GET,
            "/api/mix/v1/order/detail",
            &params,
            false,
        )
    }

    /// 查询现货未成交订单（活跃订单，v2）
    pub fn get_spot_unfilled_orders(
        &self,
        symbol: &str,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u32>,
    ) -> Result<String> {
        info!("查询现货未成交订单 symbol={}", symbol);
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        if let Some(start) = start_time {
            params.insert("startTime".to_string(), start.to_string());
        }
        if let Some(end) = end_time {
            params.insert("endTime".to_string(), end.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        self.request(
            crate::consts::GET,
            "/api/v2/spot/trade/unfilled-orders",
            &params,
            false,
        )
    }
}
