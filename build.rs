use std::env;
use std::path::Path;

fn main() {
    let dir = env::var("HARVEY").unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).join("amd64/lib").display()
    );
}
