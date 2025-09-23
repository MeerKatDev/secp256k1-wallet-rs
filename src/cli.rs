use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "wallet-cli",
    about = "A simple blockchain wallet CLI with Diesel"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new wallet
    NewWallet,
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
}
