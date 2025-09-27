use crate::db::schema::{signatures, wallets};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = wallets)]
pub struct Wallet {
    pub id: Option<i32>,
    pub address: String,
    pub private_key: String,
    pub created_at: NaiveDateTime,
    pub key_type: String,
}

#[derive(Insertable)]
#[diesel(table_name = wallets)]
pub struct NewWallet<'a> {
    pub address: &'a str,
    pub private_key: &'a str,
    pub key_type: String,
}

#[derive(Queryable, Associations, Identifiable, Debug)]
#[diesel(table_name = signatures)]
#[diesel(belongs_to(Wallet))]
pub struct Signature {
    pub id: Option<i32>,
    pub wallet_id: i32,
    pub message: String,
    pub signature: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = signatures)]
pub struct NewSignature<'a> {
    pub wallet_id: i32,
    pub message: &'a str,
    pub signature: &'a str,
}
