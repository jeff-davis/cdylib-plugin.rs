
//! Provides workarounds for two problems encountered with ``cdylib``
//! crates for plugin-style shared libraries, where the library refers
//! to symbols in the host program:
//! 1. Linking errors on some platforms due to undefined symbols
//! 1. Difficulty finding the produced shared library for testing or installation

/// call from build.rs to emit build flags for building a plugin-style cdylib
pub fn buildflags() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-cdylib-link-arg=-undefined dynamic_lookup");
    } else if cfg!(target_os = "windows") {
        println!("cargo:rustc-cdylib-link-arg=/FORCE");
    }
}

/// return the absolute path of the generated cdylib, using the
/// CARGO_PKG_NAME environment variable and the current directory.
pub fn cdylib_path() -> String {
    let pkgname = std::env::var("CARGO_PKG_NAME").unwrap();
    let libname = pkgname_to_libname(pkgname);
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    format!(
        "{}/target/{}/{}",
        std::env::current_dir().unwrap().to_str().unwrap(),
        profile,
        libname
    )
}

fn pkgname_to_libname(name: String) -> String {
    let libname = name.replace("-", "_");
    if cfg!(target_os = "windows") {
        format!("{}.dll", libname)
    } else if cfg!(target_os = "macos") {
        format!("lib{}.dylib", libname)
    } else {
        format!("lib{}.so", libname)
    }
}
