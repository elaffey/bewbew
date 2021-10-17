use lazy_static::lazy_static;
use ring::signature::Ed25519KeyPair;

lazy_static! {
    static ref KEY_PAIR: Ed25519KeyPair = load_key_pair();
}

fn load_key_pair() -> Ed25519KeyPair {
    use std::io::Read;
    let mut file = std::fs::File::open("hi.p8").unwrap();
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    Ed25519KeyPair::from_pkcs8(&contents).unwrap()
}

pub fn hi() -> u32 {
    3
}
