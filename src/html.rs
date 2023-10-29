use axum::{response::Html, routing::get, Router};
use stupid_utils::predule::*;
use tokio::fs;
const INDEX_HTML: &'static str = "./res/index.html";
const API_JS: &'static str = "./res/api.js";
const VUE_JS: &'static str = "./res/vue.global.prod.js";

pub fn get_html_router() -> Router {
    let r = Router::new()
        .route("/", get(get_index))
        .route("/vue", get(get_vue_js))
        .route("/api.js", get(get_api_js))
        .route("/res/index.html", get(get_index));
    r
}

static INNER_HTML: bool = false;

pub async fn get_index() -> Html<String> {
    let f = if INNER_HTML {
        include_str!("../res/index.html").to_owned()
    } else {
        fs::read_to_string(INDEX_HTML).await.unwrap()
    };
    Html(f)
}

pub async fn get_api_js() -> ([(&'static str, &'static str); 1], String) {
    if INNER_HTML {
        include_str!("../res/api.js").to_owned()
    } else {
        fs::read_to_string(API_JS).await.unwrap()
    }
    .map_value(|f| ([("content-type", "application/javascript")], f))
}

pub async fn get_vue_js() -> ([(&'static str, &'static str); 1], String) {
    if INNER_HTML {
        include_str!("../res/vue.global.prod.js").to_owned()
    } else {
        fs::read_to_string(VUE_JS).await.unwrap()
    }
    .map_value(|f| ([("content-type", "application/javascript")], f))
}
