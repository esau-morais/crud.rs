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

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, App};
    use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};

    use crate::{
        db,
        models::post::Post,
        routes::post::get_post_by_id_route,
        schema::posts::dsl::{id, posts},
        shared::post::shared_create_post,
    };

    #[actix_web::test]
    async fn test_get_post_by_id() {
        let app = test::init_service(App::new().service(get_post_by_id_route)).await;

        let recent_created_post_id = shared_create_post().await.id;
        let req = test::TestRequest::get()
            .uri(&format!("/posts/{}", recent_created_post_id))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);

        let body: serde_json::Value = test::read_body_json(res).await;

        assert_eq!(body["status"], "success");

        let post: Post =
            serde_json::from_value(body["post"].clone()).expect("failed to deserialize posts");

        assert_eq!(post.title, "Any Title");
        assert_eq!(post.body, "any body");
        assert!(post.published);

        // cleanup after test is done
        diesel::delete(posts.filter(id.eq(post.id)))
            .execute(&mut db::init())
            .expect("failed to clean up posts table");
    }

    #[actix_web::test]
    async fn test_get_unexistent_post_by_id() {
        let app = test::init_service(App::new().service(get_post_by_id_route)).await;

        let recent_created_post_id = shared_create_post().await.id;

        // cleanup before test is done to test case in which post does not exist
        diesel::delete(posts.filter(id.eq(recent_created_post_id)))
            .execute(&mut db::init())
            .expect("failed to clean up posts table");

        let req = test::TestRequest::get()
            .uri(&format!("/posts/{}", recent_created_post_id))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_client_error());
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        let body: serde_json::Value = test::read_body_json(res).await;

        assert_eq!(body["status"], "error");
        assert_eq!(
            body["message"],
            format!("post with ID {} not found", recent_created_post_id),
        );
    }
}
