use base64::{engine::general_purpose, Engine as _};
use std::ops::Rem;

enum CipherConfig {
    Ceasar { shift: i64, line: String },
    Vigenere { line: String, key: String },
}

enum ProcessMode {
    Encode,
    Decode,
}

struct InputConfig {
    mode: ProcessMode,
    config: CipherConfig,
}


// -- Ceasar ------------------------------------------------------------------
fn ceasar(bytes: &mut [u8], shift: i64, encode: bool) {
    let shift = shift.rem(256) as u8;
    let shift = if encode {
        shift
    } else {
        0u8.wrapping_sub(shift)
    };

    for b in bytes.iter_mut() {
        *b = (*b).wrapping_add(shift);
    }
}

fn encode_ceasar(data: &str, shift: i64) -> Result<String, String> {
    let mut bytes: Vec<u8> = data.bytes().collect();
    ceasar(&mut bytes, shift, true);
    Ok(general_purpose::STANDARD_NO_PAD.encode(bytes))
}

fn decode_ceasar(data: &str, shift: i64) -> Result<String, String> {
    let mut bytes = general_purpose::STANDARD_NO_PAD
        .decode(data)
        .map_err(|e| format!("Base decode failed: {}", e))?;

    ceasar(&mut bytes, shift, false);

    String::from_utf8(bytes).map_err(|e| format!("Output are not valid UTF bytes: {}", e))
}


// -- Vigener -----------------------------------------------------------------
fn validate_vigener_input(bytes: &[u8], key: &[u8]) -> Result<(), String> {
    if bytes.len() != key.len() {
        return Err(format!(
            "Input and key length mismatches! i: {} x k: {}",
            bytes.len(),
            key.len()
        ));
    };
    Ok(())
}

fn encode_vigener(data: &str, key: &str) -> Result<String, String> {
    let mut bytes: Vec<u8> = data.bytes().collect();
    let key = general_purpose::STANDARD_NO_PAD
        .decode(key)
        .map_err(|e| format!("Key base64 decode failed: {}", e))?;

    validate_vigener_input(&bytes, &key)?;

    bytes
        .iter_mut()
        .zip(key)
        .for_each(|(x, y)| *x = x.wrapping_add(y));

    Ok(general_purpose::STANDARD_NO_PAD.encode(bytes))
}

fn decode_vigener(data: &str, key: &str) -> Result<String, String> {
    let mut bytes = general_purpose::STANDARD_NO_PAD
        .decode(data)
        .map_err(|e| format!("Base decode failed: {}", e))?;
    let key = general_purpose::STANDARD_NO_PAD
        .decode(key)
        .map_err(|e| format!("Key base64 decode failed: {}", e))?;

    validate_vigener_input(&bytes, &key)?;

    bytes
        .iter_mut()
        .zip(key)
        .for_each(|(x, y)| *x = x.wrapping_sub(y));

    String::from_utf8(bytes).map_err(|e| format!("Output are not valid UTF bytes: {}", e))
}


// --- Input handling ---------------------------------------------------------
fn read_config() -> Result<Option<InputConfig>, String> {
    // Read cipher type
    let is_ceasar = loop {
        eprintln!("Cipher type: (c) Ceasar, (v) Vigenere or (q) quit the program");

        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .map_err(|w| format!("Failed to read type: {}", w))?;
        let buf = buf.trim();

        if buf.len() != 1 {
            return Err(format!(
                "Exactly 1 character expected per line, got '{}'",
                buf
            ));
        }
        let cipher = buf.chars().next().unwrap();
        match cipher {
            'q' | 'Q' => {
                eprintln!("Exiting…");
                return Ok(None);
            }
            'c' | 'C' => break true,
            'v' | 'V' => break false,
            _ => eprintln!("You have problems with letters you stupid donkey I see"),
        };
    };

    // Read mode
    let mode = loop {
        eprintln!("(e) Encode, (d) Decode");

        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .map_err(|w| format!("Failed to read mode: {}", w))?;
        let buf = buf.trim();

        if buf.len() != 1 {
            return Err(format!(
                "Exactly 1 character expected per line, got '{}'",
                buf
            ));
        }
        let mode = buf.chars().next().unwrap();

        match mode {
            'e' | 'E' => break ProcessMode::Encode,
            'd' | 'D' => break ProcessMode::Decode,
            _ => eprintln!("No, stop, behave!!!"),
        };
    };

    // Read final text to process
    fn read_input() -> Result<String, String> {
        eprintln!("Enter the text to process:");
        let mut line = String::new();
        let _bytes_read = std::io::stdin()
            .read_line(&mut line)
            .map_err(|e| format!("Failed to read the input: {}", e))?;
        Ok(line.trim().to_owned())
    }

    // Read extras
    let config = if is_ceasar {
        eprintln!("Enter the shift:");
        let shift: i64 = {
            let mut buf = String::new();
            std::io::stdin()
                .read_line(&mut buf)
                .map_err(|w| format!("Failed to read shift: {}", w))?;

            buf.trim()
                .parse()
                .map_err(|e| format!("Input not an integer: {}", e))?
        };

        CipherConfig::Ceasar {
            shift,
            line: read_input()?,
        }
    } else {
        eprintln!("Enter the key (base64 encoded, same length as the text):");
        let mut key = String::new();
        let _bytes_read = std::io::stdin()
            .read_line(&mut key)
            .map_err(|e| format!("Failed to read the key: {}", e))?;
        let key = key.trim();

        CipherConfig::Vigenere {
            line: read_input()?,
            key: key.to_owned(),
        }
    };

    Ok(Some(InputConfig {
        mode,
        config,
    }))
}

fn handle_request() -> Result<bool, String> {
    let config = if let Some(cfg) = read_config()? {
        cfg
    } else {
        return Ok(false);
    };
    let InputConfig { mode, config } = config;

    let output = match config {
        CipherConfig::Ceasar { shift, line } => match mode {
            ProcessMode::Encode => encode_ceasar(&line, shift),
            ProcessMode::Decode => decode_ceasar(&line, shift),
        },
        CipherConfig::Vigenere { line, key } => match mode {
            ProcessMode::Encode => encode_vigener(&line, &key),
            ProcessMode::Decode => decode_vigener(&line, &key),
        },
    }?;

    println!("{}", output);
    Ok(true)
}

fn main() -> Result<(), String> {
    loop {
        let attemp = handle_request()?;
        if !attemp {
            break;
        };
    }

    eprintln!("Ok, bye!");
    Ok(())
}

// --- Tests ------------------------------------------------------------------
#[test]
fn test_ceasar_encode() {
    assert_eq!(encode_ceasar("bflm", 0).unwrap(), "YmZsbQ");
    assert_eq!(encode_ceasar("bflm", 1).unwrap(), "Y2dtbg");
    assert_eq!(encode_ceasar("bflm", -1).unwrap(), "YWVrbA");

    assert_eq!(encode_ceasar("bflm", 0).unwrap(), "YmZsbQ");
    assert_eq!(encode_ceasar("bflm", 256).unwrap(), "YmZsbQ");
    assert_eq!(encode_ceasar("bflm", -256).unwrap(), "YmZsbQ");

    assert_eq!(encode_ceasar("bflm", 1).unwrap(), "Y2dtbg");
    assert_eq!(encode_ceasar("bflm", 257).unwrap(), "Y2dtbg");
    assert_eq!(encode_ceasar("bflm", -255).unwrap(), "Y2dtbg");

    assert_eq!(encode_ceasar("bflm", -1).unwrap(), "YWVrbA");
    assert_eq!(encode_ceasar("bflm", 255).unwrap(), "YWVrbA");
    assert_eq!(encode_ceasar("bflm", -257).unwrap(), "YWVrbA");
}

#[test]
fn test_ceasar_decode() {
    assert_eq!(decode_ceasar("YmZsbQ", 0).unwrap(), "bflm");
    assert_eq!(decode_ceasar("Y2dtbg", 1).unwrap(), "bflm");
    assert_eq!(decode_ceasar("YWVrbA", -1).unwrap(), "bflm");

    assert_eq!(decode_ceasar("YmZsbQ", 0).unwrap(), "bflm");
    assert_eq!(decode_ceasar("YmZsbQ", 256).unwrap(), "bflm");
    assert_eq!(decode_ceasar("YmZsbQ", -256).unwrap(), "bflm");

    assert_eq!(decode_ceasar("Y2dtbg", 1).unwrap(), "bflm");
    assert_eq!(decode_ceasar("Y2dtbg", 257).unwrap(), "bflm");
    assert_eq!(decode_ceasar("Y2dtbg", -255).unwrap(), "bflm");

    assert_eq!(decode_ceasar("YWVrbA", -1).unwrap(), "bflm");
    assert_eq!(decode_ceasar("YWVrbA", 255).unwrap(), "bflm");
    assert_eq!(decode_ceasar("YWVrbA", -257).unwrap(), "bflm");
}

#[test]
fn test_ceasar_both() {
    let example_text = "asfd12345ěščřžýáíéü@#$%^&*";
    for i in -420..420 {
        assert_eq!(
            decode_ceasar(&encode_ceasar(example_text, i).unwrap(), i).unwrap(),
            example_text
        );
    }
}

#[test]
fn test_vigener_encode() {
    assert_eq!(
        encode_vigener(
            "Never gonna let you down",
            "TmV2ZXIgZ29ubmEgbWFrZSB5b3UgY3J5", // Never gonna make you cry
        )
        .unwrap(),
        "nMrsyuRAzt7c3MJA2cbfhZno5JWE0unn"
    );

    // Different length
    assert_eq!(
        encode_vigener(
            "Never gonna let you down",
            "TmV2ZXIgZ29ubmEgZ2l2ZSB5b3UgdXA", // Never gonna give you up
        ),
        Err("Input and key length mismatches! i: 24 x k: 23".to_string())
    );
}

#[test]
fn test_vigener_decode() {
    assert_eq!(
        decode_vigener(
            "nMrsyuRAzt7c3MJA2cbfhZno5JWE0unn",
            "TmV2ZXIgZ29ubmEgbWFrZSB5b3UgY3J5", // Never gonna make you cry
        )
        .unwrap(),
        "Never gonna let you down"
    );

    // Different length
    assert_eq!(
        decode_vigener(
            "nMrsyuRAzt7c3MJA2cbfhZno5JWE0unn",
            "TmV2ZXIgZ29ubmEgZ2l2ZSB5b3UgdXA", // Never gonna give you up
        ),
        Err("Input and key length mismatches! i: 24 x k: 23".to_string())
    );
}

