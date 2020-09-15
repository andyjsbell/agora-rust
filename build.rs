extern crate cpp_build;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut extract = Command::new("tar");
    extract
        .arg("xvfz")
        .arg("Agora_Recording_SDK_for_Linux_v3.0.1.tar");
    extract.status().expect("failed to extract SDK");

    let mut cp = Command::new("cp");
    cp.arg("./Agora_Recording_SDK_for_Linux_FULL/bin/AgoraCoreService")
        .arg("/tmp/AgoraCoreService");
    cp.status().expect("failed to copy service");

    let mut rm = Command::new("rm");
    rm.arg("-rf").arg("./Agora_Recording_SDK_for_Linux_FULL");
    rm.status().expect("failed to clean up");

    cpp_build::Config::new()
        .file("src/cpp/agorasdk/AgoraSdk.cpp")
        .include("src/cpp/include")
        .include("src/cpp")
        .build("src/lib.rs");

    println!(
        "cargo:rustc-link-search={}",
        Path::new(&dir).join("src/cpp/libs").display()
    );
    println!("cargo:rustc-link-lib=recorder");
}
