// @generated automatically by Diesel CLI.

diesel::table! {
    orders (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 20]
        symbol -> Varchar,
        #[max_length = 4]
        side -> Varchar,
        price -> Numeric,
        quantity -> Numeric,
        filled_quantity -> Numeric,
        #[max_length = 20]
        status -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    user_balances (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 44]
        token_mint -> Varchar,
        available_balance -> Numeric,
        locked_balance -> Numeric,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        password_hash -> Text,
        #[max_length = 44]
        wallet_pubkey -> Varchar,
        wallet_privkey_enc -> Bytea,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        last_login_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(orders -> users (user_id));
diesel::joinable!(user_balances -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    orders,
    user_balances,
    users,
);
