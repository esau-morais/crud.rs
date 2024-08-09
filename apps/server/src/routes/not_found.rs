use actix_web::HttpResponse;

pub async fn route_not_found() -> HttpResponse {
    HttpResponse::NotFound()
        .json(serde_json::json!({"status": "error", "message": "route not found"}))
}
