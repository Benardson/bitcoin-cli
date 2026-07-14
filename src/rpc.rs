use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use reqwest::Client;
use serde_json::{json, Value};

pub struct RpcClient {
    url: String,
    auth: String,
    client: Client,
}

impl RpcClient {
    pub fn new(
        url: &str,
        username: &str,
        password: &str,
    ) -> Self {
        let credentials = format!("{}:{}", username, password);

        let auth = general_purpose::STANDARD
            .encode(credentials);

        Self {
            url: url.to_string(),
            auth,
            client: Client::new(),
        }
    }

    async fn call(
        &self,
        method: &str,
        params: Value,
    ) -> Result<Value> {
        let response = self
            .client
            .post(&self.url)
            .header(
                "Authorization",
                format!("Basic {}", self.auth),
            )
            .json(&json!({
                "jsonrpc": "1.0",
                "id": "rust-cli",
                "method": method,
                "params": params
            }))
            .send()
            .await?;

        let result: Value = response.json().await?;

        if !result["error"].is_null() {
            anyhow::bail!(
                "RPC Error: {}",
                result["error"]
            );
        }

        Ok(result["result"].clone())
    }


    pub async fn get_blockchain_info(&self) -> Result<Value> {
        self.call(
            "getblockchaininfo",
            json!([])
        ).await
    }


    pub async fn get_wallet_info(&self) -> Result<Value> {
        self.call(
            "getwalletinfo",
            json!([])
        ).await
    }


    pub async fn get_balance(&self) -> Result<Value> {
        self.call(
            "getbalance",
            json!([])
        ).await
    }


    pub async fn get_new_address(&self) -> Result<Value> {
        self.call(
            "getnewaddress",
            json!([])
        ).await
    }


    pub async fn execute_rpc(
        &self,
        method: &str,
        params: Value,
    ) -> Result<Value> {
        self.call(method, params).await
    }
}