use error::Error;
use serde::{Deserialize, Serialize};

pub struct Handle {
    db: sled::Db,
}

impl Handle {
    fn new(db: sled::Db) -> Self {
        Self { db }
    }
}

pub fn open() -> Result<Handle, Error> {
    let db = sled::open("db").map_err(|e| Error::wrap("open db", e))?;
    Ok(Handle::new(db))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuth {
    pub username: String,
    pub pw_hash: Vec<u8>,
}

pub fn store_user_auth(h: &Handle, user: &UserAuth) -> Result<(), Error> {
    let bytes: Vec<u8> =
        bincode::serialize(&user).map_err(|e| Error::wrap("serialize UserAuth", e))?;
    h.db.insert(&user.username, bytes)
        .map_err(|e| Error::wrap("store user", e))?;
    Ok(())
}

pub fn get_user_auth(h: &Handle, username: &str) -> Result<Option<UserAuth>, Error> {
    let bytes =
        h.db.get(username)
            .map_err(|e| Error::wrap("get UserAuth", e))?;
    if let Some(bytes) = bytes {
        let user_auth: UserAuth = bincode::deserialize(bytes.as_ref())
            .map_err(|e| Error::wrap("deserialize UserAuth", e))?;
        return Ok(Some(user_auth));
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let h = open().unwrap();
        let username = "eamonn".to_string();
        let pw_hash = b"secret".to_vec();
        let usr_auth = UserAuth { username, pw_hash };
        store_user_auth(&h, &usr_auth);
        let got = get_user_auth(&h, "eamonn").unwrap().unwrap();
        assert_eq!(&got.username, "eamonn");
        assert_eq!(&got.pw_hash, b"secret");
        let got = get_user_auth(&h, "julia").unwrap();
        assert!(got.is_none());
    }
}