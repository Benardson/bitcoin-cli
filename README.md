\# Bitcoin CLI - Rust Bitcoin Core RPC Client

\## Notes



This project was developed as part of the Rust for Bitcoin 2.0 technical assessment.



\## Project Description



A Rust command-line application that communicates with a local Bitcoin Core regtest node using the Bitcoin Core JSON-RPC interface.



The application provides common Bitcoin Core operations through a simple CLI and also supports executing generic RPC commands.



\## Features



\* Connects to Bitcoin Core using JSON-RPC

\* Supports regtest network

\* Query blockchain information

\* Query wallet information

\* Retrieve wallet balance

\* Generate new wallet addresses

\* Execute arbitrary Bitcoin Core RPC methods



\## Requirements



\* Rust toolchain

\* Bitcoin Core

\* Running Bitcoin Core regtest node



\## Bitcoin Core Setup



Start Bitcoin Core in regtest mode:



```bash

bitcoind -regtest -server \\

\-rpcuser=bitcoin \\

\-rpcpassword=bitcoin123

```



The application expects Bitcoin Core RPC at:



```

http://127.0.0.1:18443

```



\## Build



Clone the repository:



```bash

git clone <https://github.com/Benardson/bitcoin-cli.git>

cd bitcoin-cli

```



Build the project:



```bash

cargo build

```



\## Running the CLI



\### Blockchain Information



```bash

cargo run -- blockchain-info

```



Example output:



```json

{

&#x20; "chain": "regtest",

&#x20; "blocks": 101,

&#x20; "headers": 101

}

```



\### Wallet Information



```bash

cargo run -- wallet-info

```



Example output:



```json

{

&#x20; "walletname": "testwallet",

&#x20; "private\_keys\_enabled": true

}

```



\### Wallet Balance



```bash

cargo run -- balance

```



Example output:



```text

50.0

```



\### Generate New Address



```bash

cargo run -- new-address

```



Example output:



```text

bcrt1qprdgyga0w9zrejrrmx4t0mlj2086yggew7dwd7

```



\### Generic RPC Command



Execute any Bitcoin Core RPC method:



```bash

cargo run -- rpc getblockcount

```



Example output:



```text

101

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

│

└── src/

&#x20;   ├── main.rs

&#x20;   └── rpc.rs

```



\## Notes



\* The application requires Bitcoin Core to be running before executing commands.

\* The project was developed and tested using Bitcoin Core regtest mode.

\* RPC authentication is configured using username and password.



