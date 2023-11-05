use base64_light::{base64_decode, base64_encode_bytes};
use js_sys::ArrayBuffer;
use sev::certs::snp::Certificate;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

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

    // check the cache
    let window = web_sys::window().expect("Could not get web_sys::window");
    let local_storage = window
        .local_storage()?
        .expect("Could not get local storage");
    if let Ok(Some(value)) = local_storage.get_item(&url) {
        let decoded = base64_decode(&value);
        return Certificate::from_der(&decoded).map_err(|e| e.to_string().into());
    }

    let cert_array_buffer = fetch_array_buffer(url.clone())
        .await
        .expect("Could not get cert chain from KDS");

    let cert_vec = js_sys::Uint8Array::new(&cert_array_buffer).to_vec();

    // store in cache
    local_storage.set_item(&url, &base64_encode_bytes(&cert_vec))?;

    Certificate::from_der(&cert_vec).map_err(|e| e.to_string().into())
}

pub async fn fetch_array_buffer(url: String) -> Result<ArrayBuffer, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let cors_proxy = "https://shielded-earth-13917-10ba15245911.herokuapp.com/";
    let url = format!("{}{}", cors_proxy, url);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    // assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    if !resp.ok() {
        return Err(JsValue::from_str(&format!(
            "fetch_array_buffer failed to get the vcek from AMD servers with status code: {}",
            resp.status()
        )));
    }

    // Convert this other `Promise` into a rust `Future`.
    let resolved_js_value = JsFuture::from(resp.array_buffer()?).await?;
    let resolved_array_buffer: ArrayBuffer = resolved_js_value.dyn_into().unwrap();

    Ok(resolved_array_buffer)
}
