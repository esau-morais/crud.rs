use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn init() -> PgConnection {
    dotenvy::dotenv().expect("environment variables must be set");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("error connecting to {}", database_url))
}
