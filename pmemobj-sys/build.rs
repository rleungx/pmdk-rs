extern crate bindgen;
extern crate pmdk_builder;

use std::env;
use std::path::PathBuf;

fn main() {
    pmdk_builder::build_lib("libpmemobj");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let lib_path = out_dir.join("build");
    println!("cargo:rustc-link-search={}", lib_path.as_path().display());

    #[cfg(not(target_os = "windows"))]
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::Stable_1_33)
        .header("libpmemobj.h")
        .blacklist_type("max_align_t")
        .generate()
        .expect("Unable to generate bindings");

    #[cfg(target_os = "windows")]
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::Stable_1_33)
        .header("libpmemobj.h")
        .header("pmemcompat.h")
        .blacklist_type("max_align_t")
        .derive_debug(false)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write pmem bindings!");
}
