use std::io::stdin;

use crate::algorithms::sieve_of_eratosthenes_general;

pub fn run_cracker() -> Result<(), String> {
    println!("Hi, I'm a cracker, your favourite password cracker.");
    println!("I will be working quite a lot, so please compile me with optimizations.");
    println!("cargo build --release");

    print_primes_suggestions();

    loop {
        println!("Enter your favourite prime numbers product, so I can crack it four you (up to 64b):");

        let number = read_number()?;

        println!("Here we go!!!");

        crack(number);

        println!("\nOr maybe not nextime, give me more!");
    }
}

fn print_primes_suggestions() {
    println!("Some suggested large prime products:");
    let primes = [
        524_287u32,
        6_700_417,
        2_147_483_647,
    ];

    primes.iter().for_each(|prime| println!("Prime: {}", prime));

    for prime1 in primes {
        for prime2 in primes {
            let product: u64 = prime1 as u64 * prime2 as u64;
            println!("{}", product)
        }
    };

    println!()
}

fn read_number() -> Result<u64, String> {
    let mut line = String::new();
    stdin().read_line(&mut line)
        .map_err(|e| format!("Failed to read stdin: {}", e))?;

    let number: u64 = line.trim().parse()
        .map_err(|e| format!("Failed to parse the number: {}", e))?;

    Ok(number)
}

/// Yeah, I could quite easily cache the prime numbers,
/// but I think it would be against the spirit of this assignment.
fn crack(target: u64) {
    sieve_of_eratosthenes_general(u32::MAX.into(), |prime| {
        if target % prime == 0 {
            let other = target / prime;
            println!("Found it, your numbers are:");
            println!("{}", prime);
            println!("{}", other);
            println!("See you next time, bye!");

            return false;
        };
        true
    })
}
