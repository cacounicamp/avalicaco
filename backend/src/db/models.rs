use diesel::prelude::*;
use crate::db::schema::evaluations;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::NaiveDateTime;
#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = evaluations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Evaluation {
    pub id: i32,
    pub title: String,
    pub class : String,
    pub date : NaiveDateTime
}
#[derive(Insertable, Queryable, Selectable, ToSchema)]
#[diesel(table_name = evaluations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEvaluation {
    pub title: String,
    pub class : String,
    pub date : NaiveDateTime
}
 