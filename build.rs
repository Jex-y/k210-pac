use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        File::create(out.join("device.x"))
            .unwrap()
            .write_all(include_bytes!("device.x"))
            .unwrap();
        println!("cargo:rerun-if-changed=device.x");
    }
    File::create(out.join("memory-k210.x"))
        .unwrap()
        .write_all(include_bytes!("memory-k210.x"))
        .unwrap();
    println!("cargo:rerun-if-changed=memory-k210.x");
    println!("cargo:rerun-if-changed=build.rs");
}
