pub mod evaluation_db;
pub mod models;
pub mod schema;
pub mod suggestion_db;
pub mod prelude;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
