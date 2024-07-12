use actix_web::{get, post, web, HttpResponse};
use diesel::{RunQueryDsl, SelectableHelper};

use crate::{
    db::init,
    models::post::{NewPost, Post},
};

#[post("/posts/create")]
pub async fn create_post(req: web::Json<NewPost>) -> HttpResponse {
    use crate::schema::posts;

    let new_post = NewPost {
        title: req.title.to_string(),
        body: req.body.to_string(),
    };
    let new_post_result = diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result::<Post>(&mut init());

    match new_post_result {
        Ok(post) => {
            let res = serde_json::json!({"status": "success", "post": post});
            return HttpResponse::Ok().json(res);
        }
        Err(err) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", err)}))
        }
    }
}

#[get("/posts")]
pub async fn get_posts() -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let posts_result = posts.load::<Post>(&mut init());

    match posts_result {
        Ok(all_posts) => {
            let res = serde_json::json!({"status": "success", "posts": all_posts});
            return HttpResponse::Ok().json(res);
        }
        Err(err) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", err)}))
        }
    }
}
