use blocking::unblock;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rust_bert::distilbert::{DistilBertConfigResources, DistilBertModelResources, DistilBertVocabResources};
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::common::ModelType::DistilBert;
use rust_bert::pipelines::sequence_classification::Label;
use rust_bert::pipelines::zero_shot_classification::{ZeroShotClassificationConfig, ZeroShotClassificationModel, ZeroShotClassificationOption};
use rust_bert::resources::RemoteResource;
use tch::Device;

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

        // FIXME: this isn't working, error:
        // thread 'blocking-6' panicked at 'id2label must be provided for classifiers', /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/rust-bert-0.19.0/src/distilbert/distilbert_model.rs:303:14
        // let config = ZeroShotClassificationConfig {
        //     model_type: ModelType::DistilBert,
        //     model_resource: Box::new(RemoteResource::from_pretrained(
        //         DistilBertModelResources::DISTIL_BERT,
        //     )),
        //     config_resource: Box::new(RemoteResource::from_pretrained(
        //         DistilBertConfigResources::DISTIL_BERT,
        //     )),
        //     vocab_resource: Box::new(RemoteResource::from_pretrained(
        //         DistilBertVocabResources::DISTIL_BERT,
        //     )),
        //     merges_resource: None, // is this ok?
        //     lower_case: false,
        //     strip_accents: None,
        //     add_prefix_space: None,
        //     device: Device::cuda_if_available()
        // };

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
