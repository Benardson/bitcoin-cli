use anyhow::Result;
use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        dotenv().ok();

        Ok(Self {
            rpc_url: env::var("RPC_URL")?,
            rpc_user: env::var("RPC_USER")?,
            rpc_password: env::var("RPC_PASSWORD")?,
        })
    }
}