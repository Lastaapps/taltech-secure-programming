use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;

fn hmac(key: &[u8], msg: &[u8], target: &[u8]) -> Option<Vec<u8>> {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(msg);

    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    if code_bytes[..] == target[..] {
        return Some(code_bytes.to_vec());
    };

    None
}

#[allow(unreachable_code)]
fn main() {
    let msg = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJKV1QgQ2hhbGxlbmdlIiwicm9sZSI6InVzZXIiLCJpYXQiOjE2OTc4NDAxMjV9";
    let target = "2HKZrHxFb5JuWQ5bb47NZ8ThPZad-eJNUOPxO3G6ALA";

    let msg = msg.as_bytes();
    let target = general_purpose::URL_SAFE_NO_PAD.decode(target).unwrap();
    let target = target.as_slice();

    let chars =
        b"abcdefghijjklmnopqrstuvwxyzABCDEFGHIJJKLMNOPQRSTUVWXYZ1234567890*/.,!@#$%^&*({}:;\\)";

    // crack the human readable variants
    for c1 in chars.iter() {
        println!("{}", String::from_utf8(vec![*c1]).unwrap());
        for c2 in chars.iter() {
            for c3 in chars.iter() {
                for c4 in chars.iter() {
                    let bytes: [u8; 4] = [*c1, *c2, *c3, *c4];
                    let bytes = &bytes;

                    if let Some(_) = hmac(bytes, msg, target) {
                        let encoded = general_purpose::URL_SAFE_NO_PAD.encode(&bytes);
                        println!("Found result ({}): {}", String::from_utf8(bytes.to_vec()).unwrap(), encoded);
                        return;
                    }
                }
            }
        }
    }

    // crack all the variants
    for i in 0..=(1u32 << 32 - 1) {
        let bytes = i.to_be_bytes();
        let bytes = &bytes;

        if i % 100_000 == 0 {
            println!("{}", i);
        }

        if let Some(_) = hmac(bytes, msg, target) {
            let encoded = general_purpose::URL_SAFE_NO_PAD.encode(&bytes);
            println!("Found result (dec: {}): {}", i, encoded);
            return;
        }
    }

    eprintln!("Mission failed!");
    return;
}
