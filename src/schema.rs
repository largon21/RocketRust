table! {
    users (id) {
        id -> Nullable<Integer>,
        nickname -> Text,
        password -> Text,
        email -> Text,
        token -> Nullable<Integer>,
    }
}
