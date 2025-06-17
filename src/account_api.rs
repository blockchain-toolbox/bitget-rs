//! Bitget 账户相关接口
//!
//! 该模块提供了与 Bitget 账户相关的 API 接口
//! 包括查询账户信息、账户资产等功能

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::debug;

use crate::client::BitgetClient;
use crate::consts;

/// 账户信息响应结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountInfoResp {
    /// 保证金币种
    #[serde(rename = "marginCoin")]
    pub margin_coin: Option<String>,

    /// 可用余额
    pub available: Option<String>,

    /// 冻结金额
    pub frozen: Option<String>,

    /// 总资产
    pub total: Option<String>,

    /// 权益
    pub equity: Option<String>,

    /// 账户类型
    #[serde(rename = "accountType")]
    pub account_type: Option<String>,
}

/// 持仓信息响应结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PositionInfoResp {
    /// 交易对
    pub symbol: Option<String>,

    /// 保证金币种
    #[serde(rename = "marginCoin")]
    pub margin_coin: Option<String>,

    /// 持仓数量
    pub total: Option<String>,

    /// 可用数量
    pub available: Option<String>,

    /// 冻结数量
    pub frozen: Option<String>,

    /// 持仓方向（1: 多头方向，2: 空头方向）
    pub hold_side: Option<String>,

    /// 持仓模式（1: 逻辑仓位，2: 真实仓位）
    #[serde(rename = "holdMode")]
    pub hold_mode: Option<String>,

    /// 持仓均价
    #[serde(rename = "averageOpenPrice")]
    pub average_open_price: Option<String>,

    /// 杯利率
    pub leverage: Option<String>,
}

impl BitgetClient {
    /// 查询账户信息
    ///
    /// # 参数
    /// * `symbol` - 交易对
    /// * `margin_coin` - 保证金币种
    ///
    /// # 返回
    /// 返回账户信息响应字符串或错误
    pub fn get_account_info(&self, symbol: &str, margin_coin: &str) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("marginCoin".to_string(), margin_coin.to_string());

        debug!(
            "查询账户信息 - 交易对: {}, 保证金币种: {}",
            symbol, margin_coin
        );

        self.request(consts::GET, "/api/mix/v1/account/account", &params, false)
    }

    /// 查询账户信息（异步版本）
    ///
    /// # 参数
    /// * `symbol` - 交易对
    /// * `margin_coin` - 保证金币种
    ///
    /// # 返回
    /// 返回账户信息响应字符串或错误
    pub async fn get_account_info_async(&self, symbol: &str, margin_coin: &str) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("marginCoin".to_string(), margin_coin.to_string());

        debug!(
            "查询账户信息 - 交易对: {}, 保证金币种: {}",
            symbol, margin_coin
        );

        self.request(consts::GET, "/api/mix/v1/account/account", &params, false)
    }

    /// 查询持仓信息
    ///
    /// # 参数
    /// * `symbol` - 交易对
    /// * `margin_coin` - 保证金币种
    ///
    /// # 返回
    /// 返回持仓信息响应字符串或错误
    pub fn get_positions(&self, symbol: &str, margin_coin: &str) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("marginCoin".to_string(), margin_coin.to_string());

        debug!(
            "查询持仓信息 - 交易对: {}, 保证金币种: {}",
            symbol, margin_coin
        );

        self.request(
            consts::GET,
            "/api/mix/v1/position/allPosition",
            &params,
            false,
        )
    }

    /// 查询持仓信息（异步版本）
    ///
    /// # 参数
    /// * `symbol` - 交易对
    /// * `margin_coin` - 保证金币种
    ///
    /// # 返回
    /// 返回持仓信息响应字符串或错误
    pub async fn get_positions_async(&self, symbol: &str, margin_coin: &str) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("marginCoin".to_string(), margin_coin.to_string());

        debug!(
            "查询持仓信息 - 交易对: {}, 保证金币种: {}",
            symbol, margin_coin
        );

        self.request(
            consts::GET,
            "/api/mix/v1/position/allPosition",
            &params,
            false,
        )
    }

    /// 设置杠杆模式
    ///
    /// # 参数
    /// * `symbol` - 交易对
    /// * `margin_coin` - 保证金币种
    /// * `leverage` - 杠杆
    /// * `hold_side` - 持仓方向
    ///
    /// # 返回
    /// 返回设置结果响应字符串或错误
    pub fn set_leverage(
        &self,
        symbol: &str,
        margin_coin: &str,
        leverage: &str,
        hold_side: &str,
    ) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("marginCoin".to_string(), margin_coin.to_string());
        params.insert("leverage".to_string(), leverage.to_string());
        params.insert("holdSide".to_string(), hold_side.to_string());

        debug!(
            "设置杠杆 - 交易对: {}, 保证金币种: {}, 杠杆: {}, 方向: {}",
            symbol, margin_coin, leverage, hold_side
        );

        self.request(
            consts::POST,
            "/api/mix/v1/account/setLeverage",
            &params,
            false,
        )
    }

    /// 设置杠杆模式（异步版本）
    ///
    /// # 参数
    /// * `symbol` - 交易对
    /// * `margin_coin` - 保证金币种
    /// * `leverage` - 杠杆
    /// * `hold_side` - 持仓方向
    ///
    /// # 返回
    /// 返回设置结果响应字符串或错误
    pub async fn set_leverage_async(
        &self,
        symbol: &str,
        margin_coin: &str,
        leverage: &str,
        hold_side: &str,
    ) -> Result<String> {
        let mut params = BTreeMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("marginCoin".to_string(), margin_coin.to_string());
        params.insert("leverage".to_string(), leverage.to_string());
        params.insert("holdSide".to_string(), hold_side.to_string());

        debug!(
            "设置杠杆 - 交易对: {}, 保证金币种: {}, 杠杆: {}, 方向: {}",
            symbol, margin_coin, leverage, hold_side
        );

        self.request(
            consts::POST,
            "/api/mix/v1/account/setLeverage",
            &params,
            false,
        )
    }
}
