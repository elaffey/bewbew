use error::Error;
use ring::signature::{Ed25519KeyPair, KeyPair, UnparsedPublicKey, ED25519};
use ring::{digest, pbkdf2};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    claims: Claims,
    signature: Vec<u8>,
}

pub fn gen_token(key_pair: &Ed25519KeyPair, email: &str) -> Result<Token, Error> {
    let claims = Claims {
        email: String::from(email),
    };
    let bytes: Vec<u8> =
        bincode::serialize(&claims).map_err(|e| Error::wrap("serialize token claims", e))?;
    let signature: Vec<u8> = key_pair.sign(&bytes).as_ref().into();
    Ok(Token { claims, signature })
}

pub fn verify_token(key_pair: &Ed25519KeyPair, token: &Token) -> Result<bool, Error> {
    let bytes: Vec<u8> =
        bincode::serialize(&token.claims).map_err(|e| Error::wrap("serialize token claims", e))?;
    let peer_public_key_bytes = key_pair.public_key().as_ref();
    let peer_public_key = UnparsedPublicKey::new(&ED25519, &peer_public_key_bytes);
    Ok(peer_public_key.verify(&bytes, &token.signature).is_ok())
}

fn salt(usr: &str, salt_secret: &[u8]) -> Vec<u8> {
    let usr = usr.as_bytes();
    let len = salt_secret.len() + usr.len();
    let mut salt = Vec::with_capacity(len);
    salt.extend(salt_secret.iter());
    salt.extend(usr);
    salt
}

pub fn hash_pw(usr: &str, pw: &str, salt_secret: &[u8]) -> Vec<u8> {
    let salt = salt(usr, salt_secret);
    let iters = NonZeroU32::new(100_000).unwrap();
    let alg = pbkdf2::PBKDF2_HMAC_SHA256;
    let mut buf = [0u8; digest::SHA256_OUTPUT_LEN];
    pbkdf2::derive(alg, iters, &salt, pw.as_bytes(), &mut buf);
    buf.to_vec()
}

pub fn verify_pw(usr: &str, pw: &str, hash: &[u8], salt_secret: &[u8]) -> bool {
    let salt = salt(usr, salt_secret);
    let iters = NonZeroU32::new(100_000).unwrap();
    let alg = pbkdf2::PBKDF2_HMAC_SHA256;
    pbkdf2::verify(alg, iters, &salt, pw.as_bytes(), hash).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_token() {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        let token = gen_token(&key_pair, "me@elaffey.com").unwrap();
        let ok = verify_token(&key_pair, &token).unwrap();
        assert!(ok);

        let claims = Claims {
            email: "me@elaffey.com".to_string(),
        };
        let token = Token {
            claims,
            signature: vec![1, 2, 3],
        };
        let ok = verify_token(&key_pair, &token).unwrap();
        assert!(!ok);
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
