use crate::db::establish_connection;
use crate::models::{NewWallet, Wallet};
use crate::schema::wallets::dsl::*;

use diesel::prelude::*;
use secp256k1::Secp256k1;

pub fn create_wallet() -> anyhow::Result<()> {
    let conn = &mut establish_connection();
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut rand::rng());

    let wallet_address = hex::encode(public_key.serialize());

    let new_wallet = NewWallet {
        address: &wallet_address,
        private_key: &hex::encode(secret_key.secret_bytes()),
    };

    diesel::insert_into(wallets)
        .values(&new_wallet)
        .execute(conn)?;

    println!("Created wallet with address: {wallet_address}");
    Ok(())
}

pub fn list_wallets() -> anyhow::Result<()> {
    let conn = &mut establish_connection();
    let results: Vec<Wallet> = wallets.limit(10).load::<Wallet>(conn)?;

    for w in results {
        println!(
            "ID: {:?}, Address: {}, Created: {}",
            w.id, w.address, w.created_at
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::Connection;

    // helper: create an in-memory SQLite DB with the wallets table
    fn setup_test_db() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        diesel::sql_query(
            "CREATE TABLE wallets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                address TEXT NOT NULL,
                private_key TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            );",
        )
        .execute(&mut conn)
        .unwrap();
        conn
    }

    #[test]
    fn test_insert_wallet() {
        let mut conn = setup_test_db();

        // generate keypair
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::rng());
        let w_address = hex::encode(public_key.serialize());

        let new_wallet = NewWallet {
            address: &w_address,
            private_key: &hex::encode(secret_key.secret_bytes()),
        };

        diesel::insert_into(wallets)
            .values(&new_wallet)
            .execute(&mut conn)
            .unwrap();

        let results: Vec<Wallet> = wallets.load(&mut conn).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].address, w_address);
    }

    #[test]
    fn test_multiple_wallets() {
        let mut conn = setup_test_db();

        for _ in 0..3 {
            let secp = Secp256k1::new();
            let (secret_key, public_key) = secp.generate_keypair(&mut rand::rng());
            let w_address = hex::encode(public_key.serialize());

            let new_wallet = NewWallet {
                address: &w_address,
                private_key: &hex::encode(secret_key.secret_bytes()),
            };

            diesel::insert_into(wallets)
                .values(&new_wallet)
                .execute(&mut conn)
                .unwrap();
        }

        let results: Vec<Wallet> = wallets.load(&mut conn).unwrap();
        assert_eq!(results.len(), 3);
    }
}
