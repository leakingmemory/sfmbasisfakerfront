use std::rc::Rc;
use dioxus::prelude::*;
use crate::{config, patient, patient_list};
use crate::patient_list::PatientInList;

#[derive(serde::Deserialize, Clone)]
struct HealthResponse {
    status: String
}

async fn fetch_health_status(base_url: String) -> Result<String, reqwest::Error> {
    let url = format!("{}/health", base_url);
    let response = reqwest::get(url).await?;
    let health: HealthResponse = response.json().await?;
    Ok(health.status)
}

pub fn faker_main(config: config::Config) -> Element {
    let base_url = config.base_uri.clone();
    let status = use_resource(move || {
        let url = base_url.clone();
        async move {
            fetch_health_status(url).await
        }
    });

    let mut current_patient = use_signal(|| patient_list::PatientInList {id : "".to_string(), firstName: "".to_string(), lastName: "".to_string()});

    rsx! {
        match &*status.read_unchecked() {
            None => rsx!(p { "Status: querying for status" }),
            Some(Ok(health)) => rsx!(p { "Status: {health}" }),
            Some(Err(_)) => rsx!(p { "Status: failed to fetch health status" })
        },
        if current_patient().id == "" {
            patient_list::patient_list { base_uri: config.base_uri, onselect: move |sel_patient: patient_list::PatientInList| {
                current_patient.set(sel_patient);
            }}
        } else {
            patient::patient { base_uri: config.base_uri, patient: current_patient()}
        }
    }
}