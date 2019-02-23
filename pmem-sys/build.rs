extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = generate_bindings();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write pmem bindings!");
}

#[cfg(not(target_os = "windows"))]
fn generate_bindings() -> bindgen::Bindings {
    println!("cargo:rustc-link-lib=pmem");
    let bindings = bindgen::Builder::default()
        .header("libpmem.h")
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
        .header("libpmem.h")
        .header("pmemcompat.h")
        .derive_debug(false)
        .generate()
        .expect("Unable to generate bindings");
    bindings
}
