
mod client;
mod common;
mod server;

extern crate exitcode;

use clap::Parser;
use client::run_client;
use server::run_server;

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
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    if (args.server && args.client) || (!args.server && !args.client) {
        eprintln!("You need to run in either server (Bob) or client (Alice) mode.");
        std::process::exit(exitcode::USAGE);
    };

    if args.server {
        run_server()?;
    } else if args.client {
        run_client()?;
    };

    Ok(())
}

