/// Build libspiro binding for Rust.

extern crate bindgen;
extern crate cc;
extern crate dos2unix;

use std::path::PathBuf;
use std::process::Command;

fn main() {
    // We aren't using libspiro's autotools build, so I wrote a generic spiro-config.h which will
    // work on all the systems I care about.
    let cp = Command::new("cp").arg("spiro-config.h").arg("libspiro/").output().unwrap();
    assert!(cp.status.success());

    cc::Build::new()
        .file("libspiro/bezctx.c")
        .file("libspiro/spiro.c")
        .file("libspiro/spiroentrypoints.c")
        .static_flag(true)
        .shared_flag(true)
        .static_crt(true)
        .compile("libspiro.a");

    println!("cargo:rerun-if-changed=wrapper.h");

    // Link built library. "lib" and ".a" are added back by Cargo.
    println!("cargo:rustc-link-lib=static=spiro");

    // Below code basically unchanged from `bindgen` manual.

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src");
    let out_file = out_path.join("lib.rs");
    bindings
        .write_to_file(out_file.clone())
        .expect("Couldn't write bindings!");

    // Windows sanity - use \n even on Windows.
    dos2unix::Dos2Unix::convert(&out_file.into_os_string().into_string().unwrap(), false);
}
