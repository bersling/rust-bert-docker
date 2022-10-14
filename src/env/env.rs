use std::env;

pub fn gpt3_haaskmekey() -> String {
    env::var("GPT3_HAASKMEKEY").unwrap().to_owned()
}

pub fn haaskme_apikey() -> String {
    env::var("HAASKME_APIKEY").unwrap().to_owned()
}