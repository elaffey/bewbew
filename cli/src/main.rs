use argh::FromArgs;
use error::Error;
use ring::rand::SecureRandom;
use std::{io::Write, path::PathBuf};

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "gen_key_pair")]
/// Make a Ed25519 key pair
struct GenKeyPair {
    #[argh(option)]
    /// output file location
    output: PathBuf,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "gen_salt_secret")]
/// Output 16 random bytes to a file
struct GenSaltSecret {
    #[argh(option)]
    /// output file location
    output: PathBuf,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommands {
    GenKeyPair(GenKeyPair),
    GenSaltSecret(GenSaltSecret),
}

#[derive(FromArgs, PartialEq, Debug)]
/// helper tools
struct Cli {
    #[argh(subcommand)]
    sub_commands: SubCommands,
}

fn gen_key_pair(args: GenKeyPair) -> Result<(), Error> {
    let rng = ring::rand::SystemRandom::new();
    let pkcs8_bytes = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng)
        .map_err(|e| Error::wrap("generating keypair", e))?;
    let mut file =
        std::fs::File::create(args.output).map_err(|e| Error::wrap("creating output file", e))?;
    file.write_all(pkcs8_bytes.as_ref())
        .map_err(|e| Error::wrap("writing to file", e))?;
    Ok(())
}

fn gen_salt_secret(args: GenSaltSecret) -> Result<(), Error> {
    let rng = ring::rand::SystemRandom::new();
    let mut bytes: [u8; 16] = [0; 16];
    rng.fill(&mut bytes)
        .map_err(|e| Error::wrap("generating random", e))?;
    let mut file =
        std::fs::File::create(args.output).map_err(|e| Error::wrap("creating output file", e))?;
    file.write_all(bytes.as_ref())
        .map_err(|e| Error::wrap("writing to file", e))?;
    Ok(())
}

fn main() {
    let cli: Cli = argh::from_env();
    let res = match cli.sub_commands {
        SubCommands::GenKeyPair(args) => gen_key_pair(args),
        SubCommands::GenSaltSecret(args) => gen_salt_secret(args),
    };
    match res {
        Ok(_) => println!("success"),
        Err(e) => eprintln!("error - {}", e),
    }
}
