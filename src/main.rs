use anyhow::Result;

mod config;
mod rpc;

use config::Config;
use rpc::RpcClient;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let command = args
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("Missing command"))?;

    let config = Config::from_env();

    let client = RpcClient::new(
        &config.rpc_url,
        &config.rpc_user,
        &config.rpc_password,
    );

    match command.as_str() {
        "blockchain-info" => {
            let result = client.get_blockchain_info().await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }

        "wallet-info" => {
            let result = client.get_wallet_info().await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }

        "balance" => {
            let result = client.get_balance().await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }

        "new-address" => {
            let result = client.get_new_address().await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }

        "rpc" => {
            let method = args
                .get(2)
                .ok_or_else(|| anyhow::anyhow!("Missing RPC method"))?;

            let params: Vec<serde_json::Value> = args
                .iter()
                .skip(3)
                .map(|arg| {
                    match arg.parse::<u64>() {
                        Ok(value) => serde_json::json!(value),
                        Err(_) => serde_json::json!(arg),
                    }
                })
                .collect();

            let result = client
                .execute_rpc(
                    method,
                    serde_json::json!(params),
                )
                .await?;

            println!("{}", serde_json::to_string_pretty(&result)?);
        }

        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Available commands:");
            eprintln!("  blockchain-info");
            eprintln!("  wallet-info");
            eprintln!("  balance");
            eprintln!("  new-address");
            eprintln!("  rpc <method> [params...]");
        }
    }

    Ok(())
}