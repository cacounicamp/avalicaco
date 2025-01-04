// @generated automatically by Diesel CLI.

diesel::table! {
    evaluations (id) {
        id -> Int4,
        title -> Varchar,
        class -> Text,
        date -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        login -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    evaluations,
    users,
);
