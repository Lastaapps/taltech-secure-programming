use std::io::stdin;

use crate::algorithms::{
    inverse_mod, random_prime, random_undivisible_with, rsa_decrypt, rsa_encrypt,
    sieve_of_eratosthenes_general,
};

mod algorithms;

enum UserIntention {
    GENERATE,
    CRACK,
    ENCRYPT,
    DECRYPT,
    EXLAMPLES,
    QUIT,
}

fn read_user_intention() -> Result<UserIntention, String> {
    println!("What do want to do?");

    loop {
        println!("(G)enerate a RSA key pair");
        println!("(C)rack a RSA key");
        println!("(E)ncrypt a text");
        println!("(D)ecrypt a text");
        println!("E(x)amples");
        println!("(Q)uit the app");

        let mut line = String::new();

        if let Err(e) = stdin().read_line(&mut line) {
            return Err(format!("Failed to read stdin: {}", e));
        };

        let chars = line.trim().as_bytes();
        let char = match chars.len() {
            1 => chars[0],
            _ => {
                eprintln!("Enter exactly 1 character from the list above");
                continue;
            }
        };

        let mode = match char {
            b'g' | b'G' => UserIntention::GENERATE,
            b'c' | b'C' => UserIntention::CRACK,
            b'e' | b'E' => UserIntention::ENCRYPT,
            b'd' | b'D' => UserIntention::DECRYPT,
            b'x' | b'X' => UserIntention::EXLAMPLES,
            b'q' | b'Q' => UserIntention::QUIT,
            _ => {
                eprintln!("This is not an allowed character");
                continue;
            }
        };

        return Ok(mode);
    }
}

fn main() -> Result<(), String> {
    loop {
        let mode = read_user_intention()?;
        let res = match mode {
            UserIntention::GENERATE => generate_keys(),
            UserIntention::CRACK => crack(),
            UserIntention::ENCRYPT => encrypt(),
            UserIntention::DECRYPT => decrypt(),
            UserIntention::EXLAMPLES => examples(),
            UserIntention::QUIT => {
                println!("Ok, bye!");
                return Ok(());
            }
        };

        if let Err(e) = res {
            // return Err(format!("Operation failed: {}", e));
            eprintln!("Operation failed: {}\n", e)
        };
        println!("\n")
    }
}

fn generate_keys() -> Result<(), String> {
    let (p1, p2, n) = loop {
        let p1 = random_prime();
        let p2 = random_prime();
        let n = p1 * p2;

        // if the total modulo was to small, we would have to decrease
        // the block size
        if n >> 32 == 0 {
            continue;
        };
        break (p1, p2, n);
    };

    let m = (p1 - 1) * (p2 - 1);
    let e = random_undivisible_with(m);
    let d = inverse_mod(e, m).unwrap();

    println!("p: {}", p1);
    println!("r: {}", p2);
    println!("n: {}", n);
    // println!("m: {}", m);
    println!("e: {}", e);
    println!("d: {}", d);

    Ok(())
}

fn crack() -> Result<(), String> {
    println!("Enter modulo n:");
    let n = read_number()?;

    println!("Enter exponent (public key):");
    let e = read_number()?;

    println!("Let me think now...");
    println!("Make sure you have compiled the program in the release variant");

    let mut res = Ok(());
    sieve_of_eratosthenes_general(u32::MAX.into(), |prime| {
        if n % prime == 0 {
            let other = n / prime;

            println!("Found it, the prime numbers are:");
            println!("{}", prime);
            println!("{}", other);

            let m = (prime - 1) * (other - 1);
            let d = inverse_mod(e, m)
                .ok_or_else(|| format!("Exponent does not have an inverse, wrong input, aborting"));

            match d {
                Ok(d) => {
                    println!("Inverse (private key): {}", d);
                }
                Err(e) => {
                    res = Err(e);
                }
            };

            return false;
        };
        true
    });

    res
}

fn examples() -> Result<(), String> {
    let biggest_prime = 2_147_483_647 as u64;
    println!(
        "Largest 32-bit prime and it's 2nd power: {} {}",
        biggest_prime,
        biggest_prime * biggest_prime,
    );

    println!("Usual exponent (0b10001): {}", 0b10001);

    Ok(())
}

fn encrypt() -> Result<(), String> {
    println!("Enter modulo n:");
    let n = read_number()?;

    println!("Enter exponent (public key):");
    let e = read_number()?;

    println!("Enter text to encrypt (one line only):");
    let line = read_line()?;

    let res = rsa_encrypt(line.trim(), e, n)?;
    println!("Base64: {}", res);

    Ok(())
}

fn decrypt() -> Result<(), String> {
    println!("Enter modulo n:");
    let n = read_number()?;

    println!("Enter inverse (private key):");
    let e = read_number()?;

    println!("Enter base64 encode text to decrypt:");
    let line = read_line()?;

    let res = rsa_decrypt(line.trim(), e, n)?;
    println!("Message: {}", res);

    Ok(())
}

fn read_line() -> Result<String, String> {
    let mut line = String::new();
    stdin()
        .read_line(&mut line)
        .map_err(|e| format!("Failed to read input: {}", e))?;
    Ok(line)
}

fn read_number() -> Result<u64, String> {
    let num: u64 = read_line()?
        .trim()
        .parse()
        .map_err(|e| format!("Failed to parse number: {}", e))?;
    Ok(num)
}
