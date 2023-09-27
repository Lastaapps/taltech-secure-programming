use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use seeded_random::{Random, Seed};

use crate::{
    algorithms::{
        prng_cipher_decrypt, prng_cipher_encrypt, random_undivisible_with, sqruare_and_multiply_mod,
    },
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

fn handle_server(mut tcp: TcpStream, name: &str) -> Result<(), String> {
    tcp.set_nodelay(true).unwrap();

    msg(name, "Waiting for shared key");

    let key = key_excahnge(&mut tcp)?;
    let prng = Random::from_seed(Seed::unsafe_new(key));

    msg(name, &format!("The key is {}, but don't tell anybody", key));

    loop {
        // receive
        let data = if let DHMessage::Message { data } = read_message(&mut tcp)? {
            data
        } else {
            return Err("Authorization message received, but comunication expected".into());
        };

        let text = prng_cipher_decrypt(data, &prng)?;

        let text = process_string(&text);

        msg(name, &format!("Got: {}", text));

        // send
        let data = prng_cipher_encrypt(&text, &prng)?;
        send_message(&mut tcp, DHMessage::Message { data })?;
    }
}

fn process_string(text : &str) -> String {
    text.chars().rev().collect()
}

fn key_excahnge(tcp: &mut TcpStream) -> Result<u64, String> {
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

    let private: u64 = random_undivisible_with(modulo - 1);
    let public = sqruare_and_multiply_mod(base, private, modulo);

    send_message(tcp, DHMessage::ConnectionAck { public })?;
    let key = sqruare_and_multiply_mod(foreign_key, private, modulo);

    // eprintln!("My private:  {}", private);
    // eprintln!("My public:   {}", public);
    // eprintln!("Foreign:     {}", foreign_key);
    // eprintln!("Base:        {}", base);
    // eprintln!("Modulo:      {}", modulo);
    // eprintln!("Encrypt key: {}", key);

    Ok(key)
}
