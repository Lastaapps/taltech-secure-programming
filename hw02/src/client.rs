use std::net::TcpStream;

fn msg(msg: &str) {
    println!("server> {}", msg)
}

pub fn run_client(host: &str, port: u16) -> Result<(), String> {
    msg("Opening a stream");

    let address = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(address)
        .map_err(|e| format!("Failed to open a TCP connection: {}", e))?;

    msg("All done, closing");
    Ok(())
}

