mod amd;
mod utils;

use wasm_bindgen::prelude::*;

use sev::certs::snp::{builtin::milan, ca, Certificate, Chain, Verifiable};
use sev::firmware::guest::AttestationReport;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub(crate) fn fmt_bin_vec_to_hex(vec: &Vec<u8>) -> String {
    vec.iter().map(|b| format!("{:02x}", b)).collect::<String>()
}

#[wasm_bindgen]
pub async fn verify_attestation_report(attestation_report: JsValue) -> Result<(), JsValue> {
    console::log_1(&"Checking attestation report...".into());
    console_error_panic_hook::set_once();

    let uint8_report = js_sys::Uint8Array::new(&attestation_report);
    let report_bytes = uint8_report.to_vec();

    let report: AttestationReport = unsafe { std::ptr::read(report_bytes.as_ptr() as *const _) };

    // let uint8arrayReport = js_sys::Uint8Array::new(&attestation_report);
    // console::log_1(&"Bit 0:".into());
    // console::log_1(&uint8arrayReport.at(0).into());
    // let report: AttestationReport = serde_wasm_bindgen::from_value(attestation_report)
    //     .expect("Could not parse attestation report");

    console::log_1(&"Got report".into());

    let ark = milan::ark().unwrap();
    let ask = milan::ask().unwrap();
    // let vcek = Certificate::from_der(TEST_MILAN_VCEK_DER).unwrap();
    let vcek = amd::fetch_kds_vcek_der(
        "Milan",
        &fmt_bin_vec_to_hex(&report.chip_id.to_vec()),
        report.reported_tcb.bootloader,
        report.reported_tcb.tee,
        report.reported_tcb.snp,
        report.reported_tcb.microcode,
    )
    .await
    .expect("Could not get vcek");

    let ca = ca::Chain { ark, ask };

    let chain = Chain { ca, vcek };

    let _ = (&chain, &report).verify();
    alert("Hello, sev-snp-utils-wasm!");
    Ok(())
}
