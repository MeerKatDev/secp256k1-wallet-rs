use crate::key_type::KeyType;
use crate::models::{NewSignature, Signature, Wallet};
use crate::schema::{signatures, wallets};
use diesel::prelude::*;
use std::io::Write;

/// Signs a message with the given wallet ID and writes output to `out`.
pub fn sign_message<W: Write>(
    conn: &mut SqliteConnection,
    wallet_id: i32,
    msg: &str,
    out: &mut W,
) -> anyhow::Result<()> {
    let wallet: Wallet = wallets::table
        .filter(wallets::id.eq(wallet_id))
        .first(conn)?;

    let mut secret_bytes = [0u8; 32];
    hex::decode_to_slice(wallet.private_key, &mut secret_bytes)?;

    let sig_hex = match wallet.key_type.into() {
        KeyType::Ecdsa => make_ecdsa_signature(&secret_bytes, msg.as_bytes()),
        KeyType::Eddsa => make_eddsa_signature(&secret_bytes, msg.as_bytes()),
    }?;

    let new_sig = NewSignature {
        wallet_id,
        message: msg,
        signature: &sig_hex,
    };

    diesel::insert_into(signatures::table)
        .values(&new_sig)
        .execute(conn)?;

    writeln!(out, "Signature: {sig_hex}")?;

    Ok(())
}

/// Lists signatures for a wallet, writing output to `out`.
pub fn list_signatures<W: Write>(
    conn: &mut SqliteConnection,
    wallet_id: i32,
    out: &mut W,
) -> anyhow::Result<()> {
    let results: Vec<Signature> = signatures::table
        .filter(signatures::wallet_id.eq(wallet_id))
        .load(conn)?;

    writeln!(out, "Signatures:")?;
    for s in results {
        writeln!(
            out,
            "ID: {:?}, Message: {}, Sig: {}",
            s.id, s.message, s.signature
        )?;
    }

    Ok(())
}

/// Removes all signatures from the DB.
pub fn clear_signatures(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    diesel::delete(signatures::table).execute(conn)?;
    Ok(())
}

/// Removes signatures for a specific wallet.
pub fn clear_signatures_for_wallet(
    conn: &mut SqliteConnection,
    wallet_id: i32,
) -> anyhow::Result<()> {
    diesel::delete(signatures::table.filter(signatures::wallet_id.eq(wallet_id))).execute(conn)?;
    Ok(())
}

fn make_ecdsa_signature(priv_key: &[u8; 32], msg: &[u8]) -> anyhow::Result<String> {
    use secp256k1::hashes::{sha256, Hash};
    use secp256k1::{Message, Secp256k1, SecretKey};

    let secret = SecretKey::from_byte_array(*priv_key)?;
    let secp = Secp256k1::new();

    let digest = sha256::Hash::hash(msg);
    let message = Message::from_digest(digest.to_byte_array());
    let sig = secp.sign_ecdsa(message, &secret);

    Ok(hex::encode(sig.serialize_der()))
}

fn make_eddsa_signature(priv_key: &[u8; 32], msg: &[u8]) -> anyhow::Result<String> {
    use ed25519_dalek::{Signer, SigningKey};

    let signing_key: SigningKey = SigningKey::from_bytes(priv_key);
    let sig = signing_key.sign(msg);
    Ok(hex::encode(sig.to_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Wallet;
    use crate::signature::wallets::dsl::*;

    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    fn setup_test_db() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        conn
    }

    fn insert_test_wallet<W: Write>(conn: &mut SqliteConnection, out: &mut W) -> Wallet {
        crate::wallet::create_wallet(conn, KeyType::Ecdsa, out).unwrap();
        wallets.order(id.desc()).first(conn).unwrap()
    }

    #[test]
    fn test_sign_message_inserts_signature() {
        let mut conn = setup_test_db();
        let mut out = Vec::new();

        let wallet = insert_test_wallet(&mut conn, &mut out);

        sign_message(&mut conn, wallet.id.unwrap(), "hello world", &mut out).unwrap();

        let sigs: Vec<Signature> = signatures::table.load(&mut conn).unwrap();
        assert_eq!(sigs.len(), 1);
        assert_eq!(sigs[0].message, "hello world");

        let output = String::from_utf8(out).unwrap();
        assert!(output.contains("Signature:"));
    }

    #[test]
    fn test_list_signatures_prints_entries() {
        let mut conn = setup_test_db();
        let mut out = Vec::new();
        let wallet = insert_test_wallet(&mut conn, &mut out);

        sign_message(&mut conn, wallet.id.unwrap(), "msg1", &mut out).unwrap();
        sign_message(&mut conn, wallet.id.unwrap(), "msg2", &mut out).unwrap();

        list_signatures(&mut conn, wallet.id.unwrap(), &mut out).unwrap();

        let output = String::from_utf8(out).unwrap();
        assert!(output.contains("Signatures:"));
        assert!(output.contains("msg1"));
        assert!(output.contains("msg2"));
    }

    #[test]
    fn test_clear_signatures_removes_all() {
        let mut conn = setup_test_db();
        let mut out = Vec::new();
        let wallet = insert_test_wallet(&mut conn, &mut out);

        sign_message(&mut conn, wallet.id.unwrap(), "msg", &mut out).unwrap();
        clear_signatures(&mut conn).unwrap();

        let sigs: Vec<Signature> = signatures::table.load(&mut conn).unwrap();
        assert!(sigs.is_empty());
    }

    #[test]
    fn test_clear_signatures_for_wallet() {
        let mut conn = setup_test_db();
        let mut out = Vec::new();
        let wallet1 = insert_test_wallet(&mut conn, &mut out);
        let wallet2 = insert_test_wallet(&mut conn, &mut out);

        sign_message(&mut conn, wallet1.id.unwrap(), "msg1", &mut out).unwrap();
        sign_message(&mut conn, wallet2.id.unwrap(), "msg2", &mut out).unwrap();

        clear_signatures_for_wallet(&mut conn, wallet1.id.unwrap()).unwrap();

        let sigs: Vec<Signature> = signatures::table.load(&mut conn).unwrap();
        assert_eq!(sigs.len(), 1);
        assert_eq!(sigs[0].wallet_id, wallet2.id.unwrap());
    }
}
