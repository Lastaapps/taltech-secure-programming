pub fn sqruare_and_multiply_mod(base: u64, power: u64, modulo: u64) -> u64 {
    if modulo < 2 {
        panic!("Wtf, modulo < 2 ???");
    }

    let mut acu = base % modulo;
    let mut res = 1;
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

    return res;
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
