use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(n) => n,
            Err(e) => panic!("\n{} failed with {}\n", stringify!($e), e),
        }
    };
}

fn cp_r(dir: &Path, dst: &Path) {
    for f in t!(fs::read_dir(dir)) {
        let f = t!(f);
        let path = f.path();
        let dst = dst.join(path.file_name().unwrap());
        if t!(f.file_type()).is_dir() {
            t!(fs::create_dir_all(&dst));
            cp_r(&path, &dst);
        } else {
            let _ = fs::remove_file(&dst);
            t!(fs::copy(path, dst));
        }
    }
}

pub fn build_lib(lib: &str) {
    let target = env::var("TARGET").expect("TARGET is not found");
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("build");
    let src = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .join("pmdk");
    t!(fs::create_dir_all(&dst));
    let build_src = dst.join("src");

    cp_r(&src, &dst);
    let is_bsd = target.contains("dragonfly")
        || target.contains("freebsd")
        || target.contains("netbsd")
        || target.contains("openbsd");

    if lib == "libpmempool" {
        Command::new(if is_bsd { "gmake" } else { "make" })
            .arg("CC=clang")
            .arg("CXX=clang++")
            .arg("libpmemblk")
            .current_dir(&build_src)
            .output()
            .expect("Failed to execute make");
    }

    let output = Command::new(if is_bsd { "gmake" } else { "make" })
        .arg("CC=clang")
        .arg("CXX=clang++")
        .arg(lib)
        .current_dir(build_src)
        .output()
        .expect("Failed to execute make");

    if !output.status.success() {
        io::stderr().write_all(&output.stderr).unwrap();
        panic!()
    }
}
