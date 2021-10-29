use super::state::State;
use error::Error;
use types::{UserAuth, SignUpReq, LoginRes, LoginReq};

pub fn sign_up(state: &State, req: SignUpReq) -> Result<(), Error> {
    let pw_hash = super::auth::hash_pw(&req.username, &req.password, &state.salt_secret);
    let user_auth = UserAuth {
        username: req.username,
        pw_hash,
    };
    store::store_user_auth(&state.handle, &user_auth)
}

pub fn login(state: &State, req: LoginReq) -> Result<LoginRes, Error> {
    if let Some(user_auth) = store::get_user_auth(&state.handle, &req.username)? {
        let ok = super::auth::verify_pw(
            &req.username,
            &req.password,
            &user_auth.pw_hash,
            &state.salt_secret,
        );
        let res = match ok {
            true => LoginRes::Success,
            false => LoginRes::Fail,
        };
        return Ok(res);
    } else {
        return Ok(LoginRes::UserNotFound);
    }
}
