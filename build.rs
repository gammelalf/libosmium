static FILES: &[&'static str] = &[
    "area.cpp",
    "handler.cpp",
    "node.cpp",
    "node_ref_list.cpp",
    "object.cpp",
    "tag_list.cpp",
    "way.cpp",
];

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