table! {
    transactions (id) {
        id -> Integer,
        user_id -> Integer,
        date_transaction -> Integer,
        sell_amount -> Integer,
        sell_currency -> Integer,
        buy_amount -> Integer,
        buy_currency -> Integer,
        price_for_one -> Integer,
    }
}

table! {
    user_sessions (id) {
        id -> Integer,
        user_id -> Integer,
        token -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        nickname -> Text,
        password -> Text,
        email -> Text,
    }
}

joinable!(transactions -> users (user_id));
joinable!(user_sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    transactions,
    user_sessions,
    users,
);
