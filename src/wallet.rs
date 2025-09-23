use crate::models::{NewWallet, Wallet};
use crate::schema::wallets::dsl::*;

use diesel::prelude::*;
use secp256k1::Secp256k1;
use std::io::Write;

pub fn create_wallet<W: Write>(conn: &mut SqliteConnection, out: &mut W) -> anyhow::Result<()> {
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

    writeln!(out, "Created wallet with address: {wallet_address}")?;
    Ok(())
}

pub fn list_wallets<W: Write>(conn: &mut SqliteConnection, out: &mut W) -> anyhow::Result<()> {
    let results: Vec<Wallet> = wallets.limit(10).load::<Wallet>(conn)?;

    writeln!(out, "Wallets:")?;
    for w in &results {
        writeln!(
            out,
            "ID: {}, Address: {}, Created: {}",
            w.id.unwrap(),
            w.address,
            w.created_at
        )?;
    }
    writeln!(out, "{} wallets listed.", results.len())?;
    Ok(())
}

pub fn clear_wallets(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    diesel::delete(wallets).execute(conn)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::Connection;

    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    fn setup_test_db() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        conn
    }

    #[test]
    fn test_create_wallet() {
        let mut conn = setup_test_db();
        let mut buf = Vec::new();

        create_wallet(&mut conn, &mut buf).unwrap();
        let wallet: Wallet = wallets.load(&mut conn).unwrap().into_iter().next().unwrap();

        assert!(!wallet.address.is_empty());
        assert!(!wallet.private_key.is_empty());

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Created wallet with address"));
    }

    #[test]
    fn test_list_wallets() {
        let mut conn = setup_test_db();
        let mut buf = Vec::new();

        // Insert multiple wallets
        create_wallet(&mut conn, &mut buf).unwrap();
        create_wallet(&mut conn, &mut buf).unwrap();

        // Now capture list_wallets output
        buf.clear();
        list_wallets(&mut conn, &mut buf).unwrap();

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Wallets:"));
        assert!(output.contains("wallets listed."));
    }

    #[test]
    fn test_clear_wallets() {
        let mut conn = setup_test_db();
        let mut buf = Vec::new();

        create_wallet(&mut conn, &mut buf).unwrap();
        create_wallet(&mut conn, &mut buf).unwrap();

        let results: Vec<Wallet> = wallets.load(&mut conn).unwrap();
        assert_eq!(results.len(), 2);

        clear_wallets(&mut conn).unwrap();

        let results: Vec<Wallet> = wallets.load(&mut conn).unwrap();
        assert_eq!(results.len(), 0);
    }
}
