use bit_vec::BitVec;
use rand::Rng;
use seeded_random::Random;

pub fn sqruare_and_multiply_mod(base: u64, power: u64, modulo: u64) -> u64 {
    let modulo = match modulo {
        0 => 1u128 << 64,
        1 => panic!("Wtf, modulo < 2 ???"),
        _ => modulo as u128,
    };
    let base = base as u128;

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
    let prime_no_limit = 2u64.pow(16);
    let primes = sieve_of_eratosthenes(prime_no_limit);

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

        if !miller_rabin_test_loop(number, 420) {
            continue;
        }

        break number;
    }
}

pub fn gcd(x: u64, y: u64) -> (u64, (i64, i64)) {
    let mut r: (i128, i128) = (x as i128, y as i128);
    let mut s: (i128, i128) = (1, 0);
    let mut t: (i128, i128) = (0, 1);

    while r.1 != 0 {
        let q = r.0 / r.1;
        r = (r.1, r.0 - q * r.1);
        s = (s.1, s.0 - q * s.1);
        t = (t.1, t.0 - q * t.1);
    }

    // safe from definition of the algorithm
    (r.0 as u64, (s.0 as i64, t.0 as i64))
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(2, 5), (1, (-2, 1)));
    assert_eq!(gcd(5, 2), (1, (1, -2)));

    assert_eq!(gcd(2, 4), (2, (1, 0)));
    assert_eq!(gcd(4, 2), (2, (0, 1)));

    assert_eq!(gcd(6, 8), (2, (-1, 1)));
    assert_eq!(gcd(8, 6), (2, (1, -1)));

    assert_eq!(gcd(1, 5), (1, (1, 0)));
    assert_eq!(gcd(5, 1), (1, (0, 1)));

    // assert_eq!(gcd(0, 5).0, 0);
    // assert_eq!(gcd(5, 0).0, 0);
    // assert_eq!(gcd(0, 0).0, 0);

    for x in 0..42 {
        for y in 0..42 {
            let (res, (a, b)) = gcd(x, y);
            let res = res as i64;
            let x = x as i64;
            let y = y as i64;
            assert_eq!(res, a * x + b * y)
        }
    }
}

pub fn inverse_mod(x: u64, module: u64) -> Option<u64> {
    let (g, (a, _)) = gcd(x % module, module);

    if g != 1 {
        return None;
    }

    let idk = a % module as i64;
    let idk = if idk < 0 { idk + module as i64 } else { idk };

    Some(idk as u64)
}

#[test]
fn test_inverse_mod() {
    assert_eq!(inverse_mod(0, 5), None);
    assert_eq!(inverse_mod(1, 5), Some(1));
    assert_eq!(inverse_mod(2, 5), Some(3));
    assert_eq!(inverse_mod(3, 5), Some(2));
    assert_eq!(inverse_mod(4, 5), Some(4));

    assert_eq!(inverse_mod(0, 6), None);
    assert_eq!(inverse_mod(1, 6), Some(1));
    assert_eq!(inverse_mod(2, 6), None);
    assert_eq!(inverse_mod(3, 6), None);
    assert_eq!(inverse_mod(4, 6), None);
    assert_eq!(inverse_mod(5, 6), Some(5));

    assert_eq!(inverse_mod(0, 9), None);
    assert_eq!(inverse_mod(1, 9), Some(1));
    assert_eq!(inverse_mod(2, 9), Some(5));
    assert_eq!(inverse_mod(3, 9), None);
    assert_eq!(inverse_mod(4, 9), Some(7));
    assert_eq!(inverse_mod(5, 9), Some(2));
    assert_eq!(inverse_mod(6, 9), None);
    assert_eq!(inverse_mod(7, 9), Some(4));
    assert_eq!(inverse_mod(8, 9), Some(8));

    for x in 0..42 {
        for m in 2..42 {
            if let Some(i) = inverse_mod(x, m) {
                assert_eq!(x * i % m, 1);
            }
        }
    }
}

pub fn random_undivisible_with(m: u64) -> u64 {
    if m == 1 {
        panic!("All the numbers are dividible by 1");
    }
    if m == 0 {
        panic!("We don't do 0 divisions");
    }

    loop {
        let num: u64 = rand::thread_rng().gen();
        if gcd(num, m).0 == 1 {
            return num;
        }
    }
}

/// return false if the number is compounded for sure
fn miller_rabin_test_loop(num: u64, iteration: usize) -> bool {
    for _ in 0..iteration {
        let victim = rand::thread_rng().gen();
        if !miller_rabin_test(victim, num) {
            return false;
        }
    }
    true
}

/// Runs Miller-Rabin test for prime numbers
/// @param a - chosen victim
/// @param p - prime to test
/// @return false if the number is not prime for sure
fn miller_rabin_test(a: u64, p: u64) -> bool {
    // p - 1 = 2^b * m
    let (b, m) = {
        let mut m = p - 1;
        let mut b = 0;
        while m % 2 != 0 {
            m /= 2;
            b += 1;
        }
        (b, m)
    };

    let mut am = sqruare_and_multiply_mod(a, m, p) as u128;
    if am == 1 {
        return true;
    }

    let p = p as u128;
    let target = p - 1;

    for _ in 0..b {
        if am == target {
            return true;
        }
        am *= am;
        am %= p;
    }

    false
}

#[test]
fn test_miller_rabin_test() {
    assert!(!miller_rabin_test(5, 21));
    assert!(miller_rabin_test(5, 13));
    assert!(miller_rabin_test(7, 25)); // even though 25 is not a prime
}

pub fn prng_cipher_encrypt(msg: &str, prng: &Random) -> Result<Vec<u8>, String> {
    let mut data = msg.as_bytes().to_vec();
    padding_add(&mut data)?;
    prng_cipher_apply(&mut data, prng)?;
    Ok(data)
}

pub fn prng_cipher_decrypt(mut data: Vec<u8>, prng: &Random) -> Result<String, String> {
    padding_remove(&mut data)?;
    prng_cipher_apply(&mut data, prng)?;
    let msg = String::from_utf8(data).map_err(|e| format!("Failed to decode UTF8 bytes: {}", e))?;
    Ok(msg)
}

fn prng_cipher_apply(data: &mut [u8], prng: &Random) -> Result<(), String> {
    // if data.len() % 8 != 0 {
    //     return Err("Wrong input buffer length".into());
    // }

    let _ = data.iter_mut().map(|b| {
        let code: u8 = prng.gen();
        *b ^= code;
    });

    Ok(())
}

#[allow(dead_code)]
pub fn exp_cipher_encrypt(msg: &str, key: u64) -> Result<Vec<u8>, String> {
    let mut data = msg.as_bytes().to_vec();
    padding_add(&mut data)?;
    exp_cipher_apply(&mut data, key)?;
    Ok(data)
}

#[allow(dead_code)]
pub fn exp_cipher_decrypt(mut data: Vec<u8>, key: u64) -> Result<String, String> {
    padding_remove(&mut data)?;
    exp_cipher_apply(&mut data, key)?;
    let msg = String::from_utf8(data).map_err(|e| format!("Failed to decode UTF8 bytes: {}", e))?;
    Ok(msg)
}

fn exp_cipher_apply(data: &mut [u8], key: u64) -> Result<(), String> {
    if data.len() % 8 != 0 {
        return Err("Wrong input buffer".into());
    }

    for i in (0..data.len()).step_by(8) {
        // fuck Rust
        let num = u64::from_be_bytes([
            data[i],
            data[i + 1],
            data[i + 2],
            data[i + 3],
            data[i + 4],
            data[i + 5],
            data[i + 6],
            data[i + 7],
        ]);

        let res = sqruare_and_multiply_mod(num, key, 0);
        let res = res.to_be_bytes();

        data[i..(i + 8)].copy_from_slice(&res[i..(i + 8)]);
    }

    Ok(())
}

fn padding_add(data: &mut Vec<u8>) -> Result<(), String> {
    let padding = (8 - data.len() % 8) as u8;

    let content = (padding << 4) + padding;
    for _ in 0..padding {
        data.push(content);
    }

    Ok(())
}

fn padding_remove(data: &mut Vec<u8>) -> Result<(), String> {
    if data.len() % 8 != 0 {
        return Err("Wrong buffer len on input".into());
    }
    if data.is_empty() {
        return Err("Empty buffer on input".into());
    }

    let padding = data.last().unwrap() >> 4;

    for _ in 0..padding {
        data.pop();
    }

    Ok(())
}

#[test]
fn test_padding() -> Result<(), String> {
    let mut data1 = Vec::<u8>::new();
    let mut data2 = Vec::<u8>::new();

    for i in 0..42 {
        padding_add(&mut data1)?;
        assert_eq!(data1.len() % 8, 0);

        padding_remove(&mut data1)?;
        assert_eq!(data1, data2);

        data1.push(i);
        data2.push(i);
    }

    Ok(())
}

// #[test]
#[allow(dead_code)]
fn test_exp_cipher() -> Result<(), String> {
    let strings = ["", "a", "aa", "aaa", "aaaa", "aaaaa"];
    let keys = [37, 59, 1001];

    for key_encrypt in keys {
        let key_decrypt = inverse_mod(key_encrypt, u64::MAX).unwrap();

        for string in strings {
            let data = exp_cipher_encrypt(string, key_encrypt)?;
            let res = exp_cipher_decrypt(data, key_decrypt)?;
            assert_eq!(string, res);
        }
    }

    Ok(())
}
