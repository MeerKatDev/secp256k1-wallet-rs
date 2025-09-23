# Wallet handler with DB

`wallet-cli` is a command-line interface for managing secp256k1 wallets and signatures. 

It is a simple CLI app which
 - creates keypairs/wallets
 - lists the wallets
 - creates signatures
 - lists signatures

clear them all too. To start, make sure to have `diesel-cli` installed
```
cargo install diesel_cli --no-default-features --features sqlite
```
and then
```
diesel setup
diesel migration run
```
this will crate the local db with which it can interact. 

The commands follow the clap defaults:

## Commands Overview

| Command | Description | Options / Arguments | Example |
|---------|-------------|------------------|---------|
| `new-wallet` | Create a new wallet with a random keypair | None | `cargo run -- new-wallet` |
| `list-wallets` | List all wallets in the database (max 10) | None | `cargo run -- list-wallets` |
| `sign` | Sign a message with a wallet | `-w, --wallet-id <ID>` <br> `-m, --message <MESSAGE>` | `cargo run -- sign -w 1 -m "Hello world"` |
| `list-signatures` | List signatures for a wallet | `-w, --wallet-id <ID>` | `cargo run -- list-signatures -w 1` |
| `clear-wallets` | Delete all wallets from the database | None | `cargo run -- clear-wallets` |
| `clear-signatures` | Delete all signatures from the database | None | `cargo run -- clear-signatures` |
| `clear-signatures-for-wallet` | Delete all signatures for a specific wallet | `-w, --wallet-id <ID>` | `cargo run -- clear-signatures-for-wallet -w 1` |

## Examples

```bash
# Create two wallets
cargo run -- new-wallet
cargo run -- new-wallet

# List all wallets
cargo run -- list-wallets

# Sign a message with wallet ID 1
cargo run -- sign -w 1 -m "Hello world"

# List all signatures for wallet ID 1
cargo run -- list-signatures -w 1

# Clear all signatures for wallet ID 1
cargo run -- clear-signatures-for-wallet -w 1

# Clear all wallets
cargo run -- clear-wallets

# Clear all signatures
cargo run -- clear-signatures