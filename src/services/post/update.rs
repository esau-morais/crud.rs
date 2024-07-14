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

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, web, App};
    use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};

    use crate::{
        db,
        models::post::{Post, UpdatePost},
        routes::post::update_post_route,
        schema::posts::dsl::{id, posts},
        shared::post::shared_create_post,
    };

    #[actix_web::test]
    async fn test_update_post() {
        let app = test::init_service(App::new().service(update_post_route)).await;

        let updated_post = UpdatePost {
            title: Some(String::from("Any Title (edited)")),
            body: Some(String::from("any body (edited)")),
            published: Some(false),
            created_at: None,
            updated_at: None,
        };
        let payload = web::Json(updated_post);

        let recent_created_post_id = shared_create_post().await.id;
        let req = test::TestRequest::put()
            .uri(&format!("/posts/{}", recent_created_post_id))
            .set_json(&payload)
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);

        let body: serde_json::Value = test::read_body_json(res).await;

        assert_eq!(body["status"], "success");

        let post: Post =
            serde_json::from_value(body["post"].clone()).expect("failed to deserialize post");

        assert_eq!(post.title, "Any Title (edited)");
        assert_eq!(post.body, "any body (edited)");
        assert!(!post.published);

        // cleanup after test is done
        diesel::delete(posts.filter(id.eq(recent_created_post_id)))
            .execute(&mut db::init())
            .expect("failed to clean up posts table");
    }

    #[actix_web::test]
    async fn test_try_update_unexistent_post_by_id() {
        let app = test::init_service(App::new().service(update_post_route)).await;

        let recent_created_post_id = shared_create_post().await.id;

        // cleanup before test is done to test case in which post does not exist
        diesel::delete(posts.filter(id.eq(recent_created_post_id)))
            .execute(&mut db::init())
            .expect("failed to clean up posts table");

        let updated_post = UpdatePost {
            title: Some(String::from("Any Title (edited)")),
            body: Some(String::from("any body (edited)")),
            published: Some(false),
            created_at: None,
            updated_at: None,
        };
        let payload = web::Json(updated_post);

        let req = test::TestRequest::put()
            .uri(&format!("/posts/{}", recent_created_post_id))
            .set_json(&payload)
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
