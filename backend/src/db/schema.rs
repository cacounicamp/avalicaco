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
    suggestion (id) {
        id -> Int4,
        suggestion_type_id -> Int4,
        evaluation_id -> Nullable<Int4>,
        title -> Nullable<Varchar>,
        class -> Nullable<Varchar>,
        date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    suggestion_type (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        login -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::joinable!(suggestion -> evaluations (evaluation_id));
diesel::joinable!(suggestion -> suggestion_type (suggestion_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    evaluations,
    suggestion,
    suggestion_type,
    users,
);
