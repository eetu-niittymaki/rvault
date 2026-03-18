use std::env;
use std::fs;

fn main() {
    dotenv::dotenv().ok();

    let master_pass = env::var("MASTER_PASSWORD").expect("MASTER_PASSWORD must be set in .env");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("built_env.rs");

    fs::write(
        &dest_path,
        format!("pub const MASTER_PASSWORD: &str = {:?};", master_pass)
    )
    .unwrap();

    println!("cargo:rerun-if-changed=.env");
}