use crate::db::suggestion_db::{suggestion_apply, suggestion_select_by_id};

use super::prelude::*;

#[utoipa::path(
    tag = tag::SUGGESTIONS,
    responses(
        (status = 200, description = "The created evaluation", body = SuggestionDetails)
    ),
    security(("jwt" = []))
)]
#[post("/suggestions", wrap = "RequireAuth")]
pub async fn post(req_body: web::Json<SuggestionDetails>) -> impl Responder {
    let connection = &mut establish_connection();
    let req = req_body.into_inner();
    let suggestion = match req {
        SuggestionDetails::Create { title, date, class } => SuggestionInsert {
            suggestion_type_id: suggestion_type::CREATE,
            evaluation_id: None,
            title: Some(title),
            date: Some(date.naive_utc()),
            class: Some(class),
        },
        SuggestionDetails::Update {
            evaluation_id,
            title,
            date,
        } => SuggestionInsert {
            suggestion_type_id: suggestion_type::UPDATE,
            evaluation_id: Some(evaluation_id),
            title: Some(title),
            date: Some(date.naive_utc()),
            class: None,
        },
        SuggestionDetails::Delete { evaluation_id } => SuggestionInsert {
            suggestion_type_id: suggestion_type::DELETE,
            evaluation_id: Some(evaluation_id),
            title: None,
            date: None,
            class: None,
        },
    };

    let result = diesel::insert_into(schema::suggestion::table)
        .values(&suggestion)
        .execute(connection);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

fn select_to_details(sug: SuggestionSelect) -> SuggestionDetails {
    match sug.suggestion_type_id {
        suggestion_type::CREATE => SuggestionDetails::Create {
            title: sug.title.unwrap(),
            date: sug.date.unwrap().and_utc(),
            class: sug.class.unwrap(),
        },
        suggestion_type::UPDATE => SuggestionDetails::Update {
            evaluation_id: sug.evaluation_id.unwrap(),
            title: sug.title.unwrap(),
            date: sug.date.unwrap().and_utc(),
        },
        suggestion_type::DELETE => SuggestionDetails::Delete {
            evaluation_id: sug.evaluation_id.unwrap(),
        },
        _ => unreachable!("suggestion_type don't exist"),
    }
}

fn select_to_response(select: SuggestionSelect) -> SuggestionModel {
    SuggestionModel {
        id: select.id,
        details: select_to_details(select),
    }
}
#[utoipa::path(
    tag = tag::SUGGESTIONS,
    responses(
        (status = 200, description = "List of suggestions", body = [SuggestionModel])
    ),
    security(("jwt" = []))
  )]
#[get("/suggestions", wrap = "RequireAuth")]
pub async fn get() -> impl Responder {
    use schema::suggestion::dsl::*;
    let connection = &mut establish_connection();
    let results: Vec<SuggestionModel> = suggestion
        .select(SuggestionSelect::as_select())
        .load(connection)
        .expect("Error loading users")
        .into_iter()
        .map(select_to_response)
        .collect();

    HttpResponse::Ok().json(results)
}

#[utoipa::path(
    tag = tag::SUGGESTIONS,
    responses(
        (status = 200, description = "The suggestion", body = [SuggestionModel])
    ),
    security(("jwt" = []))
  )]
#[get("/suggestions/{id}", wrap = "RequireAuth")]
pub async fn get_by_id(path: web::Path<(i32,)>) -> impl Responder {
    let (id,) = path.into_inner();
    let connection = &mut establish_connection();
    let result = suggestion_select_by_id(connection, id);

    match result {
        Ok(select) => HttpResponse::Ok().json(select_to_response(select)),
        Err(DieselError::NotFound) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    tag = tag::SUGGESTIONS,
    responses(
        (status = 200),
    ),
    security(("jwt" = []))
  )]
#[patch("/suggestions/{id}/apply", wrap = "RequireAuth")]
pub async fn patch(path: web::Path<(i32,)>) -> impl Responder {
    use schema::suggestion::dsl::*;
    let (iid,) = path.into_inner();
    let connection = &mut establish_connection();
    let result = suggestion
        .select(SuggestionSelect::as_select())
        .filter(id.eq(iid))
        .first(connection);

    match result {
        Err(_) => HttpResponse::InternalServerError().finish(),
        Ok(select) => {
            let sug = select_to_response(select);
            suggestion_apply(connection, sug);
            HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("Suggestion Applied")
        }
    }
}
