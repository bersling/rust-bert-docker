use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::http::Method::Options;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        let origin = match _request.headers().get_one("origin") {
            Some(inner) => inner,
            None => "*"
        };
        let headers = if _request.method() == Options {
            _request.headers().get_one("Access-Control-Request-Headers").unwrap().to_string()
        } else {
            _request.headers().iter().map(|h| h.name.to_string()).collect::<Vec<String>>().join(", ")
        };
        response.set_header(Header::new("Access-Control-Allow-Origin", origin));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", headers));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}