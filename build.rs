use std::fs;

fn main() {
    let var = fs::read_to_string("config").unwrap();
    println!("cargo:rustc-env=URL={var}");
}