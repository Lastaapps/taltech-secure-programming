use std::net::TcpStream;

use crate::{
    algorithms::{random_prime, sqruare_and_multiply_mod, random_undivisible_with},
    network::{read_message, send_message, DHMessage},
};

fn msg(msg: &str) {
    println!("server> {}", msg)
}

pub fn run_client(host: &str, port: u16) -> Result<(), String> {
    msg("Opening a stream");

    let address = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(address)
        .map_err(|e| format!("Failed to open a TCP connection: {}", e))?;
    stream.set_nodelay(true).unwrap();

    let key = key_excahnge(&mut stream)?;
    msg(&format!("The key is {}, but don't tell anybody", key));

    msg("All done, closing");
    Ok(())
}

fn key_excahnge(tcp: &mut TcpStream) -> Result<u64, String> {
    let modulo = random_prime();
    let base: u64 = random_undivisible_with(modulo);

    let private: u64 = random_undivisible_with(modulo - 1);
    let public = sqruare_and_multiply_mod(base, private, modulo);

    send_message(
        tcp,
        DHMessage::ConnectionProposal {
            base,
            modulo,
            public,
        },
    )?;

    let foreign_key = match read_message(tcp)? {
        DHMessage::ConnectionAck { public } => public,
        DHMessage::ConnectionProposal { .. } => {
            return Err("Proposal is not a valid message for a client".into())
        }
        DHMessage::Message { data } => todo!("Not ready yet: {}", data.len()),
    };

    let key = sqruare_and_multiply_mod(foreign_key, private, modulo);

    // eprintln!("My private is: {}", private);
    // eprintln!("My public is:  {}", public);
    // eprintln!("Foreign is:    {}", foreign_key);
    // eprintln!("Base is:       {}", base);
    // eprintln!("Modulo is:     {}", modulo);
    // eprintln!("Key is:        {}", key);

    Ok(key)
}
