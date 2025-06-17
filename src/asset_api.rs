//! Bitget 资产相关接口
//! 示例：GET /api/mix/v1/account/account
use crate::client::BitgetClient;
use anyhow::Result;
use serde::Deserialize;
use std::collections::BTreeMap;

/// 资产信息响应结构体（可根据实际返回补充字段）
#[derive(Debug, Deserialize)]
pub struct AssetInfoResp {
    pub coin: Option<String>,
    pub available: Option<String>,
    // TODO: 按需补充更多字段
}

impl BitgetClient {
    /// 查询资产信息（v1，已废弃）
    pub fn get_assets(&self) -> Result<String> {
        let params = BTreeMap::new();
        self.request(
            crate::consts::GET,
            "/api/spot/v1/account/assets",
            &params,
            false,
        )
    }

    /// 查询指定币种资产（v2，推荐）
    pub fn get_asset_v2(&self, coin: &str) -> anyhow::Result<String> {
        let mut params = BTreeMap::new();
        params.insert("coin".to_string(), coin.to_string());
        self.request(
            crate::consts::GET,
            "/api/v2/spot/account/assets",
            &params,
            false,
        )
    }
}
