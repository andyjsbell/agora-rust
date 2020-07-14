extern crate cpp_build;

fn main() {
    cpp_build::Config::new()
        .include("src/cpp/include")
        .include("src/cpp")
        .build("src/lib.rs");
}