use actix_web::{web, HttpResponse};
use chrono::Utc;
use diesel::{result, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    db::init,
    models::post::{Post, UpdatePost},
};

pub async fn update_post(path: web::Path<i32>, req: web::Json<UpdatePost>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let post_id = path.into_inner();
    let current_post = req.into_inner();
    let now = Utc::now().naive_utc();

    let updated_post: Result<Post, result::Error> = diesel::update(posts.filter(id.eq(post_id)))
        .set(&UpdatePost {
            updated_at: Some(now),
            ..current_post
        })
        .get_result::<Post>(&mut init());

    match updated_post {
        Ok(post) => {
            let res = serde_json::json!({"status": "success", "post": post});
            HttpResponse::Ok().json(res)
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let message = format!("post with ID {} not found", post_id);
                let res = serde_json::json!({"status": "error", "message": message});
                HttpResponse::NotFound().json(res)
            }
            _ => HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", err)})),
        },
    }
}
