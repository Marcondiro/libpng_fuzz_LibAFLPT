use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    let zlib_root =
        PathBuf::from(env::var("DEP_Z_ROOT").expect("libz-sys did not provide DEP_Z_ROOT"));
    let zlib_include =
        PathBuf::from(env::var("DEP_Z_INCLUDE").expect("libz-sys did not provide DEP_Z_INCLUDE"));
    let zlib_lib = zlib_root.join("lib");
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let zlib_library = if target_env == "msvc" {
        zlib_lib.join("z.lib")
    } else {
        zlib_lib.join("libz.a")
    };

    println!("cargo:rerun-if-env-changed=DEP_Z_ROOT");
    println!("cargo:rerun-if-env-changed=DEP_Z_INCLUDE");
    println!("cargo:rustc-link-search=native={}", zlib_lib.display());
    println!("cargo:rustc-link-lib=static=z");

    let libpng_dst = cmake::Config::new("./third_party/libpng")
        .define("ZLIB_ROOT", &zlib_root)
        .define("ZLIB_INCLUDE_DIR", &zlib_include)
        .define("ZLIB_LIBRARY", &zlib_library)
        .define("PNG_SHARED", "OFF")
        .define("PNG_STATIC", "ON")
        .define("PNG_TESTS", "OFF")
        .define("PNG_TOOLS", "OFF")
        .define("PNG_DEBUG_POSTFIX", "")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        libpng_dst.join("lib").display()
    );
    if target_env == "msvc" {
        println!("cargo:rustc-link-lib=static=libpng18_static");
    } else {
        println!("cargo:rustc-link-lib=static=png18");
    }

    println!("cargo:rerun-if-changed=libpng_read_fuzzer.cc");

    let abseil_path = Path::new("third_party").join("abseil-cpp");
    let libpng_include = Path::new("third_party").join("libpng");

    let mut fuzzer_build = cc::Build::new();
    fuzzer_build.cpp(true);
    if target_env == "msvc" {
        fuzzer_build.flag("/std:c++17");
    } else {
        fuzzer_build.flag("-std=c++17");
    }
    fuzzer_build
        .file("libpng_read_fuzzer.cc")
        .include(&abseil_path)
        .include(&libpng_include)
        .include(&zlib_include)
        .include(libpng_dst.join("include"))
        .compile("libpng_read_fuzzer");
}
