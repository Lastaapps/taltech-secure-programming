use std::io::stdin;

use crate::algorithms::{random_prime, random_undivisible_with, inverse_mod};

mod algorithms;

enum UserIntention {
    GENERATE,
    CRACK,
    ENCRYPT,
    DECRYPT,
    QUIT,
}

fn read_user_intention() -> Result<UserIntention, String> {
    println!("What do want to do?");

    loop {
        println!("(G)enerate a RSA key pair");
        println!("(C)rack a RSA key");
        println!("(E)ncrypt a text");
        println!("(D)ecrypt a text");
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
            UserIntention::CRACK => todo!(),
            UserIntention::ENCRYPT => todo!(),
            UserIntention::DECRYPT => todo!(),
            UserIntention::QUIT => {
                println!("Ok, bye!");
                return Ok(());
            }
        };

        if let Err(e) = res {
            return Err(format!("Operation failed: {}", e));
        };

    }
}

fn generate_keys() -> Result<(), String> {

    let p1 = random_prime();
    let p2 = random_prime();
    let n = p1 * p2;
    let m = (p1 - 1) * (p2 - 1);
    let e = random_undivisible_with(m);
    let d = inverse_mod(e, m).unwrap();

    println!("p: {}", p1);
    println!("r: {}", p2);
    println!("n: {}", n);
    println!("m: {}", m);
    println!("e: {}", e);
    println!("d: {}", d);
    println!("");

    Ok(())
}

