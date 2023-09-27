use std::net::TcpStream;
use rand::Rng;

use crate::{algorithms::{random_prime, sqruare_and_multiply_mod}, network::{send_message, DHMessage, read_message}};

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
    let mut rng = rand::thread_rng();
    let private: u64 = rng.gen();
    let public: u64 = rng.gen();

    msg("Finding a prime");
    let prime = random_prime();

    send_message(tcp, DHMessage::ConnectionProposal { public_key: public, modulo: prime })?;

    let foreign_key: u64 = match read_message(tcp)? {
        DHMessage::ConnectionAck { public_key } => public_key,
        DHMessage::ConnectionProposal { .. } => return Err("Proposal is not a valid message for a client".into()),
        DHMessage::Message { data } => todo!("Not ready yet: {}", data.len()),
    };

    println!("My private is: {}", private);
    println!("My public is:  {}", public);
    println!("Foreign is:    {}", foreign_key);
    println!("Modulo is:     {}", prime);

    let key = sqruare_and_multiply_mod(private, public, prime);
    let key = sqruare_and_multiply_mod(key, foreign_key, prime);
    Ok(key)
}

