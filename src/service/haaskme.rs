use hyper::{Body, Client, header, Request};
use hyper::body::Buf;
use hyper_tls::HttpsConnector;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::env::env::gpt3_haaskmekey;

#[derive(Serialize, Deserialize)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

#[derive(Serialize, Deserialize)]
struct OAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>,
}

#[derive(Serialize, Deserialize)]
struct OAIRequest {
    prompt: String,
    max_tokens: u32,
}


pub async fn make_gpt3_request(user_text: String) -> String {
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";

    let preamble = ""; // no preamble, pure GPT3 power!!!
    let oai_token = gpt3_haaskmekey();
    let auth_header_val = format!("Bearer {}", oai_token);

    let oai_request = OAIRequest {
        prompt: format!("{} {}", preamble, user_text),
        max_tokens: 100,
    };

    let body = Body::from(serde_json::to_vec(&oai_request).expect("Could not deserialize request"));
    let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, &auth_header_val)
        .body(body)
        .unwrap();
    let res = client.request(req).await.expect("request failed");
    let body = hyper::body::aggregate(res).await.expect("could not aggregate body");
    let json: OAIResponse = serde_json::from_reader(body.reader()).unwrap();

    let first_choice = &json.choices[0];

    // OPTIMIZE: how's that performance wise? What would happen in a java program?!
    return first_choice.text.clone();
}
