use crate::key_type::KeyType;
use crate::models::{NewWallet, Wallet};
use crate::schema::wallets::dsl::*;

use diesel::prelude::*;
use std::io::Write;

pub fn create_wallet<W: Write>(
    conn: &mut SqliteConnection,
    key_type_arg: KeyType,
    out: &mut W,
) -> anyhow::Result<()> {
    let (addr, privkey) = match key_type_arg {
        crate::key_type::KeyType::Eddsa => make_eddsa_key(),
        crate::key_type::KeyType::Ecdsa => make_ecdsa_key(),
    };

    let new_wallet = NewWallet {
        address: &addr,
        private_key: &privkey,
        key_type: key_type_arg.into(),
    };

    diesel::insert_into(wallets)
        .values(&new_wallet)
        .execute(conn)?;

    writeln!(out, "Created wallet with address: {addr}")?;
    Ok(())
}

fn make_eddsa_key() -> (String, String) {
    use ed25519_dalek::{SigningKey, SECRET_KEY_LENGTH};
    use rand::rngs::OsRng;
    use rand::TryRngCore;
    let mut key = [0u8; SECRET_KEY_LENGTH];
    OsRng.try_fill_bytes(&mut key).unwrap();

    let signing_key: SigningKey = SigningKey::from_bytes(&key);
    // assuming address derivation for Solana
    let addr = bs58::encode(signing_key.verifying_key().as_bytes()).into_string();
    let privkey = hex::encode(signing_key.to_bytes());
    (addr, privkey)
}

fn make_ecdsa_key() -> (String, String) {
    use secp256k1::Secp256k1;

    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut rand::rng());
    // assuming address derivation for Ethereum
    let addr = hex::encode(public_key.serialize());
    let privkey = hex::encode(secret_key.secret_bytes());
    (addr, privkey)
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

        for key_type_val in &[KeyType::Ecdsa, KeyType::Eddsa] {
            let mut buf = Vec::new();

            create_wallet(&mut conn, *key_type_val, &mut buf).unwrap();
            let wallet: Wallet = wallets.load(&mut conn).unwrap().into_iter().last().unwrap();

            // Validate that address and private key are populated
            assert!(!wallet.address.is_empty());
            assert!(!wallet.private_key.is_empty());

            // Validate that the output contains the expected string
            let output = String::from_utf8(buf).unwrap();
            assert!(output.contains("Created wallet with address"));

            // Optionally, check that the key type is stored correctly
            assert_eq!(wallet.key_type, key_type_val.to_string());
        }
    }

    #[test]
    fn test_list_wallets() {
        let mut conn = setup_test_db();
        let mut buf = Vec::new();

        // Insert multiple wallets
        create_wallet(&mut conn, KeyType::Ecdsa, &mut buf).unwrap();
        create_wallet(&mut conn, KeyType::Eddsa, &mut buf).unwrap();

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

        create_wallet(&mut conn, KeyType::Ecdsa, &mut buf).unwrap();
        create_wallet(&mut conn, KeyType::Eddsa, &mut buf).unwrap();

        let results: Vec<Wallet> = wallets.load(&mut conn).unwrap();
        assert_eq!(results.len(), 2);

        clear_wallets(&mut conn).unwrap();

        let results: Vec<Wallet> = wallets.load(&mut conn).unwrap();
        assert_eq!(results.len(), 0);
    }
}
