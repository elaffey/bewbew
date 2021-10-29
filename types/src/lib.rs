use error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Req {
    PlusOne(u32),
    SignUpReq(SignUpReq),
    Login(LoginReq),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlusOneRes {
    pub msg: String,
    pub num: u32,
}

pub type PlusOneResType = Result<PlusOneRes, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpReq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LoginRes {
    Success,
    Fail,
    UserNotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub claims: Claims,
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuth {
    pub username: String,
    pub pw_hash: Vec<u8>,
}

pub fn ser<T: ?Sized>(value: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    bincode::serialize(value).map_err(|e| Error::wrap("ser", e))
}

pub fn de<'a, T>(bytes: &'a [u8]) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    bincode::deserialize(bytes).map_err(|e| Error::wrap("de", e))
}

pub fn de_from<R, T>(reader: R) -> Result<T, Error>
where
    R: std::io::Read,
    T: serde::de::DeserializeOwned,
{
    bincode::deserialize_from(reader).map_err(|e| Error::wrap("de_from", e))
}
