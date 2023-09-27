// use crate::constants::BTimeout;
// use crate::errors::BError;
// use crate::messages::{ClientMessage, ServerMessage};
// use crate::state_machine::BState;

// use std::cmp::min;
// use std::{net::TcpStream, io::{Read, Write}};

// use crate::state_machine;

use std::{net::{TcpListener, TcpStream}, thread};

use rand::Rng;

use crate::{network::{DHMessage, send_message, read_message}, algorithms::sqruare_and_multiply_mod};

fn msg(name: &str, msg: &str) {
    println!("{}> {}", name, msg)
}

pub fn run_server(host: &str, port: u16) -> Result<(), String>{

    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(addr)
        .map_err(|e| format!("Failed to start a server: {}", e))?;

    let mut counter = 0;

    msg("server", "Waiting for connections...");
    for stream in listener.incoming() {
        let name = format!("client{}", counter);
        counter += 1;

        let stream = stream.unwrap();
        thread::spawn(move || {
            msg(&name, "Connection established!");

            match handle_server(stream, &name) {
                Ok(_) => {},
                Err(e) => {eprintln!("{}", e)},
            }

            msg(&name, "Connection closed!");
        });
    };

    todo!()
}

fn handle_server(mut stream: TcpStream, name: &str) -> Result<(), String> {
    stream.set_nodelay(true).unwrap();

    msg(name, "Waiting for shared key");

    let key = key_excahnge(&mut stream)?;
    msg(name, &format!("The key is {}, but don't tell anybody", key));

    Ok(())
}

fn key_excahnge(tcp: &mut TcpStream) -> Result<u64, String> {
    let mut rng = rand::thread_rng();
    let private: u64 = rng.gen();
    let public: u64 = rng.gen();

    send_message(tcp, DHMessage::ConnectionAck { public_key: public })?;

    let (foreign_key, prime) = match read_message(tcp)? {
        DHMessage::ConnectionProposal { public_key, modulo } => (public_key, modulo),
        DHMessage::ConnectionAck { .. } => return Err("Ack is not a valid message for a server".into()),
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

