mod counter;

use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::{HeaderValue, Request};
use axum::routing::get;
use axum::Router;
use axum_extra::routing::SpaRouter;
use clap::Parser;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[derive(Parser, Debug)]
#[clap(name = "{{project-name}}-server", about = "A Rust web server.")]
struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// the default file to serve, aka index.html
    #[clap(short = 'i', long = "index", default_value = "index.html")]
    index_file: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "../dist")]
    static_dir: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }

    tracing_subscriber::fmt::init();

    // let tracing_layer = TraceLayer::new_for_http();
    let tracing_layer = TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
        let ConnectInfo(addr) = request
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .unwrap();
        let empty_val = &HeaderValue::from_static("");
        let user_agent = request
            .headers()
            .get("User-Agent")
            .unwrap_or(empty_val)
            .to_str()
            .unwrap_or("");
        let uri = request.uri();
        tracing::debug_span!("client-addr", uri=%uri, addr = %addr, user_agent=%user_agent)
    });

    let app = Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .merge(SpaRouter::new("/assets", opt.static_dir).index_file(opt.index_file))
        .layer(ServiceBuilder::new().layer(tracing_layer));

    let app = counter::setup(app);

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Unable to start server");
}
