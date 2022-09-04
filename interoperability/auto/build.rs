use std::env;
use std::path::PathBuf;

use bindgen::{Builder, CargoCallbacks};

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=/path/to/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=math");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CargoCallbacks))
        // Include some basic mathematical functions (float and double)
        .allowlist_function(r"sqrt(f?)")
        .allowlist_function(r"pow(f?)")
        // Include trigonometric functions (float and double)
        .allowlist_function(r"(a?)sin(h?)(f?)")
        .allowlist_function(r"(a?)cos(h?)(f?)")
        .allowlist_function(r"(a?)tan(h?)(f?)")
        // Include exponential functions (float and double)
        .allowlist_function(r"(\w*)exp(\w*)")
        .blocklist_function(r"(\w*)exp(\w*)l")
        // Include logarithmic functions (float and double)
        .allowlist_function(r"(\w*)log(\w*)")
        .blocklist_function(r"(\w*)log(\w*)l")
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