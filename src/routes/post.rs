use actix_web::{delete, get, post, web, HttpResponse};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl, SelectableHelper};

use crate::{
    db::init,
    models::post::{NewPost, Post},
};

#[post("/posts/create")]
pub async fn create_post(req: web::Json<NewPost>) -> HttpResponse {
    use crate::schema::posts;

    let new_post = req.into_inner();

    let new_post_result = diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result::<Post>(&mut init());

    match new_post_result {
        Ok(post) => {
            let res = serde_json::json!({"status": "success", "post": post});
            return HttpResponse::Created().json(res);
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

#[delete("/posts/{id}")]
pub async fn delete_post(path: web::Path<i32>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let post_id = path.into_inner();

    let affected_rows = match diesel::delete(posts.filter(id.eq(post_id))).execute(&mut init()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let message = format!("post with ID {} not found", post_id);
                let res = serde_json::json!({"status": "error", "message": message});
                return HttpResponse::NotFound().json(res);
            }
            _ => {
                panic!("database error: {}", err);
            }
        },
    };

    if affected_rows == 0 {
        let message = format!("post with ID {} not found", post_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "error", "message": message}));
    };

    HttpResponse::NoContent().finish()
}
