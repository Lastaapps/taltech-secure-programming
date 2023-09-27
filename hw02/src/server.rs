use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use rand::Rng;

use crate::{
    algorithms::sqruare_and_multiply_mod,
    network::{read_message, send_message, DHMessage},
};

fn msg(name: &str, msg: &str) {
    println!("{}> {}", name, msg)
}

pub fn run_server(host: &str, port: u16) -> Result<(), String> {
    let addr = format!("{}:{}", host, port);
    let listener =
        TcpListener::bind(addr).map_err(|e| format!("Failed to start a server: {}", e))?;

    msg("server", "Waiting for connections...");
    for (counter, stream) in listener.incoming().enumerate() {
        let name = format!("client{}", counter);

        let stream = stream.unwrap();
        thread::spawn(move || {
            msg(&name, "Connection established!");

            match handle_server(stream, &name) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e)
                }
            }

            msg(&name, "Connection closed!");
        });
    }

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

    let (base, modulo, foreign_key) = match read_message(tcp)? {
        DHMessage::ConnectionProposal {
            base,
            modulo,
            public,
        } => (base, modulo, public),
        DHMessage::ConnectionAck { .. } => {
            return Err("Ack is not a valid message for a server".into())
        }
        DHMessage::Message { data } => todo!("Not ready yet: {}", data.len()),
    };

    let private: u64 = rng.gen(); // TODO check gcd(private, prime - 1) == 1
    let public = sqruare_and_multiply_mod(base, private, modulo);

    send_message(tcp, DHMessage::ConnectionAck { public })?;
    let key = sqruare_and_multiply_mod(foreign_key, private, modulo);

    // eprintln!("My private is: {}", private);
    // eprintln!("My public is:  {}", public);
    // eprintln!("Foreign is:    {}", foreign_key);
    // eprintln!("Base is:       {}", base);
    // eprintln!("Modulo is:     {}", modulo);
    // eprintln!("Key is:        {}", key);

    Ok(key)
}
