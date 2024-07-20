use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

use crate::core::types::PsqlConn;

pub fn init_db() -> PsqlConn {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenvy::dotenv().expect("environment variables must be set");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn_manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(conn_manager)
        .expect("error building connection pool")
}
