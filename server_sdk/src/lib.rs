use lazy_static::lazy_static;
use ring::signature::{Ed25519KeyPair, KeyPair, UnparsedPublicKey, ED25519};
use ring::{digest, pbkdf2};
use serde::{Serialize, Deserialize};
use std::num::NonZeroU32;

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

fn salt(usr: &str, salt_secret: &[u8]) -> Vec<u8> {
    let usr = usr.as_bytes();
    let len = salt_secret.len() + usr.len();
    let mut salt = Vec::with_capacity(len);
    salt.extend(salt_secret.iter());
    salt.extend(usr);
    salt
}

fn hash_pw(usr: &str, pw: &str, salt_secret: &[u8]) -> Vec<u8> {
    let salt = salt(usr, salt_secret);
    let iters = NonZeroU32::new(100_000).unwrap();
    let alg = pbkdf2::PBKDF2_HMAC_SHA256;
    let mut buf = [0u8; digest::SHA256_OUTPUT_LEN];
    pbkdf2::derive(
        alg,
        iters,
        &salt,
        pw.as_bytes(),
        &mut buf,
    );
    buf.to_vec()
}

fn verify_pw(usr: &str, pw: &str, hash: &[u8], salt_secret: &[u8]) -> bool {
    let salt = salt(usr, salt_secret);
    let iters = NonZeroU32::new(100_000).unwrap();
    let alg = pbkdf2::PBKDF2_HMAC_SHA256;
    pbkdf2::verify(
        alg,
        iters,
        &salt,
        pw.as_bytes(),
        hash,
    )
    .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_token() {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        let token = gen_token(&key_pair, "me@elaffey.com");
        let ok = verify(&key_pair, &token);
        assert!(ok);
    }

    #[test]
    fn test_pw() {
        let salt_secret = b"123";
        let hashed = hash_pw("eamonn", "secret", salt_secret);
        assert!(verify_pw("eamonn", "secret", &hashed, salt_secret));
        assert!(!verify_pw("eamonn", "wrong", &hashed, salt_secret));
        assert!(!verify_pw("julia", "secret", &hashed, salt_secret));
    }
}

pub fn hi() -> u32 {
    3
}
