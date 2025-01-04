mod db;
mod endpoints;
use endpoints::evaluations;
use actix_web::{middleware::Logger, App, HttpServer};
use std::{env, error::Error};
use utoipa::OpenApi;
use utoipa_actix_web::{service_config::ServiceConfig, AppExt};
use utoipa_swagger_ui::SwaggerUi;

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
                  .service(evaluations::get)
                  .service(evaluations::get_id)
                  .service(evaluations::patch)
                  .service(evaluations::delete)
                  .service(evaluations::post);
              },
            )
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
