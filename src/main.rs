use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
mod api;
mod dir_info;
mod global_state;
mod html;
mod sending_structs;

use clap::Parser;
#[derive(Parser, Debug)]
#[command(author="eqf", version, about, long_about = None)]
struct Args {
    /// Port to listen
    #[arg(short, long, default_value_t = 13001)]
    port: u16,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let args = Args::parse();

    let app = Router::new()
        .with_state(())
        .route("/info/*path", get(crate::dir_info::serve_dir_info))
        .route(
            "/info",
            get(|s: State<String>| async {
                crate::dir_info::serve_dir_info(s, Path("".to_owned())).await
            }),
        )
        .route(
            "/info/",
            get(|s: State<String>| async {
                crate::dir_info::serve_dir_info(s, Path("".to_owned())).await
            }),
        )
        .with_state("./".to_owned())
        .nest_service("/file", tower_http::services::ServeDir::new("./"))
        .merge(html::get_html_router());

    // run it with hyper on localhost:3000
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), args.port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use tower_http::services::fs::ServeFile;
