use base64_light::base64_decode;
use hex::FromHex;
use sha2::{Digest, Sha512};
use wasm_bindgen::JsValue;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub(crate) fn fmt_bin_vec_to_hex(vec: &Vec<u8>) -> String {
    vec.iter().map(|b| format!("{:02x}", b)).collect::<String>()
}

pub fn hash_report_data(data: JsValue, signatures: JsValue, challenge: &str) -> [u8; 64] {
    let noonce = <[u8; 32]>::from_hex(challenge).expect("Could not convert challenge to bytes");

    let mut hasher = Sha512::new();

    hasher.update("noonce");
    hasher.update(noonce);

    let data_obj = js_sys::Object::from(data);
    hasher.update("data");
    for entry in js_sys::Object::entries(&data_obj) {
        let key_val_array = js_sys::Array::from(&entry);
        let key = key_val_array
            .get(0)
            .as_string()
            .expect("Could not convert key to string in data map");
        let val = key_val_array
            .get(1)
            .as_string()
            .expect("Could not convert value to string in data map");
        hasher.update(key);
        hasher.update(base64_decode(&val));
    }

    let sig_array = js_sys::Array::from(&signatures);
    let sig_len = sig_array.length();
    if sig_len > 0 {
        let last_sig_idx = (sig_len - 1) as isize;

        hasher.update("signatures");

        for idx in 0..(last_sig_idx as u32) {
            let sig = sig_array.get(idx);
            hasher.update(base64_decode(&sig.as_string().unwrap()));
        }
    }

    let result = hasher.finalize();
    let mut array = [0u8; 64];
    array.copy_from_slice(&result[..]);
    array
}

/* attestation object looks like this:
 "attestation": {
    "type": "AMD_SEV_SNP",
    "noonce": "RPFFYVWtSV37r9/VExEvma5xAjmPazJ4+AG51lT3cD0=",
    "data": {
        "INSTANCE_ID": "YzJjNmI3NjE=",
        "RELEASE_ID": "ZmM1YzkyNTBjY2MxNTllNGEwM2QzOGZiNGRmMDdhNTM1OGE0NGEyN2NjNDkxYjBk",
        "UNIX_TIME": "gqNFZQAAAAA="
    },
    "signatures": [
        "MEQCIH4A2AhIi6GgedbNnmXVQFn+qx1tBppcsrEhmv4fK2vTAiAWhfHnJHPepkSoKzoxMc9Sc3wNtKyzEt1IJXdfqd0RgQEEouNBbEJ/Y5ZQNxtsJ1EfM+xOKzCnc1dSxSMXdCVTun8KDChld60axa7i6kCkUjDG7XrIRzaqjO3pHwbKOYSatQ=="
    ],
    "report": "AgAAAAAAAAAAAAMAAAAAAAEAAAAFEAABCwyQBQajBzX8XJJQzMFZ5KA9OPtN8HpTAAAAAAEAAAADAAAAAAAKqQEAAAAAAAAAAQAAAAAAAAD="
}
*/

/*
this is the function at creates the hash.  the hash_report_data function above must match exactly
fn update_hash(&self, hasher: &mut impl Digest, last_sig_idx: Option<isize>) {
        hasher.update("noonce");
        hasher.update(self.noonce.as_slice());

        hasher.update("data");
        for (key, val) in self.data.iter() {
            hasher.update(key);
            hasher.update(val.as_slice());
        }

        let sig_len = self.signatures.len();
        if sig_len > 0 {
            let max_last_sig_idx = (sig_len - 1) as isize;
            let mut last_sig_idx = match last_sig_idx {
                Some(last_sig_idx) => last_sig_idx,
                None => max_last_sig_idx,
            };
            if last_sig_idx > max_last_sig_idx {
                last_sig_idx = max_last_sig_idx;
            }

            if last_sig_idx >= 0 {
                hasher.update("signatures");

                for idx in 0..(last_sig_idx as usize) {
                    if let Some(sig) = self.signatures.get(idx) {
                        hasher.update(sig.as_slice());
                    }
                }
            }
        }
    } */
