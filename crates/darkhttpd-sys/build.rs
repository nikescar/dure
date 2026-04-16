use std::env;
use std::path::PathBuf;

fn main() {
    // Get the path to the darkhttpd source directory
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let darkhttpd_c = manifest_dir.join("darkhttpd_lib.c");

    println!("cargo:rerun-if-changed={}", darkhttpd_c.display());

    // Compile darkhttpd_lib.c as a library
    cc::Build::new()
        .file(&darkhttpd_c)
        .warnings(false) // Suppress warnings from C code
        .opt_level(2)
        .compile("darkhttpd");

    println!("cargo:rustc-link-lib=static=darkhttpd");
}
