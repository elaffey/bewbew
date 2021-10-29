use error::Error;
use types::{PlusOneResType, Req};

async fn call_apis() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let req = Req::PlusOne(4);
    let req_bytes = types::ser(&req).unwrap();
    let res = client
        .post("http://localhost:8080")
        .body(req_bytes)
        .send()
        .await
        .map_err(|e| Error::wrap("calling api", e))?;
    let code = res.status();
    dbg!(code);
    let bytes = res
        .bytes()
        .await
        .map_err(|e| Error::wrap("getting request bytes", e))?;
    let added: PlusOneResType = types::de(&bytes).unwrap();
    match added {
        Ok(a) => {
            dbg!(a);
        }
        Err(e) => {
            dbg!(e);
        }
    };
    Ok(())
}

async fn go() {
    match call_apis().await {
        Ok(_) => println!("success"),
        Err(e) => eprintln!("error - {}", e),
    }
}

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(go());
}
