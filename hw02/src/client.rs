use std::{io::stdin, net::TcpStream};

use seeded_random::{Random, Seed};

use crate::{
    algorithms::{
        prng_cipher_decrypt, prng_cipher_encrypt, random_prime, random_undivisible_with,
        sqruare_and_multiply_mod,
    },
    network::{read_message, send_message, DHMessage},
};

fn msg(msg: &str) {
    println!("server> {}", msg)
}

pub fn run_client(host: &str, port: u16) -> Result<(), String> {
    msg("Opening a stream");

    let address = format!("{}:{}", host, port);
    let mut tcp = TcpStream::connect(address)
        .map_err(|e| format!("Failed to open a TCP connection: {}", e))?;
    tcp.set_nodelay(true)
        .map_err(|e| format!("Cannot set nodelay mode: {}", e))?;

    let key = key_excahnge(&mut tcp)?;
    let prng = Random::from_seed(Seed::unsafe_new(key));

    msg(&format!("The key is {}, but don't tell anybody", key));

    loop {
        // send
        let mut line = String::new();
        stdin()
            .read_line(&mut line)
            .map_err(|e| format!("Failed to read a line from stdin: {}", e))?;

        let data = prng_cipher_encrypt(line.trim(), &prng)?;
        send_message(&mut tcp, DHMessage::Message { data })?;
        msg("Data sent");

        // receive
        let data = if let DHMessage::Message { data } = read_message(&mut tcp)? {
            data
        } else {
            return Err("Authorization message received, but comunication expected".into());
        };

        let text = prng_cipher_decrypt(data, &prng)?;
        msg(&format!("Got: {}", text));
    }
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

    // eprintln!("My private:  {}", private);
    // eprintln!("My public:   {}", public);
    // eprintln!("Foreign:     {}", foreign_key);
    // eprintln!("Base:        {}", base);
    // eprintln!("Modulo:      {}", modulo);
    // eprintln!("Encrypt key: {}", key);

    Ok(key)
}
