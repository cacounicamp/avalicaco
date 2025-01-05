use crate::db::schema::{evaluations, users};
use diesel::prelude::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
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
    pub class: String,
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
