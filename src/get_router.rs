use axum::{routing::get, Router};

use simple_file_server::server_included_file;

pub fn generate_router() -> Router {
    let app = Router::new();
    let app = app.route(
        "/index.html",
        get(server_included_file!("../app/dist/index.html")),
    );
    app
}
