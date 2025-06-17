//! Bitget 合约下单相关接口
//! 示例：POST /api/mix/v1/order/place
use crate::client::BitgetClient;
use crate::consts;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 现货/合约撤单响应结构体
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResp {
    pub code: String,
    pub msg: Option<String>,
    pub request_time: Option<u64>,
    pub data: Option<serde_json::Value>,
}

/// 批量撤单响应结构体
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrderResp {
    pub code: String,
    pub msg: Option<String>,
    pub request_time: Option<u64>,
    pub data: Option<serde_json::Value>,
}

/// v2 现货下单参数
#[derive(Debug, Serialize, Clone)]
pub struct SpotOrderV2Req {
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub force: String,
    pub price: Option<String>,
    pub size: String,
    pub client_oid: Option<String>,
}

/// v2 现货下单响应
#[derive(Debug, Deserialize)]
pub struct SpotOrderV2Resp {
    pub order_id: Option<String>,
    pub client_oid: Option<String>,
    pub msg: Option<String>,
}

impl BitgetClient {
    /// v2 现货下单
    pub fn place_spot_order_v2(&self, req: &SpotOrderV2Req) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), req.symbol.clone());
        params.insert("side".to_string(), req.side.clone());
        params.insert("orderType".to_string(), req.order_type.clone());
        params.insert("force".to_string(), req.force.clone());
        if let Some(price) = &req.price {
            params.insert("price".to_string(), price.clone());
        }
        params.insert("size".to_string(), req.size.clone());
        self.request(
            consts::POST,
            "/api/v2/spot/trade/place-order",
            &params,
            false,
        )
    }

    /// 合约撤单
    pub fn cancel_futures_order(
        &self,
        symbol: &str,
        order_id: &str,
        margin_coin: &str,
    ) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("orderId".to_string(), order_id.to_string());
        params.insert("marginCoin".to_string(), margin_coin.to_string());
        self.request(
            consts::POST,
            "/api/mix/v1/order/cancel-order",
            &params,
            false,
        )
    }

    /// 现货撤单(v2)
    pub fn cancel_spot_order(&self, symbol: String, order_id: String) -> Result<CancelOrderResp> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol);
        params.insert("orderId".to_string(), order_id);
        let resp = self.request(
            consts::POST,
            "/api/v2/spot/trade/cancel-order",
            &params,
            false,
        )?;
        serde_json::from_str(&resp).map_err(|e| e.into())
    }

    /// 批量现货撤单(v2)
    pub fn batch_cancel_spot_order(
        &self,
        symbol: String,
        order_ids: Vec<String>,
    ) -> Result<BatchCancelOrderResp> {
        #[derive(Serialize)]
        struct OrderItem {
            order_id: String,
            symbol: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            client_oid: Option<String>,
        }

        let order_list: Vec<OrderItem> = order_ids
            .into_iter()
            .map(|order_id| OrderItem {
                order_id,
                symbol: symbol.clone(),
                client_oid: None,
            })
            .collect();

        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol);
        params.insert("batchMode".to_string(), "multiple".to_string());
        params.insert("orderList".to_string(), serde_json::to_string(&order_list)?);

        let resp = self.request(
            consts::POST,
            "/api/v2/spot/trade/batch-cancel-order",
            &params,
            false,
        )?;
        serde_json::from_str(&resp).map_err(|e| e.into())
    }

    /// 批量合约撤单
    pub fn cancel_futures_orders(
        &self,
        symbol: &str,
        order_ids: &[&str],
        margin_coin: &str,
    ) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("orderIds".to_string(), order_ids.join(","));
        params.insert("marginCoin".to_string(), margin_coin.to_string());
        self.request(
            consts::POST,
            "/api/mix/v1/order/cancel-batch-orders",
            &params,
            false,
        )
    }

    /// v2 批量撤销某 symbol 下所有现货订单
    pub fn cancel_spot_symbol_orders(&self, symbol: String) -> Result<CancelOrderResp> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol);
        let resp = self.request(
            consts::POST,
            "/api/v2/spot/trade/cancel-symbol-order",
            &params,
            false,
        )?;
        serde_json::from_str(&resp).map_err(|e| e.into())
    }
}
