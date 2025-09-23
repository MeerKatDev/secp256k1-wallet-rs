use crate::db::establish_connection;
use crate::models::{NewSignature, Signature, Wallet};
use crate::schema::{signatures, wallets};
use diesel::prelude::*;

pub fn sign_message(wallet_id: i32, msg: &str) -> anyhow::Result<()> {
    let conn = &mut establish_connection();
    let wallet: Wallet = wallets::table
        .filter(wallets::id.eq(wallet_id))
        .first(conn)?;

    let sig_hex = make_ecdsa_signature(&wallet.private_key, msg)?;

    let new_sig = NewSignature {
        wallet_id,
        message: msg,
        signature: &sig_hex,
    };

    diesel::insert_into(signatures::table)
        .values(&new_sig)
        .execute(conn)?;

    println!("Signature: {sig_hex}");
    Ok(())
}

fn make_ecdsa_signature(priv_key: &str, msg: &str) -> anyhow::Result<String> {
    use secp256k1::hashes::{sha256, Hash};
    use secp256k1::{Message, Secp256k1, SecretKey};

    let mut secret_bytes = [0u8; 32];
    hex::decode_to_slice(priv_key, &mut secret_bytes)?;
    let secret = SecretKey::from_byte_array(secret_bytes)?;
    let secp = Secp256k1::new();

    let digest = sha256::Hash::hash(msg.as_bytes());
    let message = Message::from_digest(digest.to_byte_array());
    let sig = secp.sign_ecdsa(message, &secret);

    Ok(hex::encode(sig.serialize_der()))
}

pub fn list_signatures(wallet_id: i32) -> anyhow::Result<()> {
    let conn = &mut establish_connection();
    let results: Vec<Signature> = signatures::table
        .filter(signatures::wallet_id.eq(wallet_id))
        .load(conn)?;

    for s in results {
        println!(
            "ID: {:?}, Message: {}, Sig: {}",
            s.id, s.message, s.signature
        );
    }
    Ok(())
}
