use diesel::{r2d2::ConnectionManager, PgConnection};

use super::error::CustomError;

pub type AppResult<T> = Result<T, CustomError>;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PsqlConn = Pool<PgConnection>;
