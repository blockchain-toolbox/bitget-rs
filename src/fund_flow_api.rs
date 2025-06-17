//! Bitget 资金流水、特殊划转等接口
use crate::client::BitgetClient;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use tracing::info;

#[derive(Debug, Serialize)]
pub struct FundFlowReq {
    // TODO: 按官方文档定义
}

#[derive(Debug, Deserialize)]
pub struct FundFlowResp {
    // TODO: 按官方文档定义
}

impl BitgetClient {
    pub fn get_fund_flow(&self, req: FundFlowReq) -> Result<FundFlowResp> {
        info!("查询资金流水: {:?}", req);
        // TODO: 实现资金流水查询
        Ok(FundFlowResp { /* ... */ })
    }
}
