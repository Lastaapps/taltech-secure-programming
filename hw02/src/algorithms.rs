use bit_vec::BitVec;
use rand::Rng;

pub fn sqruare_and_multiply_mod(base: u64, power: u64, modulo: u64) -> u64 {
    if modulo < 2 {
        panic!("Wtf, modulo < 2 ???");
    }

    let base = base as u128;
    let modulo = modulo as u128;

    let mut acu = base % modulo;
    let mut res = 1u128;
    let mut power = power;

    while power > 0 {
        if (power & 1) != 0 {
            res *= acu;
            res %= modulo;
        }

        acu *= acu;
        acu %= modulo;
        power >>= 1;
    }

    res as u64
}

#[test]
fn test_sqruare_and_multiply_mod() {
    let samm = sqruare_and_multiply_mod;
    // basic SAM
    assert_eq!(samm(3, 0, 1_000_000), 1);
    assert_eq!(samm(3, 1, 1_000_000), 3);
    assert_eq!(samm(3, 2, 1_000_000), 9);
    assert_eq!(samm(3, 3, 1_000_000), 27);
    assert_eq!(samm(3, 4, 1_000_000), 81);
    assert_eq!(samm(3, 5, 1_000_000), 243);
    assert_eq!(samm(3, 6, 1_000_000), 729);
    assert_eq!(samm(3, 7, 1_000_000), 2187);

    // modulo SAM
    assert_eq!(samm(3, 0, 7), 1);
    assert_eq!(samm(3, 1, 7), 3);
    assert_eq!(samm(3, 2, 7), 2);
    assert_eq!(samm(3, 3, 7), 6);
    assert_eq!(samm(3, 4, 7), 4);
    assert_eq!(samm(3, 5, 7), 5);
    assert_eq!(samm(3, 6, 7), 1);
    assert_eq!(samm(3, 7, 7), 3);
}

/// Find all the primes up to the limit (excluding)
pub fn sieve_of_eratosthenes(limit: u64) -> Vec<u64> {
    let mut out = Vec::<u64>::with_capacity((limit as f64 / (limit as f64).ln()) as usize);

    let mut data = BitVec::from_elem((limit / 2).try_into().unwrap(), false);
    data.set(0, true); // disable 1
    out.push(2);

    let fill_limit: u64 = (limit as f64).sqrt() as u64 + 1;
    for i in 0..data.len() {
        if data[i] {
            continue;
        }

        let val = (i * 2 + 1) as u64;
        out.push(val);

        if val > fill_limit {
            continue;
        };

        for j in (((val * val - 1) as usize / 2)..data.len()).step_by(val as usize) {
            data.set(j, true);
        }
    }

    out
}

#[test]
fn test_sieve_of_eratosthenes() {
    let primes = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251,
    ];
    assert_eq!(sieve_of_eratosthenes(255), primes);
    assert_eq!(sieve_of_eratosthenes(256), primes);
    assert_eq!(sieve_of_eratosthenes(257), primes);
    assert_ne!(sieve_of_eratosthenes(258), primes);
}

pub fn random_prime() -> u64 {
    let mut rng = rand::thread_rng();
    // TODO do statistical tests
    // let primes = sieve_of_eratosthenes(/*2f64.powi(64).sqrt() as u64*/ 2u64.pow(32));
    let primes = sieve_of_eratosthenes(/*2f64.powi(64).sqrt() as u64*/ 2u64.pow(16));

    'main: loop {
        let number: u64 = rng.gen();
        let number = number | 1;
        if number < (1 << 32) {
            continue;
        }

        for prime in primes.iter() {
            if number % prime == 0 {
                continue 'main;
            }
        }
        break number;
    }
}
