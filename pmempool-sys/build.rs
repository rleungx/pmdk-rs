extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = generate_bindings();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write pmemcto bindings!");
}

#[cfg(not(target_os = "windows"))]
fn generate_bindings() -> bindgen::Bindings {
    println!("cargo:rustc-link-lib=pmempool");
    let bindings = bindgen::Builder::default()
        .header("libpmempool.h")
        .blacklist_type("max_align_t")
        .generate()
        .expect("Unable to generate bindings");
    bindings
}

#[cfg(target_os = "windows")]
fn generate_bindings() -> bindgen::Bindings {
    let lib = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("vcpkg/packages/pmdk_x64-windows/lib");
    println!("cargo:rustc-link-search={}", lib.as_path().display());
    let bindings = bindgen::Builder::default()
        .header("libpmempool.h")
        .header("pmemcompat.h")
        .blacklist_type("max_align_t")
        .derive_debug(false)
        .generate()
        .expect("Unable to generate bindings");
    bindings
}
