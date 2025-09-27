// @generated automatically by Diesel CLI.

diesel::table! {
    signatures (id) {
        id -> Nullable<Integer>,
        wallet_id -> Integer,
        message -> Text,
        signature -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    wallets (id) {
        id -> Nullable<Integer>,
        address -> Text,
        private_key -> Text,
        created_at -> Timestamp,
        key_type -> Text,
    }
}

diesel::joinable!(signatures -> wallets (wallet_id));

diesel::allow_tables_to_appear_in_same_query!(signatures, wallets,);
