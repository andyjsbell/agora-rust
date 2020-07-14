extern crate cpp_build;

fn main() {

    cpp_build::Config::new()
        .file("src/cpp/agorasdk/AgoraSdk.cpp")
        .include("src/cpp/include")
        .include("src/cpp")
        .build("src/lib.rs");

    println!("cargo:rustc-link-search={}", "src/cpp/libs");
    println!("cargo:rustc-link-lib=recorder");
}