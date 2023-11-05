use crate::utils::fmt_bin_vec_to_hex;
use js_sys::ArrayBuffer;
use sev::certs::snp::Certificate;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::{Blob, Request, RequestInit, RequestMode, Response};

const KDS_CERT_SITE: &str = "https://kdsintf.amd.com";
#[allow(dead_code)]
const KDS_DEV_CERT_SITE: &str = "https://kdsintfdev.amd.com";
#[allow(dead_code)]
const KDS_CEK: &str = "/cek/id/";
const KDS_VCEK: &str = "/vcek/v1/"; // KDS_VCEK/{product_name}/{hwid}?{tcb parameter list}
#[allow(dead_code)]
const KDS_VCEK_CERT_CHAIN: &str = "cert_chain"; // KDS_VCEK/{product_name}/cert_chain
#[allow(dead_code)]
const KDS_VCEK_CRL: &str = "crl"; // KDS_VCEK/{product_name}/crl"

const ASK_DER_FILENAME: &str = "ask.der";
const ASK_PEM_FILENAME: &str = "ask.pem";
const ARK_DER_FILENAME: &str = "ark.der";
const ARK_PEM_FILENAME: &str = "ark.pem";

const ARK_FETCH_LOCK_FILE: &str = "ark_fetch.lock";

pub fn get_kds_vcek_cert_chain_url(product_name: &str) -> String {
    format!(
        "{}{}{}/{}",
        KDS_CERT_SITE, KDS_VCEK, product_name, KDS_VCEK_CERT_CHAIN
    )
}

// pub async fn fetch_kds_vcek_cert_chain_pem(product_name: &str) -> Result<Certificate, JsValue> {
//     let url = get_kds_vcek_cert_chain_url(product_name);

//     let cert_blob = fetch_blob(url)
//         .await
//         .expect("Could not get cert chain from KDS");

//     Certificate::from_pem(&blob_into_bytes(cert_blob).await).map_err(|e| e.to_string().into())
// }

pub fn get_kds_vcek_der_url(
    product_name: &str,
    chip_id: &str,
    boot_loader: u8,
    tee: u8,
    snp: u8,
    microcode: u8,
) -> String {
    format!(
        "{}{}{}/{}?blSPL={:0>2}&teeSPL={:0>2}&snpSPL={:0>2}&ucodeSPL={:0>2}",
        KDS_CERT_SITE, KDS_VCEK, product_name, chip_id, boot_loader, tee, snp, microcode
    )
}

pub async fn fetch_kds_vcek_der(
    product_name: &str,
    chip_id: &str,
    boot_loader: u8,
    tee: u8,
    snp: u8,
    microcode: u8,
) -> Result<Certificate, JsValue> {
    let url = get_kds_vcek_der_url(product_name, chip_id, boot_loader, tee, snp, microcode);

    let cert_array_buffer = fetch_blob(url)
        .await
        .expect("Could not get cert chain from KDS");

    let cert_vec = js_sys::Uint8Array::new(&cert_array_buffer).to_vec();
    Certificate::from_der(&cert_vec).map_err(|e| e.to_string().into())
}

pub async fn fetch_blob(url: String) -> Result<ArrayBuffer, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let cors_proxy = "https://shielded-earth-13917-10ba15245911.herokuapp.com/";
    let url = format!("{}{}", cors_proxy, url);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    // request
    //     .headers()
    //     .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    // assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let resolved_array_buffer = JsFuture::from(resp.array_buffer()?).await?;
    let resolved_array_buffer: ArrayBuffer = resolved_array_buffer.dyn_into().unwrap();
    console::log_1(&resolved_array_buffer.byte_length().into());
    // let blob = Blob::new_with_blob_sequence(&resolved_blob).expect("Could not convert to blob");

    // Send the JSON response back to JS.
    Ok(resolved_array_buffer)
}

async fn blob_into_bytes(blob: Blob) -> Vec<u8> {
    let array_buffer_promise: JsFuture = blob.array_buffer().into();

    let array_buffer: JsValue = array_buffer_promise
        .await
        .expect("Could not get ArrayBuffer from file");

    js_sys::Uint8Array::new(&array_buffer).to_vec()
}
