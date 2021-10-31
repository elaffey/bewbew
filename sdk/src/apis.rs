use super::state::State;
use error::Error;
use types::{LoginReq, LoginRes, PlusOneReq, PlusOneRes, SignUpReq, SignUpRes, UserAuth};

pub fn sign_up(state: &State, req: SignUpReq) -> Result<SignUpRes, Error> {
    let pw_hash = super::auth::hash_pw(&req.username, &req.password, &state.salt_secret);
    let user_auth = UserAuth {
        username: req.username,
        pw_hash,
    };
    let res = store::store_user_auth(&state.handle, &user_auth)?;
    match res {
        true => Ok(SignUpRes::Success),
        false => Ok(SignUpRes::UserAlreadyExists),
    }
}

pub fn login(state: &State, req: LoginReq) -> Result<LoginRes, Error> {
    if let Some(user_auth) = store::get_user_auth(&state.handle, &req.username)? {
        let ok = super::auth::verify_pw(
            &req.username,
            &req.password,
            &user_auth.pw_hash,
            &state.salt_secret,
        );
        if !ok {
            return Ok(LoginRes::Fail);
        }
        let token = super::auth::gen_token(&state.key_pair, &req.username)?;
        Ok(LoginRes::Success(token))
    } else {
        Ok(LoginRes::UserNotFound)
    }
}

pub fn plus_one(req: PlusOneReq) -> Result<PlusOneRes, Error> {
    if req.num == 3 {
        return Err(Error::new(String::from("I don't like 3s :(")));
    }
    let res = PlusOneRes {
        msg: String::from("hope you like it :)"),
        num: req.num + 1,
    };
    Ok(res)
}