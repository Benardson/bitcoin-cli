use anyhow::Result;

mod rpc;

use rpc::RpcClient;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let command = args
        .get(1)
        .expect("Missing command")
        .as_str();

    let client = RpcClient::new(
        "http://127.0.0.1:18443",
        "bitcoin",
        "bitcoin123",
    );

    match command {
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
                .expect("Missing RPC method");

            let result = client
                .execute_rpc(
                    method,
                    serde_json::json!([]),
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
            eprintln!("  rpc <method>");
        }
    }

    Ok(())
}