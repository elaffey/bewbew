use error::Error;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde::{Deserialize, Serialize};
use server_sdk::apis::{LoginReq, LoginRes, SignUpReq};
use std::convert::Infallible;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::metadata::LevelFilter;
use tracing::{error, info, span, Level};
use tracing_futures::Instrument;
use tracing_subscriber::filter::EnvFilter;

#[derive(Serialize, Deserialize, Debug)]
enum Req {
    SignUpReq(SignUpReq),
    Login(LoginReq),
}

async fn read_request<T>(req: Request<Body>) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
    T: Sized,
{
    use hyper::body::Buf;
    let body = hyper::body::aggregate(req)
        .await
        .map_err(|e| Error::wrap("reading request body", e))?;
    let result = bincode::deserialize_from(body.reader())
        .map_err(|e| Error::wrap("deserialising request body", e))?;
    Ok(result)
}

async fn serve(req: Request<Body>) -> Response<Body> {
    let span = span!(
        Level::TRACE,
        "request",
        method = ?req.method(),
        uri = ?req.uri(),
        headers = ?req.headers()
    );
    let _enter = span.enter();
    info!("received request");
    match read_request::<Req>(req).await {
        Ok(r) => {
            dbg!(r);
            Response::new(Body::from("hihi"))
        }
        Err(e) => {
            let err_msg = format!("Error - {}", e);
            Response::new(Body::from(err_msg))
        }
    }
}

async fn serve_fn(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(serve(req).await)
}

pub async fn go() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let server_span = span!(Level::INFO, "server", %addr);
    let _enter = server_span.enter();

    let service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(serve_fn)) });

    let server = Server::bind(&addr)
        .serve(service)
        .instrument(server_span.clone());

    info!("Listening");

    match server.await {
        Ok(result) => info!("Got result {:?}", result),
        Err(e) => error!("Got error {:?}", e),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let h = server_sdk::hi();
    dbg!(h);
    let filter = EnvFilter::from_default_env()
        .add_directive(LevelFilter::TRACE.into())
        .add_directive("hyper=info".parse()?);
    let subscriber = tracing_subscriber::fmt().with_env_filter(filter).finish();
    tracing::subscriber::set_global_default(subscriber)?;
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(go());
    Ok(())
}
