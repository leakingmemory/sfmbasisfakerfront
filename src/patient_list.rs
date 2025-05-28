use dioxus::prelude::*;
use crate::config;

#[derive(serde::Deserialize, Clone, PartialEq)]
pub struct PatientInList {
    pub id: String,
    pub firstName: String,
    pub lastName: String
}

async fn fetch_patient_list(base_url: String) -> Result<Vec<PatientInList>,reqwest::Error> {
    let url = format!("{}/pharmacy/patients", base_url);
    let response = reqwest::get(url).await?;
    let patients: Vec<PatientInList> = response.json().await?;
    Ok(patients)
}

#[non_exhaustive]
#[derive(Clone, Props, PartialEq)]
pub struct PatientListProps {
    base_uri: String,
    onselect: Callback<PatientInList>
}

#[non_exhaustive]
#[derive(Clone, Props, PartialEq)]
struct PatientDisp{
    patient: PatientInList,
    onselect: Callback<PatientInList>
}


pub fn display_patient(props: PatientDisp) -> Element {
    let patient = props.patient;
    let display_name = format!("{}, {}", patient.lastName, patient.firstName);
    let id = patient.id.clone();

    rsx! { li { a { href: "#", onclick: move |_| props.onselect.call(patient.clone()), "{display_name}" } } }
}

pub fn patient_list(config: PatientListProps) -> Element {
    let patients = use_resource(move || {
        let url = config.base_uri.clone();
        async move {
            fetch_patient_list(url).await
        }
    });
    rsx! {
        ul {
            match &*patients.read_unchecked() {
                None => rsx! (p {"No patients"}),
                Some(Ok(patients)) => rsx! {for patient in patients { display_patient {patient: patient.clone(), onselect: config.onselect.clone()} } },
                Some(Err(_)) => rsx! (p {"Error"})
            }
        }
    }
}