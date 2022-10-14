use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    is_healthy: bool,
    version: String
}

#[get("/health-check")]
pub fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        is_healthy: true,
        version: "2".to_owned(),
    })
}
