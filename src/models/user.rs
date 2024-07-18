use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::{pg::Pg, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub role: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub role: Option<String>,
    pub password: String,
}
