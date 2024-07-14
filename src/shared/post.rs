use actix_web::{test, web, App};

use crate::models::post::{NewPost, Post};
use crate::routes::post::create_post_route;

pub async fn shared_create_post() -> Post {
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
    let body: serde_json::Value = test::read_body_json(res).await;
    serde_json::from_value(body["post"].clone()).expect("failed to deserialize post")
}
