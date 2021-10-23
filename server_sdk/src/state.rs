use error::Error;
use ring::signature::Ed25519KeyPair;
use std::io::Read;
use std::path::Path;

pub struct State {
    pub key_pair: Ed25519KeyPair,
    pub salt_secret: Vec<u8>,
    pub handle: store::Handle,
}

impl State {
    pub fn default() -> Result<Self, Error> {
        let key_pair = load_key_pair("key_pair.p8")?;
        let salt_secret = load_salt_secret("salt_secret")?;
        let handle = load_handle("db")?;
        let state = State {
            key_pair,
            salt_secret,
            handle,
        };
        Ok(state)
    }
}

fn load_handle<P: AsRef<Path>>(path: P) -> Result<store::Handle, Error> {
    store::open(path)
}

fn load_key_pair<P: AsRef<Path>>(path: P) -> Result<Ed25519KeyPair, Error> {
    let mut file = std::fs::File::open(path).map_err(|e| Error::wrap("opening key pair", e))?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)
        .map_err(|e| Error::wrap("reading key pair file", e))?;
    Ed25519KeyPair::from_pkcs8(&contents).map_err(|e| Error::wrap("loading key pair", e))
}

fn load_salt_secret<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    let mut file = std::fs::File::open(path).map_err(|e| Error::wrap("opening salt secret", e))?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)
        .map_err(|e| Error::wrap("reading key salt secret", e))?;
    Ok(contents)
}
