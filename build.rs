use std::env;
use std::fs;
fn main() {
    let current_dir = env::current_dir().unwrap();
    let mut out_str = String::new();
    let dist_base = current_dir.join("./app/dist");
    for e in walkdir::WalkDir::new(&dist_base) {
        let e = e.unwrap();
        if !e.file_type().is_file() {
            continue;
        }
        let path = e.path();
        let p = path.strip_prefix(&dist_base).unwrap();
        let p1 = format!("{}", p.display()).replace('\\', "/");

        let str1 = format!(
            "
        let app = app.route(
            \"/{}\",
            get(serve_included_file!(\"../app/dist/{}\")),
        );
        ",
            p1, p1
        );

        out_str += &str1;
        if path.file_name().unwrap().to_str().unwrap() == "index.html" {
            let mut p2 = p1.clone();
            p2.replace_range(p2.len() - "index.html".len().., "");

            let str2 = format!(
                "
            let app = app.route(
                \"/{}\",
                get(serve_included_file!(\"../app/dist/{}\")),
            );
            ",
                p2, p1
            );

            out_str += &str2;
        }

        // println!("{:?}", p);
    }

    let out_str = format!(
        "
    use axum::{{routing::get, Router}};
    use simple_file_server::serve_included_file;
    pub fn generate_router() -> Router {{
        let app = Router::new();

        {}
        app
    }}
    ",
        out_str
    );

    let out = current_dir.join("./src/generated.rs");
    fs::write(out, out_str).unwrap();
    // println!("{}", current_dir.display());
    println!("cargo:rerun-if-changed=./app/dist");
    println!("cargo:rerun-if-changed=build.rs");
}
