# Bitget SDK for Rust

高性能、模块化的 Bitget 交易所 API SDK，支持现货下单、撤单、资产、行情、WebSocket 等全量功能，适用于量化、自动化交易开发。

## 特性
- 完全模块化，代码结构清晰
- 支持 REST v2 现货下单、撤单、批量撤单、资产查询、订单查询等
- 支持 WebSocket 行情与订单推送
- 错误处理统一采用 anyhow，日志统一 tracing
- 详细注释，便于二次开发

## 快速开始

```toml
[dependencies]
bitget_sdk = { path = "./bitget_sdk" }
```

```rust
use bitget_sdk::BitgetClient;
use bitget_sdk::order_api::*;

let client = BitgetClient::new(api_key, api_secret, passphrase);
let resp = client.place_spot_order_v2(...)?;
```

## 目录结构
- client.rs         // HTTP请求签名与发送
- asset_api.rs      // 资产相关接口
- order_api.rs      // 下单/撤单接口
- order_query_api.rs// 订单查询
- ws_api.rs         // WebSocket 行情/订单
- ...

## 依赖
- anyhow
- serde / serde_json
- tracing
- reqwest
- tokio

## 贡献与许可
欢迎 issue/PR，MIT License。
