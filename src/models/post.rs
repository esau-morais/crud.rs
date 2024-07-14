use crate::schema::posts;
use chrono::NaiveDateTime;
use diesel::{pg::Pg, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub published: Option<bool>,
}

#[derive(AsChangeset, Insertable, Deserialize, Debug)]
#[diesel(table_name = posts)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
}
