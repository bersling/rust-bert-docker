use rust_bert::pipelines::sequence_classification::Label;
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;

use blocking::unblock;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    is_healthy: bool,
    version: String
}

#[derive(Serialize)]
pub struct ZeroShotResponse {
    zero_shot_response: Vec<Vec<Label>>
}


#[get("/health-check")]
pub fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        is_healthy: true,
        version: "2".to_owned(),
    })
}

#[get("/health-check-zero-shot")]
pub async fn health_check_zero_shot() -> Json<ZeroShotResponse> {


    let result: Vec<Vec<Label>> = unblock(|| {
        let sequence_classification_model = ZeroShotClassificationModel::new(Default::default()).unwrap();

        let input_sentence = "Who are you voting for in 2020?";
        let input_sequence_2 = "The prime minister has announced a stimulus package which was widely criticized by the opposition.";
        let candidate_labels = &["politics", "public health", "economy", "sports"];

        sequence_classification_model.predict_multilabel(
            &[input_sentence, input_sequence_2],
            candidate_labels,
            Some(Box::new(|label: &str| {
                format!("This example is about {}.", label)
            })),
            128,
        )
    }).await;

    return Json(ZeroShotResponse{
        zero_shot_response: result
    });
}
