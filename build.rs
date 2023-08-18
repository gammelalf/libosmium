fn main() {
    println!("cargo:rerun-if-changed=src/libosmium.cpp");
    cc::Build::new()
        .cpp(true)
        .include("libosmium/include")
        .file("src/libosmium.cpp")
        .compile("osmium");
    println!("cargo:rustc-link-lib=z");
}
