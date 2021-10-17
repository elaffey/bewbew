use lazy_static::lazy_static;
use ring::signature::{Ed25519KeyPair, KeyPair, UnparsedPublicKey, ED25519};
use serde::{Serialize, Deserialize};

lazy_static! {
    static ref KEY_PAIR: Ed25519KeyPair = load_key_pair();
}

fn load_key_pair() -> Ed25519KeyPair {
    use std::io::Read;
    let mut file = std::fs::File::open("local.p8").unwrap();
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    Ed25519KeyPair::from_pkcs8(&contents).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    email: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    claims: Claims,
    signature: Vec<u8>,
}

pub fn gen_token(key_pair: &Ed25519KeyPair, email: &str) -> Token {
    let claims = Claims { email: String::from(email) };
    let bytes: Vec<u8> = bincode::serialize(&claims).unwrap();
    let signature: Vec<u8> = key_pair.sign(&bytes).as_ref().into();
    Token {
        claims,
        signature,
    }
}

pub fn verify(key_pair: &Ed25519KeyPair, token: &Token) -> bool {
    let bytes: Vec<u8> = bincode::serialize(&token.claims).unwrap();
    let peer_public_key_bytes = key_pair.public_key().as_ref();
    let peer_public_key = UnparsedPublicKey::new(&ED25519, &peer_public_key_bytes);
    peer_public_key.verify(&bytes, &token.signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hello() {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        let token = gen_token(&key_pair, "me@elaffey.com");
        let ok = verify(&key_pair, &token);
        assert!(ok);
    }
}

pub fn hi() -> u32 {
    3
}
