use actix_web::{web, HttpResponse};
use diesel::{result, RunQueryDsl, SelectableHelper};

use crate::{
    db::init,
    models::post::{NewPost, Post},
};

pub async fn create_post(post: web::Json<NewPost>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let new_post = post.into_inner();

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

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, web, App};
    use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};

    use crate::{
        db,
        models::post::{NewPost, Post},
        routes::post::create_post_route,
        schema::posts::{dsl::id, dsl::posts},
    };

    #[actix_web::test]
    async fn test_create_post() {
        let app = test::init_service(App::new().service(create_post_route)).await;

        let new_post = NewPost {
            title: String::from("Any Title"),
            body: String::from("any body"),
            published: Some(true),
        };
        let payload = web::Json(new_post);

        let req = test::TestRequest::post()
            .uri("/posts/create")
            .set_json(&payload)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::CREATED);

        let body: serde_json::Value = test::read_body_json(res).await;

        assert_eq!(body["status"], "success");

        let post: Post =
            serde_json::from_value(body["post"].clone()).expect("failed to deserialize post");

        assert_eq!(post.title, "Any Title");
        assert_eq!(post.body, "any body");
        assert!(post.published);

        // cleanup after test is done
        diesel::delete(posts.filter(id.eq(post.id)))
            .execute(&mut db::init())
            .expect("failed to clean up posts table");
    }
}
