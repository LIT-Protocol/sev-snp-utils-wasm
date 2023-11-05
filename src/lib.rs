mod amd;
mod utils;

use base64_light::base64_decode;
use wasm_bindgen::prelude::*;

use sev::certs::snp::{builtin::milan, ca, Chain, Verifiable};
use sev::firmware::guest::AttestationReport;
use web_sys::console;

#[wasm_bindgen]
#[doc = "Parses and returns the parsed attestation report"]
pub fn parse_attestation_report(attestation_report: &str) -> Result<JsValue, JsValue> {
    console::log_1(&"Parsing attestation report...".into());
    utils::set_panic_hook();

    let report_bytes = base64_decode(attestation_report);
    let report: AttestationReport = unsafe { std::ptr::read(report_bytes.as_ptr() as *const _) };
    return Ok(serde_wasm_bindgen::to_value(&report).expect("Could not convert report to JsValue"));
}

#[wasm_bindgen]
#[doc = "Gets the vcek url for the given attestation report.  You can fetch this certificate yourself, and if you put it into LocalStorage with the url as the key and the response body as base64 encoded value, then the next time you call verify_attestation_report it will use the cached value instead of fetching it again."]
pub fn get_vcek_url(attestation_report: &str) -> Result<JsValue, JsValue> {
    // console::log_1(&"Getting vcek url...".into());
    utils::set_panic_hook();

    let report_bytes = base64_decode(attestation_report);
    let report: AttestationReport = unsafe { std::ptr::read(report_bytes.as_ptr() as *const _) };

    let url = amd::get_kds_vcek_der_url(
        "Milan",
        &utils::fmt_bin_vec_to_hex(&report.chip_id.to_vec()),
        report.reported_tcb.bootloader,
        report.reported_tcb.tee,
        report.reported_tcb.snp,
        report.reported_tcb.microcode,
    );

    return Ok(JsValue::from_str(&url));
}

#[wasm_bindgen]
pub async fn verify_attestation_report(attestation_report: &str) -> Result<(), JsValue> {
    // console::log_1(&"Checking attestation report...".into());
    utils::set_panic_hook();

    let report_bytes = base64_decode(attestation_report);
    let report: AttestationReport = unsafe { std::ptr::read(report_bytes.as_ptr() as *const _) };

    // console::log_1(&"Got report: ".into());
    // console::log_1(&report.to_string().into());

    let ark = milan::ark().unwrap();
    let ask = milan::ask().unwrap();
    let vcek = amd::fetch_kds_vcek_der(
        "Milan",
        &utils::fmt_bin_vec_to_hex(&report.chip_id.to_vec()),
        report.reported_tcb.bootloader,
        report.reported_tcb.tee,
        report.reported_tcb.snp,
        report.reported_tcb.microcode,
    )
    .await
    .expect("Could not get vcek");

    let ca = ca::Chain { ark, ask };

    let chain = Chain { ca, vcek };

    let verified = (&chain, &report).verify();
    if let Err(e) = verified {
        console::log_1(&"Verification failed".into());
        console::log_1(&e.to_string().into());
        return Err(e.to_string().into());
    }
    Ok(())
}
