# Wallet handler with DB

`wallet-cli` is a command-line interface for managing secp256k1 and ed25519 wallets (despite the name) and signatures. 

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

# Default key type (ECDSA)
wallet-cli new-wallet

# Explicit key type
wallet-cli new-wallet --key-type eddsa

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
```
## Ideas to expand on it
- More address types. At the moment the default is Ethereum derivation and Solana derivation. Make it one-to-one with every chain which address format is supported.
- More key types. Support not just `ECDSA` and `EdDSA`, but also:
  - Tezos’s `P-256`
  - Bitcoin’s `secp256k1` multisig
  - BLS signatures (used in Ethereum 2.0)
- Add support for other DB types
- At the moment the private key is stored in clear in the DB. Aside from using some DB methods to encrypt, the OS keychain could be used or revealing it with a passphrase
- add lock for the whole 
- (big one) Add on-chain services (e.g. send txs, query balances) (connecting other rs-libraries).
- Introduce fuzz testing