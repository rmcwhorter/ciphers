/*
* The Shift Cipher; page 2
*/

use rand::prelude::*;

pub fn encrypt(message: &Vec<u8>, key: u8) -> Vec<u8> {
    message.into_iter().map(|x| (*x + key) % 26).collect()
}

pub fn decrypt(message: &Vec<u8>, key: u8) -> Vec<u8> {
    message.into_iter().map(|x| (*x + 26 - key) % 26).collect()
}

pub fn test(length: usize) {
    let mut rng = thread_rng();
    let char_vec = crate::util::rand_u8_mod26(length);
    let key = (rng.next_u32() as u8) % 26;
    let ct = encrypt(&char_vec, key);
    let pt = decrypt(&ct, key);
    println!("{:?}\n{:?}\n{:?}\n{}", &char_vec, &ct, &pt, &char_vec == &pt);
}