use crate::schema::posts;
use chrono::NaiveDateTime;
use diesel::{pg::Pg, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(Pg))]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub published: Option<bool>,
    pub user_id: i32,
}

#[derive(AsChangeset, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = posts)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePost {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
}
