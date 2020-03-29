table! {
    user_sessions (id) {
        id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        token -> Text,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        nickname -> Text,
        password -> Text,
        email -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    user_sessions,
    users,
);
