use actix_web::{get, HttpResponse};

#[get("/v0/health")]
pub async fn check_health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": "hello, world"}))
}
