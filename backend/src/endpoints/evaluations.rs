use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
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
#[get("/evaluations")]
pub async fn get() -> impl Responder {
    use schema::evaluations::dsl::*;
    let connection = &mut establish_connection();
    let results = evaluations
        .select(Evaluation::as_select())
        .load(connection)
        .expect("Error loading evaluations");
    HttpResponse::Ok().json(results)
}

#[utoipa::path(
    responses(
        (status = 200, body = Evaluation),
        (status = 404)
    )
  )]
#[get("/evaluations/{id}")]
pub async fn get_id(path: web::Path<(i32,)>) -> impl Responder {
    use schema::evaluations::dsl::*;
    let (eid, ) = path.into_inner();
    
    let connection = &mut establish_connection();
    let result = evaluations
        .select(Evaluation::as_select())
        .filter(id.eq(eid))
        .first(connection);
    match result {
        Ok(eva) => HttpResponse::Ok().json(eva),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish()
    }
}


#[utoipa::path(
    responses(
        (status = 200),
    )
  )]
#[patch("/evaluations/{id}")]
pub async fn patch(path: web::Path<(i32,)>, body : web::Json<UpdateEvaluation>) -> impl Responder {
    use schema::evaluations::dsl::*;
    let (eid, ) = path.into_inner();
    let req = body.into_inner();
    let connection = &mut establish_connection();
    let result =diesel::update(evaluations.filter(id.eq(eid)))
        .set((
            class.eq(req.class),
            title.eq(req.title),
            date.eq(req.date)
        ))
        .execute(connection);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
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
#[post("/evaluations")]
pub async fn post(req_body: web::Json<PostEvaluationRequest>) -> impl Responder {
    let connection = &mut establish_connection();
    let ev = req_body.into_inner();
    let post = create_evaluation(connection, NewEvaluation {
        date: ev.date.naive_utc(),
        title: ev.title, 
        class: ev.class,
    });
    HttpResponse::Ok().json(post)
}


#[utoipa::path(
    responses(
        (status = 200),
    )
  )]
#[delete("/evaluations/{id}")]
pub async fn delete(path: web::Path<(i32,)>) -> impl Responder {
    use schema::evaluations::dsl::*;
    let (eid, ) = path.into_inner();
    let connection = &mut establish_connection();
    let result = diesel::delete(evaluations.filter(id.eq(eid)))
        .execute(connection);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
