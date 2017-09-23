extern crate bindgen;
extern crate gcc;
extern crate git2;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::option::Option;

use git2::Repository;


// stolen from: https://github.com/alexcrichton/backtrace-rs/blob/master/backtrace-sys/build.rs
macro_rules! t {
    ($e:expr) => (match $e{
        Ok(e) => e,
        Err(e) => panic!("{} failed with {}", stringify!($e), e),
    })
}


const LIBMODBUS_DIR: &'static str = "libmodbus";

fn main() {
    let dst = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();

    let build_dir = Path::new(LIBMODBUS_DIR);
    let prefix    = Path::new(&dst).join("libmodbus-root");
    let include   = Path::new(&prefix).join("include")
                                      .join("modbus");

    // Initalize git submodules
    let repo = match Repository::open("../") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open git repo: {}", e),
    };
    let submodules = match repo.submodules() {
        Ok(submodules) => submodules,
        Err(e) => panic!("failed to get submodules: {}", e),
    };
    for mut module in submodules {
        module.update(true, Option::None).expect("Failed to update submodule");
    }

    let _ = fs::remove_dir_all(env::var("OUT_DIR").unwrap());
    t!(fs::create_dir_all(env::var("OUT_DIR").unwrap()));

    let cfg = gcc::Build::new();
    let compiler = cfg.get_compiler();
    let mut flags = OsString::new();
    for (i, flag) in compiler.args().iter().enumerate() {
        if i > 0 {
            flags.push(" ");
        }
        flags.push(flag);
    }

    // Generate configure, run configure, make, make install
    run_command("Generating configure",
        Command::new("autoreconf")
            .arg("--install")
            .arg("--symlink")
            .arg("--force")
            .current_dir(&build_dir));

    run_command("Configuring libmodbus",
        Command::new("./configure")
            .arg("--prefix")
            .arg(&prefix)
            .env("CC", compiler.path())
            .env("CFLAGS", flags)
            .arg("--with-pic")
            .arg("--disable-shared")
            .arg(format!("--target={}", target))
            .arg(format!("--host={}", host))
            .current_dir(&build_dir));

    run_command("Building libmodbus",
        Command::new("make")
            .arg("install")
            .current_dir(&build_dir));

    println!("cargo:rustc-link-lib=static=modbus");
    println!("cargo:rustc-link-search=native={}/libmodbus-root/lib", dst);

    run_bindgen(&include);
}


fn run_bindgen(include: &PathBuf) {
    let include_path = format!("-I{}", include.display());

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // Do not generate unstable Rust code that
        // requires a nightly rustc and enabling
        // unstable features.
        .unstable_rust(false)
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg(include_path)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");


    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

// Helper which run a given command, and check it's success
fn run_command(which: &'static str, cmd: &mut Command) {
    assert!(cmd.status().expect(which).success(), which);
}
