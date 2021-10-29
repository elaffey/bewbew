use serde::{Deserialize, Serialize};
use error::Error;
use sdk::apis::{LoginReq, LoginRes, SignUpReq};

#[derive(Serialize, Deserialize, Debug)]
enum Req {
    PlusOne(u32),
    SignUpReq(SignUpReq),
    Login(LoginReq),
}

#[derive(Serialize, Deserialize, Debug)]
struct PlusOneRes {
    msg: String,
    num: u32,
}

type PlusOneResType = Result<PlusOneRes, Error>;

async fn call_apis() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let req = Req::PlusOne(4);
    let req_bytes = bincode::serialize(&req).unwrap();
    let res = client.post("http://localhost:8080")
        .body(req_bytes)
        .send()
        .await.map_err(|e| Error::wrap("calling api", e))?;
    let code = res.status();
    dbg!(code);
    let bytes = res.bytes().await.map_err(|e| Error::wrap("getting request bytes", e))?;
    let added: PlusOneResType = bincode::deserialize(&bytes).unwrap();
    dbg!(added);

    // dbg!(res);
    Ok(())
}

async fn go() {
    match call_apis().await {
        Ok(_) => println!("success"),
        Err(e) => eprintln!("error - {}", e),
    }
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(go());
}
