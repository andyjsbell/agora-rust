extern crate cpp_build;
use std::env;
use std::path::Path;

fn main() {
    
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cpp_build::Config::new()
        .file("src/cpp/agorasdk/AgoraSdk.cpp")
        .include("src/cpp/include")
        .include("src/cpp")
        .build("src/lib.rs");

    println!("cargo:rustc-link-search={}", Path::new(&dir).join("src/cpp/libs").display());
    println!("cargo:rustc-link-lib=recorder");
}