fn main() {
    println!("cargo:rerun-if-changed=foo.c");
    cc::Build::new()
        .cargo_metadata(false)
        .file("foo.c")
        .compile("foo");
    println!(
        "cargo:rustc-link-search=native={}",
        std::env::var("OUT_DIR").unwrap()
    )
}
