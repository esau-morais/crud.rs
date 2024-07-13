use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::Utc;
use diesel::{result, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    db::init,
    models::post::{NewPost, Post, UpdatePost},
};

#[post("/posts/create")]
pub async fn create_post(req: web::Json<NewPost>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let new_post = req.into_inner();

    let new_post_result: Result<Post, result::Error> = diesel::insert_into(posts)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result::<Post>(&mut init());

    match new_post_result {
        Ok(post) => {
            let res = serde_json::json!({"status": "success", "post": post});
            HttpResponse::Created().json(res)
        }
        Err(err) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error", "message": format!("{:?}", err)})),
    }
}

#[get("/posts")]
pub async fn get_posts() -> HttpResponse {
    use crate::schema::posts::dsl::*;

    // TODO: check Result is of type unknown
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

#[get("/posts/{id}")]
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

#[put("/posts/{id}")]
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

#[delete("/posts/{id}")]
pub async fn delete_post(path: web::Path<i32>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let post_id = path.into_inner();

    let affected_rows: usize = match diesel::delete(posts.filter(id.eq(post_id)))
        .execute(&mut init())
    {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let message = format!("post with ID {} not found", post_id);
                let res = serde_json::json!({"status": "error", "message": message});
                return HttpResponse::NotFound().json(res);
            }
            _ => {
                return HttpResponse::InternalServerError()
                    .json(serde_json::json!({"status": "error", "message": format!("{:?}", err)}))
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
