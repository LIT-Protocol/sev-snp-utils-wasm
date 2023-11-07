//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

mod constants;

extern crate wasm_bindgen_test;

use constants::{
    BAD_SIG_REPORT, CORRUPT_REPORT, VALID_ATTESTATION_JSON_CHALLENGE_SENT,
    VALID_ATTESTATION_JSON_RESPONSE, VALID_REPORT, VALID_REPORT_VCEK_URL,
};
use sev_snp_utils_wasm::{
    get_vcek_url, parse_attestation_report, verify_attestation_report,
    verify_attestation_report_and_check_challenge,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_verify_real_attestation_report_and_check_challenge() {
    // parse json response
    let parsed_response_jsval = js_sys::JSON::parse(VALID_ATTESTATION_JSON_RESPONSE).unwrap();
    let parsed_response = js_sys::Object::from(parsed_response_jsval);

    let report =
        js_sys::Reflect::get(&parsed_response, &"report".into()).expect("Could not get report");
    let data = js_sys::Reflect::get(&parsed_response, &"data".into()).expect("Could not get data");
    let signatures = js_sys::Reflect::get(&parsed_response, &"signatures".into())
        .expect("Could not get signatures");

    let result = verify_attestation_report_and_check_challenge(
        &report.as_string().expect("Could not get report"),
        data,
        signatures,
        VALID_ATTESTATION_JSON_CHALLENGE_SENT,
    )
    .await;
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
async fn test_verify_real_attestation_report() {
    let result = verify_attestation_report(VALID_REPORT).await;
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
async fn test_verify_corrupt_attestation_report() {
    let result = verify_attestation_report(CORRUPT_REPORT).await;
    assert!(result.is_err());
}

#[wasm_bindgen_test]
async fn test_verify_bad_sig_attestation_report() {
    let result = verify_attestation_report(BAD_SIG_REPORT).await;
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_parsing_attestation_report() {
    let result = parse_attestation_report(VALID_REPORT);
    assert!(result.is_ok());

    let result = result.unwrap();
    assert!(result.is_object());

    let result = js_sys::Object::from(result);

    let version = js_sys::Reflect::get(&result, &"version".into());
    assert!(version.is_ok());
    let version = version.unwrap();
    assert!(version == 2);
}

#[wasm_bindgen_test]
fn test_get_vcek_url() {
    let result = get_vcek_url(VALID_REPORT);
    assert!(result.is_ok());

    let result = result.unwrap();
    assert!(result.is_string());

    let result = js_sys::JsString::from(result);

    assert!(result == VALID_REPORT_VCEK_URL);
}
