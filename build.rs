use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rustc-link-lib=z");

    let libpng_dst = cmake::build("./third_party/libpng");

    // Link against the compiled libpng.a directly (not the system one)
    let libpng_path = libpng_dst.join("lib").join("libpng.a");
    println!("cargo:rustc-link-search=native={}", libpng_dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=png");
    
    // Ensure we're using the right library by adding it explicitly
    println!("cargo:rustc-link-arg={}", libpng_path.display());

    println!("cargo:rerun-if-changed=libpng_read_fuzzer.cc");

    let abseil_path = Path::new("third_party").join("abseil-cpp");
    let libpng_include = Path::new("third_party").join("libpng");
    
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++17")
        .file("libpng_read_fuzzer.cc")
        .include(&abseil_path)
        .include(&libpng_include)
        .include(libpng_dst.join("include"))
        .compile("libpng_read_fuzzer");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
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
