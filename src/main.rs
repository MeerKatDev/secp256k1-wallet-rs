mod cli;
mod db;
mod models;
mod schema;
mod sign;
mod wallet;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::NewWallet => {
            wallet::create_wallet().expect("Failed to create wallet");
        }
        Commands::ListWallets => {
            wallet::list_wallets().expect("Failed to list wallets");
        }
        Commands::Sign { wallet_id, message } => {
            sign::sign_message(wallet_id, &message).expect("Failed to sign message");
        }
        Commands::ListSignatures { wallet_id } => {
            sign::list_signatures(wallet_id).expect("Failed to list signatures");
        }
    }
}
