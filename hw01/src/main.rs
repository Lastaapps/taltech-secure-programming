use std::ops::Rem;
use base64::{Engine as _, engine::general_purpose};

fn ceasar(bytes: &mut [u8], by: u8) {
    for b in bytes.iter_mut() {
        *b = (*b).wrapping_add(by);
    };
}

fn encode(data: &str, shift: i64) -> Result<String, String> {
    let shift = shift.rem(256) as u8;
    let mut bytes: Vec<u8> = data.bytes().collect();
    ceasar(&mut bytes, shift);
    Ok(general_purpose::STANDARD_NO_PAD.encode(bytes))
}

fn decode(data: &str, shift: i64) -> Result<String, String> {
    let shift = shift.rem(256) as u8;
    let mut bytes = general_purpose::STANDARD_NO_PAD.decode(data)
        .map_err(|e| format!("Base decode failed: {}", e))?;

    let shift = 0u8.wrapping_sub(shift);

    ceasar(&mut bytes, shift);

    String::from_utf8(bytes)
        .map_err(|e| format!("Output are not valid UTF bytes: {}", e))
}

fn handle_user() -> Result<bool, String> {
    // todo!("Ceasar / Vigenere")

    eprintln!("(e) Encode, (d) Decode, (q) Quit");

    let mode = {
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .map_err(|w| format!("Failed to read mode: {}", w))?;
        let buf = buf.trim();

        if buf.len() != 1 {
            return Err(format!("Exactly 1 character expected per line, got '{}'", buf));
        }
        buf.chars().nth(0).unwrap()
    };

    match mode {
        'q' | 'Q' =>  {
            eprintln!("Exiting…");
            return Ok(false);
        },
        'e' | 'E' | 'd' | 'D' => {},
        _ => {
            eprintln!("Wrong mode, try again");
            return Ok(true);
        },
    };

    eprintln!("Enter the shift:");
    let shift: i64 = {
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .map_err(|w| format!("Failed to read shift: {}", w))?;

        buf.trim().parse()
            .map_err(|e| format!("Input not an integer: {}", e))?
    };

    eprintln!("Enter the text to process:");
    let mut line = String::new();
    let _bytes_read = std::io::stdin().read_line(&mut line).unwrap();
    let line = line.trim();

    let output = if mode == 'e' || mode == 'E' {
        encode(line, shift)
    } else {
        decode(line, shift)
    }?;

    println!("{}", output);
    return Ok(true);
}

fn main() -> Result<(), String> {
    loop {
        let attemp = handle_user()?;
        if !attemp {
            break;
        };
    };

    eprintln!("Ok, bye!");
    Ok(())
}

// --- Tests ------------------------------------------------------------------
#[test]
fn test_encode() {
    assert_eq!(encode("bflm",    0).unwrap(), "YmZsbQ");
    assert_eq!(encode("bflm",    1).unwrap(), "Y2dtbg");
    assert_eq!(encode("bflm",   -1).unwrap(), "YWVrbA");

    assert_eq!(encode("bflm",    0).unwrap(), "YmZsbQ");
    assert_eq!(encode("bflm",  256).unwrap(), "YmZsbQ");
    assert_eq!(encode("bflm", -256).unwrap(), "YmZsbQ");

    assert_eq!(encode("bflm",    1).unwrap(), "Y2dtbg");
    assert_eq!(encode("bflm",  257).unwrap(), "Y2dtbg");
    assert_eq!(encode("bflm", -255).unwrap(), "Y2dtbg");

    assert_eq!(encode("bflm",   -1).unwrap(), "YWVrbA");
    assert_eq!(encode("bflm",  255).unwrap(), "YWVrbA");
    assert_eq!(encode("bflm", -257).unwrap(), "YWVrbA");
}

#[test]
fn test_decode() {
    assert_eq!(decode("YmZsbQ",    0).unwrap(), "bflm");
    assert_eq!(decode("Y2dtbg",    1).unwrap(), "bflm");
    assert_eq!(decode("YWVrbA",   -1).unwrap(), "bflm");

    assert_eq!(decode("YmZsbQ",    0).unwrap(), "bflm");
    assert_eq!(decode("YmZsbQ",  256).unwrap(), "bflm");
    assert_eq!(decode("YmZsbQ", -256).unwrap(), "bflm");
    
    assert_eq!(decode("Y2dtbg",    1).unwrap(), "bflm");
    assert_eq!(decode("Y2dtbg",  257).unwrap(), "bflm");
    assert_eq!(decode("Y2dtbg", -255).unwrap(), "bflm");

    assert_eq!(decode("YWVrbA",   -1).unwrap(), "bflm");
    assert_eq!(decode("YWVrbA",  255).unwrap(), "bflm");
    assert_eq!(decode("YWVrbA", -257).unwrap(), "bflm");
}

#[test]
fn test_both() {
    let example_text = "asfd12345ěščřžýáíéü@#$%^&*";
    for i in -420..420 {
        assert_eq!(decode(&encode(example_text, i).unwrap(), i).unwrap(), example_text);
    }
}

