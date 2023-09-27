use std::{
    io::{Read, Write},
    net::TcpStream,
};
use base64::{Engine as _, engine::general_purpose};

pub enum DHMessage {
    ConnectionProposal { base: u64, modulo: u64, public: u64 },
    ConnectionAck { public: u64 },
    Message { data: Vec<u8> },
}

pub fn send_message(tcp: &mut TcpStream, msg: DHMessage) -> Result<(), String> {
    match msg {
        DHMessage::ConnectionProposal { base, modulo, public } => {
            tcp.write_all(format!("DHSYN {} {} {}\n", base, modulo, public).as_bytes())
        }
        DHMessage::ConnectionAck { public } => {
            tcp.write_all(format!("DHACK {}\n", public).as_bytes())
        }
        DHMessage::Message { data } => {
            let msg = general_purpose::STANDARD_NO_PAD.encode(data);
            tcp.write_all(format!("DHMSG {}\n", msg).as_bytes())
        },
    }
    .map_err(|e| format!("Failed to send bytes: {}", e))
}

pub fn read_message(tcp: &mut TcpStream) -> Result<DHMessage, String> {
    let msg = read_till_space(tcp, 6)?;

    let msg = match msg.as_str() {
        "DHSYN" => {
            let base = read_number(tcp)?;
            let modulo = read_number(tcp)?;
            let public = read_number(tcp)?;

            DHMessage::ConnectionProposal { base, modulo, public }
        }
        "DHACK" => {
            let public = read_number(tcp)?;
            DHMessage::ConnectionAck { public }
        }
        "DHMSG" => {
            let encoded = read_till_space(tcp, 2usize.pow(24))?;
            let bytes = general_purpose::STANDARD_NO_PAD.decode(encoded)
                .map_err(|e| format!("Failed to decode base64 encrypted message: {}", e))?;
            DHMessage::Message { data: bytes }
        }
        _ => return Err(format!("Unsupported message: {}", msg)),
    };

    Ok(msg)
}

fn read_till_space(tcp: &mut TcpStream, limit: usize) -> Result<String, String> {
    let mut buf = Vec::with_capacity(limit);

    let mut space_found = false;
    for _ in 0..limit {
        // yeah, this sucks
        let mut b = [0; 1];

        tcp.read_exact(&mut b)
            .map_err(|e| format!("Read error: {}", e))?;

        match b[0] {
            b' ' | b'\n' => {
                space_found = true;
                break;
            }
            _ => buf.push(b[0]),
        }
    }
    if !space_found {
        return Err("To long protocol message".into());
    };

    let msg = String::from_utf8(buf).map_err(|e| format!("Non UTF-8 data transmited: {}", e))?;

    Ok(msg)
}

fn read_number(tcp: &mut TcpStream) -> Result<u64, String> {
    let msg = read_till_space(tcp, 24)?;
    let num: u64 = msg
        .parse()
        .map_err(|e| format!("Failed to parse a number: {}", e))?;
    Ok(num)
}
