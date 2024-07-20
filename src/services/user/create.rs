use actix_web::{web, HttpResponse};
use diesel::{result, RunQueryDsl, SelectableHelper};

use crate::{
    core::config::db::init_db,
    models::user::{NewUser, User},
};

pub async fn create_user(user: web::Json<NewUser>) -> HttpResponse {
    use crate::schema::users::dsl::*;

    let new_user = user.into_inner();

    let new_user_result: Result<User, result::Error> = diesel::insert_into(users)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result::<User>(&mut init_db().get().unwrap());

    match new_user_result {
        Ok(user) => {
            let res = serde_json::json!({"status": "success", "post": user});
            HttpResponse::Created().json(res)
        }
        Err(err) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error", "message": format!("{:?}", err)})),
    }
}
