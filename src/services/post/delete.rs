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
