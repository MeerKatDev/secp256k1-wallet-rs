use clap::{Parser, Subcommand};

use crate::key_type::KeyType;

#[derive(Parser)]
#[command(
    name = "wallet-cli",
    about = "A simple CLI blockchain wallet for secp256k1 addresses."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new wallet
    NewWallet {
        #[arg(short, long, value_enum)]
        key_type: KeyType,
    },
    /// List all wallets
    ListWallets,
    /// Sign a message with a wallet
    Sign {
        #[arg(short, long)]
        wallet_id: i32,
        #[arg(short, long)]
        message: String,
    },
    /// List signatures from a wallet
    ListSignatures {
        #[arg(short, long)]
        wallet_id: i32,
    },
    ClearWallets,
    ClearSignatures,
    ClearSignaturesForWallet {
        #[arg(short, long)]
        wallet_id: i32,
    },
}
