use actix_web::{get, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::web::Json;
use crate::models::ltp_response::LtpListResponse;

/// Route to get Last Time Price assets
#[get("/ltp")]
async fn get_ltp() -> HttpResponse {

    let fetch_result = LtpListResponse::default();

    HttpResponse::Ok()
        .status(StatusCode::OK)
        .content_type("application/json")
        .body(
            serde_json::to_string(&Json(fetch_result))
                .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
        )
}