use crate::db::schema::{evaluations, suggestion, users};
use diesel::prelude::*;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod suggestion_type {
    pub const CREATE: i32 = 1;
    pub const UPDATE: i32 = 2;
    pub const DELETE: i32 = 3;
}

#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = evaluations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Evaluation {
    pub id: i32,
    pub title: String,
    pub class: String,
    pub date: NaiveDateTime,
}
#[derive(Insertable, Queryable, Selectable, ToSchema)]
#[diesel(table_name = evaluations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEvaluation {
    pub title: String,
    pub class: String,
    pub date: NaiveDateTime,
}
#[derive(Deserialize, ToSchema)]
pub struct UpdateEvaluation {
    pub title: String,
    pub date: NaiveDateTime,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRead {
    pub id: i32,
    pub login: String,
}

#[derive(Insertable, ToSchema, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserUpdateInsert {
    pub login: String,
    pub password_hash: String,
}

#[derive(Insertable, ToSchema)]
#[diesel(table_name = suggestion)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SuggestionInsert {
    pub suggestion_type_id: i32,
    pub evaluation_id: Option<i32>,
    pub title: Option<String>,
    pub class: Option<String>,
    pub date: Option<NaiveDateTime>
}

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = suggestion)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SuggestionSelect {
    pub id: i32,
    pub suggestion_type_id: i32,
    pub evaluation_id: Option<i32>,
    pub title: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub class: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum SuggestionDetails {
    #[serde(rename_all = "camelCase")]
    Create {
        title: String,
        date: DateTime<Utc>,
        class: String,
    },

    #[serde(rename_all = "camelCase")]
    Update {
        evaluation_id: i32,
        title: String,
        date: DateTime<Utc>,
    },

    #[serde(rename_all = "camelCase")]
    Delete { evaluation_id: i32 },
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SuggestionModel {
    pub id: i32,
    pub details: SuggestionDetails,
}