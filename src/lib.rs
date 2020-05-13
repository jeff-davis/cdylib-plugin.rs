//! Provides workarounds for two problems encountered with ``cdylib``
//! crates for plugin-style shared libraries, where the library refers
//! to symbols in the host program:
//! 1. Linking errors on some platforms due to undefined symbols
//! 1. Difficulty finding the produced shared library for testing or installation

use std::path::PathBuf;

/// call from build.rs to emit build flags for building a plugin-style cdylib
pub fn buildflags() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-cdylib-link-arg=-undefined");
        println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
    } else if cfg!(target_os = "windows") {
        println!("cargo:rustc-cdylib-link-arg=/FORCE");
    }
}

/// return the absolute path of the generated cdylib, using the
/// CARGO_PKG_NAME environment variable and the current directory.
pub fn cdylib_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path.push("target");
    if cfg!(debug_assertions) {
        path.push("debug");
    } else {
        path.push("release");
    };
    let pkgname = std::env::var("CARGO_PKG_NAME").unwrap();
    path.push(pkgname_to_libname(&pkgname));
    return path;
}

fn pkgname_to_libname(name: &str) -> String {
    let libname = name.to_string().replace("-", "_");
    if cfg!(target_os = "windows") {
        format!("{}.dll", libname)
    } else if cfg!(target_os = "macos") {
        format!("lib{}.dylib", libname)
    } else {
        format!("lib{}.so", libname)
    }
}

// This crate is not itself a cdylib, so it won't produce a C shared
// library. However, we can test that the directory exists.
#[test]
fn cdylib_dir_exists() {
    let mut path = cdylib_path();
    path.pop();
    assert!(std::fs::metadata(path).unwrap().is_dir());
}
