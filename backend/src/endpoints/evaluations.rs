use crate::db::evaluation_db::{evaluation_create, evaluation_delete, evaluation_update};

use super::prelude::*;
#[utoipa::path(
    tag = tag::EVALUATIONS,
    responses(
        (status = 200, description = "List of evaluations", body = [Evaluation])
    ),
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
    tag = tag::EVALUATIONS,
    responses(
        (status = 200, body = Evaluation),
        (status = 404)
    )
  )]
#[get("/evaluations/{id}")]
pub async fn get_id(path: web::Path<(i32,)>) -> impl Responder {
    use schema::evaluations::dsl::*;
    let (eid,) = path.into_inner();

    let connection = &mut establish_connection();
    let result = evaluations
        .select(Evaluation::as_select())
        .filter(id.eq(eid))
        .first(connection);
    match result {
        Ok(eva) => HttpResponse::Ok().json(eva),
        Err(DieselError::NotFound) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    tag = tag::EVALUATIONS,
    responses(
        (status = 200),
    ),
    security(("jwt" = []))
  )]
#[patch("/evaluations/{id}", wrap = "RequireAuth")]
pub async fn patch(path: web::Path<(i32,)>, body: web::Json<UpdateEvaluation>) -> impl Responder {
    let (id,) = path.into_inner();
    let req = body.into_inner();
    let connection = &mut establish_connection();
    let result = evaluation_update(connection, id, req);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(ToSchema, Deserialize)]
pub struct PostEvaluationRequest {
    pub title: String,
    pub class: String,
    pub date: DateTime<Utc>,
}

#[utoipa::path(
    tag = tag::EVALUATIONS,
    responses(
        (status = 200, description = "The created evaluation", body = Evaluation)
    ),
    security(("jwt" = []))
)]
#[post("/evaluations", wrap = "RequireAuth")]
pub async fn post(req_body: web::Json<PostEvaluationRequest>) -> impl Responder {
    let connection = &mut establish_connection();
    let ev = req_body.into_inner();
    let post = evaluation_create(
        connection,
        NewEvaluation {
            date: ev.date.naive_utc(),
            title: ev.title,
            class: ev.class,
        },
    ).unwrap();
    HttpResponse::Ok().json(post)
}

#[utoipa::path(
    tag = tag::EVALUATIONS,
    responses(
        (status = 200),
    ),
    security(("jwt" = []))
)]
#[delete("/evaluations/{id}", wrap = "RequireAuth")]
pub async fn delete(path: web::Path<(i32,)>) -> impl Responder {
    let (eid,) = path.into_inner();
    let connection = &mut establish_connection();
    let result = evaluation_delete(connection, eid);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
