use actix_web::{web, HttpResponse};
use diesel::{result, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{db::init, models::post::Post};

pub async fn get_posts() -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let posts_result: Result<Vec<Post>, result::Error> = posts.load::<Post>(&mut init());

    match posts_result {
        Ok(all_posts) => {
            let res = serde_json::json!({"status": "success", "posts": all_posts});
            HttpResponse::Ok().json(res)
        }
        Err(err) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error", "message": format!("{:?}", err)})),
    }
}

pub async fn get_post_by_id(path: web::Path<i32>) -> HttpResponse {
    use crate::schema::posts;
    use diesel::OptionalExtension;

    let post_id = path.into_inner();
    let post_result: Result<Option<Post>, result::Error> = posts::table
        .find(post_id)
        .select(Post::as_select())
        .first(&mut init())
        .optional();

    match post_result {
        Ok(Some(post)) => {
            let res = serde_json::json!({"status": "success", "post": post});
            HttpResponse::Ok().json(res)
        }
        Ok(None) => {
            let message = format!("post with ID {} not found", post_id);
            HttpResponse::NotFound()
                .json(serde_json::json!({"status": "error", "message": message}))
        }
        Err(err) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error", "message": format!("{:?}", err)})),
    }
}
