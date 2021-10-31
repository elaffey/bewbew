use error::Error;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use lazy_static::lazy_static;
use sdk::state::State;
use std::convert::Infallible;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::metadata::LevelFilter;
use tracing::{error, info, span, Level};
use tracing_futures::Instrument;
use tracing_subscriber::filter::EnvFilter;

lazy_static! {
    static ref STATE: State = State::default().unwrap();
}

async fn read_request(req: Request<Body>) -> Result<impl std::io::Read, Error> {
    use hyper::body::Buf;
    let body = hyper::body::aggregate(req)
        .await
        .map_err(|e| Error::wrap("reading request body", e))?;
    Ok(body.reader())
}

async fn route(req: Request<Body>) -> Result<Response<Body>, Error> {
    let span = span!(Level::TRACE, "route");
    let _enter = span.enter();
    let reader = read_request(req).await?;
    let req = types::de_from(reader)?;
    match req {
        types::Req::Login(r) => {
            info!("login");
            let res = match sdk::apis::login(&STATE, r) {
                Ok(ok) => Ok(ok),
                Err(e) => {
                    error!("error - {}", e);
                    Err(Error::new(String::from("unexpected error")))
                }
            };
            let bytes = types::ser(&res)?;
            Ok(Response::new(Body::from(bytes)))
        }
        types::Req::SignUpReq(r) => {
            info!("sign up");
            let res = match sdk::apis::sign_up(&STATE, r) {
                Ok(ok) => Ok(ok),
                Err(e) => {
                    error!("error - {}", e);
                    Err(Error::new(String::from("unexpected error")))
                }
            };
            let bytes = types::ser(&res)?;
            Ok(Response::new(Body::from(bytes)))
        }
        types::Req::PlusOne(r) => {
            info!("plus one");
            let res = match sdk::apis::plus_one(r) {
                Ok(ok) => Ok(ok),
                Err(e) => {
                    error!("error - {}", e);
                    Err(Error::new(String::from("unexpected error")))
                }
            };
            let bytes = types::ser(&res)?;
            Ok(Response::new(Body::from(bytes)))
        }
    }
}

async fn serve(req: Request<Body>) -> Response<Body> {
    let span = span!(
        Level::TRACE,
        "serve",
        method = ?req.method(),
        uri = ?req.uri(),
        headers = ?req.headers()
    );
    let _enter = span.enter();
    info!("received request");
    match route(req).await {
        Ok(res) => res,
        Err(e) => {
            error!("error - {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::default())
                .unwrap()
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
