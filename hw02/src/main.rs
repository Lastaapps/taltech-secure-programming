mod algorithms;
mod client;
mod network;
mod server;
mod cracker;

extern crate exitcode;

use clap::Parser;
use client::run_client;
use server::run_server;

use crate::cracker::run_cracker;

/// Diffie–Hellman key exchange
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run as server (Bob)
    #[arg(short, long, default_value_t = false)]
    server: bool,

    /// Run as client (Alice)
    #[arg(short, long, default_value_t = false)]
    client: bool,

    /// Server address
    #[arg(long, default_value_t = String::from("localhost"))]
    host: String,

    /// Server port number
    #[arg(long, default_value_t = 42420)]
    port: u16,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    if args.server && args.client  {
        eprintln!("You need to run in either server (Bob) or client (Alice) mode.");
        std::process::exit(exitcode::USAGE);
    };

    if args.server {
        run_server(&args.host, args.port)?;
    } else if args.client {
        run_client(&args.host, args.port)?;
    } else {
        println!("To run DH in server or client mode, use -s or -c option");
        run_cracker()?;
    }

    Ok(())
}
