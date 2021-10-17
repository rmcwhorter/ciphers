use rand::prelude::*;
use rand::distributions::{Alphanumeric};

pub fn rand_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect::<String>()
}

pub fn rand_u8(length: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    let mut out = Vec::with_capacity(length);
    for _ in 0..length {
        out.push(rng.next_u32() as u8);
    }

    out
}

pub fn rand_u8_mod26(length: usize) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(30);
    let mut rng = thread_rng();
    for _ in 0..length {
        out.push(rng.next_u32() as u8 % 26);
    }

    out
}