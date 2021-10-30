use error::Error;
use sled::CompareAndSwapError;
use std::path::Path;
use types::UserAuth;

pub struct Handle {
    db: sled::Db,
}

impl Handle {
    fn new(db: sled::Db) -> Self {
        Self { db }
    }
}

pub fn open<P: AsRef<Path>>(path: P) -> Result<Handle, Error> {
    let db = sled::open(path).map_err(|e| Error::wrap("open db", e))?;
    Ok(Handle::new(db))
}

pub fn store_user_auth(h: &Handle, user: &UserAuth) -> Result<bool, Error> {
    let bytes: Vec<u8> = types::ser(&user)?;
    let res = h.db.compare_and_swap(&user.username, None as Option<&[u8]>, Some(bytes))
        .map_err(|e| Error::wrap("store user", e))?;
    match res {
        Ok(_) => Ok(true),
        Err(CompareAndSwapError { .. }) => Ok(false),
    }
}

pub fn get_user_auth(h: &Handle, username: &str) -> Result<Option<UserAuth>, Error> {
    let bytes =
        h.db.get(username)
            .map_err(|e| Error::wrap("get UserAuth", e))?;
    if let Some(bytes) = bytes {
        let user_auth: UserAuth = types::de(bytes.as_ref())?;
        return Ok(Some(user_auth));
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let db = sled::open("tmp_db").unwrap();
        db.clear().unwrap();
        let h = Handle::new(db);
        let username = "eamonn".to_string();
        let pw_hash = b"secret".to_vec();
        let usr_auth = UserAuth { username, pw_hash };
        let res = store_user_auth(&h, &usr_auth);
        assert!(res.is_ok());
        let got = get_user_auth(&h, "eamonn").unwrap().unwrap();
        assert_eq!(&got.username, "eamonn");
        assert_eq!(&got.pw_hash, b"secret");
        let got = get_user_auth(&h, "julia").unwrap();
        assert!(got.is_none());
    }
}
