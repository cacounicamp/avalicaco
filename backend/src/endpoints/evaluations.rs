use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;
use crate::db::{create_evaluation, establish_connection, models::*, schema};
use diesel::prelude::*;

#[utoipa::path(
    responses(
        (status = 200, description = "List of evaluations", body = [Evaluation])
    )
  )]
#[get("")]
pub async fn get() -> impl Responder {
    use schema::evaluations::dsl::*;
    let connection = &mut establish_connection();
    let results = evaluations
        .select(Evaluation::as_select())
        .load(connection)
        .expect("Error loading evaluations");
    HttpResponse::Ok().json(results)
}

#[derive(ToSchema, Deserialize)]
pub struct PostEvaluationRequest {
    pub title: String,
    pub class : String,
    pub date : DateTime<Utc>
}
 
#[utoipa::path(
  responses(
      (status = 200, description = "The created evaluation", body = Evaluation)
  )
)]
#[post("")]
pub async fn post(req_body: web::Json<PostEvaluationRequest>) -> impl Responder {
    let connection = &mut establish_connection();
    let post = create_evaluation(connection, NewEvaluation {
        title: req_body.title.clone(), 
        class: req_body.class.clone(),
        date: req_body.date.naive_utc()
    });
    HttpResponse::Ok().json(post)
}
