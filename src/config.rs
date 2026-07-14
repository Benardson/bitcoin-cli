use std::env;

pub struct Config {
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            rpc_url: env::var("RPC_URL")
                .expect("Missing RPC_URL in .env"),

            rpc_user: env::var("RPC_USER")
                .expect("Missing RPC_USER in .env"),

            rpc_password: env::var("RPC_PASSWORD")
                .expect("Missing RPC_PASSWORD in .env"),
        }
    }
}