extern crate rand;

use rand::Rng;


pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2, p) 
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modpow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modpow(b_pub, a, p)
}

fn modpow(mut base: u64, mut exp: u64, modulo: u64) -> u64 {
    let mut result = 1;

    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulo;
        }
        exp >>= 1;
        base = base * base % modulo;
    }

    result
}
