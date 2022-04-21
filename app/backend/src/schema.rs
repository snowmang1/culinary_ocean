table! {
    recipes (id) {
        id -> Binary,
        user_email -> Text,
        prep -> Text,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        user_email -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    recipes,
    users,
);
