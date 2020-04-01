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

allow_tables_to_appear_in_same_query!(
    user_sessions,
    users,
);
