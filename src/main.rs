#[macro_use]
extern crate rocket;

use crate::middleware::cors::CORS;

mod controller;
mod service;
mod middleware;
mod env;

//noinspection ALL
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![controller::health_check::health_check])
        .mount("/api", routes![controller::haaskme::get_answer])
        .mount("/api", routes![controller::haaskme::get_answer_options])
        .attach(CORS)
}
