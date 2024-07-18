use actix_web::HttpResponse;

pub async fn health_checker() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": "hello, world"}))
}
