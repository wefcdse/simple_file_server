async fn _a() {
    use tokio::fs;
    let mut dir = fs::read_dir("./Cargo.toml").await.unwrap();
    // dbg!(&dir);
    while let Some(e) = dir.next_entry().await.unwrap() {
        dbg!(&e.file_name());
        dbg!(&e.file_type().await.unwrap().is_file());
    }
}
#[test]
fn t() {
    let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    let h = rt.spawn(_a());
    while !h.is_finished() {}
}
