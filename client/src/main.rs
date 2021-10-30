use error::Error;
use reqwest::Client;
use types::{LoginReq, LoginRes, PlusOneRes, Req, SignUpReq, SignUpRes};

async fn call<T>(client: &Client, req: Req) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let req_bytes = types::ser(&req).unwrap();
    let res = client
        .post("http://localhost:8080")
        .body(req_bytes)
        .send()
        .await
        .map_err(|e| Error::wrap("calling api", e))?;
    let bytes = res
        .bytes()
        .await
        .map_err(|e| Error::wrap("getting request bytes", e))?;
    types::de::<T>(&bytes)
}

async fn do_adding(client: &Client, n: u32) -> Result<Result<PlusOneRes, Error>, Error> {
    let req = Req::PlusOne(n);
    call(&client, req).await
}

async fn sign_up(client: &Client, req: SignUpReq) -> Result<Result<SignUpRes, Error>, Error> {
    let req = Req::SignUpReq(req);
    call(&client, req).await
}

async fn login(client: &Client, req: LoginReq) -> Result<Result<LoginRes, Error>, Error> {
    let req = Req::Login(req);
    call(&client, req).await
}

async fn test_do_add(client: &Client) {
    let req = 3;
    let res = do_adding(&client, req).await;
    println!("{:?}", res);
}

async fn test_sign_up(client: &Client) {
    let req = SignUpReq {
        username: String::from("eamonn"),
        password: String::from("secret"),
    };
    let res = sign_up(&client, req).await;
    println!("{:?}", res);
}

async fn test_login(client: &Client) {
    let req = LoginReq {
        username: String::from("eamonn"),
        password: String::from("secret"),
    };
    let res = login(&client, req).await;
    println!("{:?}", res);
}

async fn go() {
    let client = reqwest::Client::new();
    test_do_add(&client).await;
    test_sign_up(&client).await;
    test_login(&client).await;
}

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(go());
}
