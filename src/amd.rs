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
