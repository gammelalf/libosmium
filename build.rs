static FILES: &[&'static str] = &["handler.cpp", "node.cpp", "node_ref.cpp", "way.cpp"];

fn main() {
    let mut build = cc::Build::new();
    for &file in FILES.iter() {
        println!("cargo:rerun-if-changed=src/{}", file);
        build.file(format!("src/{}", file));
    }
    build.cpp(true)
        .include("libosmium/include")
        .compile("osmium");
    println!("cargo:rustc-link-lib=static=z");
}