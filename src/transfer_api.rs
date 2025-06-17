//! Bitget 资金划转相关接口
//! 示例：POST /api/spot/v1/wallet/transfer
use crate::client::BitgetClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::info;

#[derive(Debug, Serialize)]
pub struct TransferReq {
    pub from_type: String,
    pub to_type: String,
    pub coin: String,
    pub amount: String,
    // TODO: 按需补充更多参数
}

#[derive(Debug, Deserialize)]
pub struct TransferResp {
    pub transfer_id: Option<String>,
    // TODO: 按需补充更多字段
}

impl BitgetClient {
    /// 资金划转
    pub fn transfer(&self, req: &TransferReq) -> Result<String> {
        info!(
            "资金划转 from={}, to={}, coin={}",
            req.from_type, req.to_type, req.coin
        );
        let mut params = BTreeMap::new();
        params.insert("fromType".to_string(), req.from_type.clone());
        params.insert("toType".to_string(), req.to_type.clone());
        params.insert("coin".to_string(), req.coin.clone());
        params.insert("amount".to_string(), req.amount.clone());
        // TODO: 其他参数
        self.request(
            crate::consts::POST,
            "/api/spot/v1/wallet/transfer",
            &params,
            false,
        )
    }
}
