use axum::headers::ContentType;
use axum::response::{IntoResponse, TypedHeader};
use mime::Mime;
use std::collections::HashMap;
use std::path::Path;

pub fn _serve_included_file(path: &str, data: &'static [u8]) -> impl IntoResponse {
    let map = [
        ("bat", "application/x-msdownload"),
        ("rs", "text/plain"),
        ("txt", "text/plain"),
        ("css", "text/css"),
        ("html", "text/html"),
        ("js", "application/javascript"),
        ("png", "image/png"),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let decoded_path = Path::new(path);
    let extend_name = decoded_path.extension().unwrap().to_str().unwrap();

    let header = if let Some(ct) = map.get(extend_name) {
        let mime: Mime = ct.parse().unwrap();
        TypedHeader(ContentType::from(mime))
    } else {
        TypedHeader(ContentType::octet_stream())
    };
    let data = data.to_owned();
    return (header, data);
}
#[macro_export]
macro_rules! serve_included_file {
    ($path:literal) => {
        || async { $crate::utils::_serve_included_file($path, include_bytes!($path)) }
    };
}

// #[test]
// fn t() {
//     let app: Router<()> =
//         Router::new().route("/", get(|| async { server_included_file!("main.rs") }));

//     let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 14111));

//     tokio::runtime::Builder::new_multi_thread()
//         .enable_all()
//         .build()
//         .unwrap()
//         .block_on(axum::Server::bind(&addr).serve(app.into_make_service()))
//         .unwrap();
// }
