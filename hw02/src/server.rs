use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use crate::{
    algorithms::{
        exp_cipher_decrypt, exp_cipher_encrypt, inverse_mod, random_undivisible_with,
        sqruare_and_multiply_mod,
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

    let key_encrypt = key_excahnge(&mut tcp)?;
    let key_dectypt = inverse_mod(key_encrypt, u64::MAX)
        .ok_or_else(|| format!("You are out of luck, the key does not meet the requirement to use the cypher, try again"))?;

    msg(
        name,
        &format!("The key is {}, but don't tell anybody", key_encrypt),
    );

    loop {
        // receive
        let data = if let DHMessage::Message { data } = read_message(&mut tcp)? {
            data
        } else {
            return Err("Authorization message received, but comunication expected".into());
        };

        let text = exp_cipher_decrypt(data, key_dectypt)?;
        let text = text.to_lowercase();
        msg(name, &format!("Got: {}", text));

        // send
        let data = exp_cipher_encrypt(&text, key_encrypt)?;
        send_message(&mut tcp, DHMessage::Message { data })?;
    }
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
