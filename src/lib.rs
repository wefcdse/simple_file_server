pub mod utils;
#[test]
fn gen() {
    for i in 0..26 {
        let l = char::from_u32('a' as u32 + i).unwrap();
        let u = char::from_u32('A' as u32 + i).unwrap();
        print!("i = i.replace(\"{}\", \"-{}\");", u, l)
    }
}
