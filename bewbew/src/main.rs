use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::metadata::LevelFilter;
use tracing::{error, info, span, Level};
use tracing_futures::Instrument;
use tracing_subscriber::filter::EnvFilter;

async fn serve(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let span = span!(
        Level::TRACE,
        "request",
        method = ?req.method(),
        uri = ?req.uri(),
        headers = ?req.headers()
    );
    let _enter = span.enter();
    info!("received request");
    Ok(Response::new(Body::from("Hello World!")))
}

pub async fn go() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let server_span = span!(Level::INFO, "server", %addr);
    let _enter = server_span.enter();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let service = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(serve)) }
    });

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
