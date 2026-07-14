\# Bitcoin CLI - Rust Bitcoin Core RPC Client



\## Notes



This project was developed as part of the Rust for Bitcoin 2.0 technical assessment.



\## Project Description



A Rust command-line application that communicates with a local Bitcoin Core regtest node using the Bitcoin Core JSON-RPC interface.



The application provides common Bitcoin Core operations through a simple CLI and supports executing generic RPC commands with dynamic parameters.



\## Features



\* Connects to Bitcoin Core using JSON-RPC

\* Supports Bitcoin Regtest network

\* Displays blockchain information

\* Displays wallet information

\* Retrieves wallet balance

\* Generates new wallet addresses

\* Executes arbitrary Bitcoin Core RPC methods

\* Loads RPC configuration from environment variables



\## Requirements



\* Rust toolchain

\* Bitcoin Core

\* Polar (for creating a local Regtest environment)

\* Running Bitcoin Core Regtest node



\## Polar Setup



1\. Install Polar.

2\. Create a new Regtest network.

3\. Create and start a Bitcoin Core node.

4\. Copy the RPC URL, username, and password from the node settings.

5\. Configure the application using the `.env` file.



Example `.env`:



```env

RPC\_URL=http://127.0.0.1:18443

RPC\_USER=bitcoin

RPC\_PASSWORD=bitcoin123

```



\## Bitcoin Core Setup



The application connects to Bitcoin Core using JSON-RPC.



Example Regtest configuration:



```bash

bitcoind -regtest -server \\

\-rpcuser=bitcoin \\

\-rpcpassword=bitcoin123

```



Default RPC endpoint:



```

http://127.0.0.1:18443

```



\## Build



Clone the repository:



```bash

git clone https://github.com/Benardson/bitcoin-cli.git

cd bitcoin-cli

```



Build the project:



```bash

cargo build

```



\## Running the CLI



\### Blockchain Information



Command:



```bash

cargo run -- blockchain-info

```



Example output:



```json

{

&#x20; "chain": "regtest",

&#x20; "blocks": 101,

&#x20; "headers": 101,

&#x20; "difficulty": 4.6565423739069247e-10,

&#x20; "verificationprogress": 1

}

```



\---



\### Wallet Information



Command:



```bash

cargo run -- wallet-info

```



Example output:



```json

{

&#x20; "walletname": "testwallet",

&#x20; "txcount": 101

}

```



\---



\### Wallet Balance



Command:



```bash

cargo run -- balance

```



Example output:



```text

50.0

```



\---



\### Generate New Address



Command:



```bash

cargo run -- new-address

```



Example output:



```text

bcrt1qprdgyga0w9zrejrrmx4t0mlj2086yggew7dwd7

```



\---



\### Generic RPC Command



The CLI supports executing arbitrary Bitcoin Core RPC methods.



Example:



```bash

cargo run -- rpc getblockcount

```



Output:



```text

101

```



Example with parameters:



```bash

cargo run -- rpc getblockhash 100

```



Example:



```bash

cargo run -- rpc getbestblockhash

```



Output:



```text

60cf23a7bc073a42b8ea2b70e3764afc0068fd51e9ff29701fca1deee1cd2b21

```



\## Project Structure



```

bitcoin-cli/

│

├── Cargo.toml

├── Cargo.lock

├── README.md

├── .gitignore

├── .env

│

└── src/

&#x20;   ├── main.rs

&#x20;   ├── rpc.rs

&#x20;   └── config.rs

```



\## Notes



\* The application requires Bitcoin Core to be running before executing commands.

\* The project was developed and tested using Bitcoin Core Regtest mode.

\* RPC credentials are configured through environment variables.

\* The application uses asynchronous Rust with Tokio and communicates through Bitcoin Core JSON-RPC.



