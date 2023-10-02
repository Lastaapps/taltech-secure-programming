use std::io::stdin;

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
        match mode {
            UserIntention::GENERATE => todo!(),
            UserIntention::CRACK => todo!(),
            UserIntention::ENCRYPT => todo!(),
            UserIntention::DECRYPT => todo!(),
            UserIntention::QUIT => {
                println!("Ok, bye!");
                return Ok(())
            },
        }
    } }

