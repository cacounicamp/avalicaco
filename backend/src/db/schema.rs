// @generated automatically by Diesel CLI.

diesel::table! {
    evaluations (id) {
        id -> Int4,
        title -> Varchar,
        class -> Text,
        date -> Timestamp,
    }
}