mod cli;
mod dao;
mod db;
mod key_type;

use clap::Parser;
use cli::{Cli, Commands};
use dao::{signature, wallet};

fn main() {
    let conn = &mut db::establish_connection();
    let out = &mut std::io::stdout();

    match Cli::parse().command {
        Commands::NewWallet { key_type } => {
            wallet::create_wallet(conn, key_type, out).expect("Failed to create wallet.");
        }
        Commands::ListWallets => {
            wallet::list_wallets(conn, out).expect("Failed to list wallets.");
        }
        Commands::ClearWallets => wallet::clear_wallets(conn).expect("Failed to clear wallets."),

        Commands::Sign { wallet_id, message } => {
            signature::sign_message(conn, wallet_id, &message, out)
                .expect("Failed to sign message.");
        }
        Commands::ListSignatures { wallet_id } => {
            signature::list_signatures(conn, wallet_id, out).expect("Failed to list signatures.");
        }
        Commands::ClearSignatures => {
            signature::clear_signatures(conn).expect("Failed to clear signatures.");
        }
        Commands::ClearSignaturesForWallet { wallet_id } => {
            signature::clear_signatures_for_wallet(conn, wallet_id)
                .expect("Failed to clear signatures.");
        }
    }
}
