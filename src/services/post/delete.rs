use actix_web::{web, HttpResponse};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::db::init;

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

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, web, App};
    use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};

    use crate::{
        db,
        schema::posts::dsl::{id, posts},
        shared::post::shared_create_post,
    };

    #[actix_web::test]
    async fn test_delete_post_by_id() {
        let app = test::init_service(
            App::new().service(
                web::resource("/posts/{id}")
                    .route(web::delete().to(crate::services::post::delete::delete_post)),
            ),
        )
        .await;

        let recent_created_post_id = shared_create_post().await.id;
        let req = test::TestRequest::delete()
            .uri(&format!("/posts/{}", recent_created_post_id))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::NO_CONTENT);

        // cleanup after test is done
        diesel::delete(posts.filter(id.eq(recent_created_post_id)))
            .execute(&mut db::init())
            .expect("failed to clean up posts table");
    }

    #[actix_web::test]
    async fn test_try_delete_unexistent_post() {
        let app = test::init_service(
            App::new().service(
                web::resource("/posts/{id}")
                    .route(web::delete().to(crate::services::post::delete::delete_post)),
            ),
        )
        .await;

        let recent_created_post_id = shared_create_post().await.id;

        // cleanup before test is done to test case in which post does not exist
        diesel::delete(posts.filter(id.eq(recent_created_post_id)))
            .execute(&mut db::init())
            .expect("failed to clean up posts table");

        let req = test::TestRequest::delete()
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
