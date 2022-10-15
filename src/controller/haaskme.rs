use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use rocket::serde::Serialize;

use crate::env::env::haaskme_apikey;
use crate::service::haaskme::make_gpt3_request;

pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key == haaskme_apikey()
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)),
        }
    }
}

#[derive(Serialize)]
pub struct HaaskmeResponse {
    answer: String,
}

#[get("/haaskme/<search_text>")]
pub async fn get_answer(search_text: String, key: ApiKey<'_>) -> Json<HaaskmeResponse> {
    // OPTIMIZE: some auth could go here
    Json(HaaskmeResponse {
        answer: make_gpt3_request(search_text).await
    })
}

#[options("/haaskme/<search_text>")]
pub async fn get_answer_options(search_text: String) -> String {
    format!("")
}

