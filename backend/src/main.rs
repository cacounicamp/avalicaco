mod db;
mod endpoints;
use endpoints::evaluations;
use endpoints::users;
use actix_web::{middleware::Logger, App, HttpServer};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use std::{env, error::Error};
use utoipa::OpenApi;
use utoipa_actix_web::{service_config::ServiceConfig, AppExt};
use utoipa_rapidoc::RapiDoc;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;

    HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .map(|app| app.wrap(Logger::default()))
            .configure(|config: &mut ServiceConfig| {
                config
                    .service(users::get)
                    .service(users::get_id)
                    .service(users::patch)
                    .service(users::delete)
                    .service(users::post)
                    .service(evaluations::get)
                    .service(evaluations::get_id)
                    .service(evaluations::patch)
                    .service(evaluations::delete)
                    .service(evaluations::post)
                ;
              },
            )
            .openapi_service(|api| 
                RapiDoc::with_openapi("/api-docs/openapi2.json", api)
                .path("/rapidoc")
            )
            .openapi_service(|api| Scalar::with_url("/scalar", api))
            .into_app()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
