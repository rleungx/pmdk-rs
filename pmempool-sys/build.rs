extern crate bindgen;
extern crate pmdk_builder;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = generate_bindings();

    bindings
        .write_to_file(PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write pmempool bindings!");
}

#[cfg(not(target_os = "windows"))]
fn generate_bindings() -> bindgen::Bindings {
    pmdk_builder::build_lib("libpmempool");
    let lib_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("build");
    println!("cargo:rustc-link-search={}", lib_path.as_path().display());

    bindgen::Builder::default()
        .header("libpmempool.h")
        .blacklist_type("max_align_t")
        .generate()
        .expect("Unable to generate bindings")
}

#[cfg(target_os = "windows")]
fn generate_bindings() -> bindgen::Bindings {
    let lib = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("vcpkg/packages/pmdk_x64-windows/lib");
    println!("cargo:rustc-link-search={}", lib.as_path().display());
    bindgen::Builder::default()
        .header("libpmempool.h")
        .header("pmemcompat.h")
        .blacklist_type("max_align_t")
        .derive_debug(false)
        .generate()
        .expect("Unable to generate bindings")
}
