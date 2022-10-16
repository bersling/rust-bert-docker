use std::env;

pub fn some_envkey() -> String {
    env::var("SOME_KEY").unwrap().to_owned()
}
