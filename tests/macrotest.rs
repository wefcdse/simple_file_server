#[test]
fn t() {
    use ::simple_file_server::serve_included_file;
    let _a = serve_included_file!("./macrotest.rs");
}
#[should_panic]
#[test]
fn p() {
    Option::<usize>::None.unwrap();
    // panic!("3");
    line!();
    line!();
    line!();
}
