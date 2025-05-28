use dioxus::core_macro::Props;
use dioxus::prelude::*;
use reqwest::{Client, StatusCode};
use serde::Serialize;
use crate::patient_list;
use uuid;

#[derive(serde::Serialize, Clone, PartialEq)]
struct PaperDispatch {
    #[serde(rename = "prescriptionGroup")]
    prescription_group: String,
    #[serde(rename = "registrationType")]
    registration_type: String,
    name: String,
    #[serde(rename = "nameFormStrength")]
    name_form_strength: String,
    #[serde(rename = "packingSize")]
    packing_size: String,
    #[serde(rename = "packingUnitCode")]
    packing_unit_code: String,
    #[serde(rename = "packingUnitDisplay")]
    packing_unit_display: String,
    #[serde(rename = "productNumber")]
    product_number: String,
    #[serde(rename = "atcCode")]
    atc_code: String,
    #[serde(rename = "atcDisplay")]
    atc_display: String,
    #[serde(rename = "formCode")]
    form_code: String,
    #[serde(rename = "formDisplay")]
    form_display: String,
    amount: Option<f64>,
    #[serde(rename = "amountUnit")]
    amount_unit: String,
    #[serde(rename = "amountText")]
    amount_text: String,
    /* */
    dssn: String,
    #[serde(rename = "numberOfPackages")]
    number_of_packages: Option<f64>,
    reit: String,
    #[serde(rename = "itemGroupCode")]
    item_group_code: String,
    #[serde(rename = "itemGroupDisplay")]
    item_group_display: String,
    #[serde(rename = "prescriptionTypeCode")]
    prescription_type_code: String,
    #[serde(rename = "prescriptionTypeDisplay")]
    prescription_type_display: String,
    #[serde(rename = "prescriptionId")]
    prescription_id: String,
    #[serde(rename = "genericSubstitutionAccepted")]
    generic_substitution_accepted: bool,
    /* */
    #[serde(rename = "prescribedByHpr")]
    prescribed_by_hpr: String,
    #[serde(rename = "prescribedByGivenName")]
    prescribed_by_given_name: String,
    #[serde(rename = "prescribedByFamilyName")]
    prescribed_by_family_name: String,
    /* */
    #[serde(rename = "dispatcherHerId")]
    dispatcher_her_id: String,
    #[serde(rename = "dispatcherName")]
    dispatcher_name: String,
    /* */ 
    #[serde(rename = "substitutionReservationCustomer")]
    substitution_reservation_customer: bool,
    #[serde(rename = "dispatchMsgId")]
    dispatch_msg_id: String,
    quantity: f64,
    #[serde(rename = "whenHandedOver")]
    when_handed_over: String
}

#[non_exhaustive]
#[derive(Clone, Props, PartialEq)]
pub struct PatientProps {
    pub patient: patient_list::PatientInList,
    pub base_uri: String
}

async fn submit_dispatch(base_url: String, patient_id: String) -> Result<StatusCode,reqwest::Error> {
    let url = format!("{}/pharmacy/patients/{}/paperdispatch", base_url, patient_id);
    let timestamp = js_sys::Date::now() as i64;
    let prescription_id = format!("{:04}_{:06}",
                                  timestamp % 10000,
                                  timestamp % 1000000
    );
    
    let paper_dispatch =
        PaperDispatch {
            prescription_group: "C".to_string(),
            registration_type: "3".to_string(),
            name: "Furix".to_string(),
            name_form_strength: "Furix tab 40 mg".to_string(),
            packing_size: "100".to_string(),
            packing_unit_code: "STK".to_string(),
            packing_unit_display: "STK".to_string(),
            product_number: "122754".to_string(),
            atc_code: "C03CA01".to_string(),
            atc_display: "furosemide".to_string(),
            form_code: "53".to_string(),
            form_display: "tablett".to_string(),
            amount: Some(100.0),
            amount_unit: "STK".to_string(),
            amount_text: "100 STK".to_string(),
            dssn: "MOT HØYT BLODTRYKK\n1 tablett morgen\n1 tablett ettermiddag\nSvelges hele".to_string(),
            number_of_packages: Some(1.0),
            reit: "0".to_string(),
            item_group_code: "L".to_string(),
            item_group_display: "Legemiddel".to_string(),
            prescription_type_code: "P".to_string(),
            prescription_type_display: "Papirresept".to_string(),
            prescription_id: prescription_id,
            generic_substitution_accepted: true,
            prescribed_by_hpr: "222200063".to_string(),
            prescribed_by_given_name: "Rolf Fos".to_string(),
            prescribed_by_family_name: "Lillehagen".to_string(),
            dispatcher_her_id: "8090732".to_string(),
            dispatcher_name: "Apoteket Vågen".to_string(),
            substitution_reservation_customer: false,
            dispatch_msg_id: uuid::Uuid::from_u64_pair(timestamp as u64, timestamp as u64).to_string(),
            quantity: 1.0,
            when_handed_over: chrono::Local::now().format("%Y-%m-%d").to_string()
        };
    let response = Client::builder().build().expect("failed to build client")
        .put(url)
        .json(&paper_dispatch)
        .send().await;
    Ok(response?.status())
}

pub fn patient(props: PatientProps) -> Element {
    let patient = props.patient;
    let mut result = use_signal(|| { let result :Option<Result<StatusCode, reqwest::Error>> = None; result});
    rsx! {
        h2 { "{patient.lastName}, {patient.firstName}"}
        button { onclick: move |e| {
            let base_uri = props.base_uri.clone();
            let patient_id = patient.id.clone();
            async move {
                let res = submit_dispatch(base_uri, patient_id).await;
                result.set(Some(res));
            }
        }, "Create paper dispatch" }
        p { "Result:" }
        p { match &*result.read_unchecked() {
            None => "No results",
            Some(result) => match result {
                Response => "Responded",
                Error => "Error"
            }
        }}
    }
}