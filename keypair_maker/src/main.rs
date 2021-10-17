use argh::FromArgs;
use std::{io::Write, path::PathBuf};

#[derive(FromArgs, Debug)]
/// Make a Ed25519 key pair
struct Cli {
    #[argh(option)]
    /// output file location
    output: PathBuf,
}

#[derive(Debug)]
pub struct Error {
    details: String,
}

impl Error {
    #[must_use]
    pub fn new(details: String) -> Error {
        Error { details }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

fn run(cli: Cli) -> Result<(), Error> {
    let rng = ring::rand::SystemRandom::new();
    let pkcs8_bytes = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng)
        .map_err(|e| Error::new(format!("error generating keypair - {}", e)))?;
    let mut file = std::fs::File::create(cli.output)
        .map_err(|e| Error::new(format!("error creating output file - {}", e)))?;
    file.write_all(pkcs8_bytes.as_ref())
        .map_err(|e| Error::new(format!("error writing to file - {}", e)))?;
    Ok(())
}

fn main() {
    let cli: Cli = argh::from_env();
    match run(cli) {
        Ok(_) => println!("done"),
        Err(e) => eprintln!("problem! - {}", e),
    }
}
