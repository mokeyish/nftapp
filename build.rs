// build.rs

fn main() {
    cc::Build::new()
        .file("include/foo.c")
        .file("include/nftset.c")
        // .shared_flag(true)
        .static_flag(true)
        .compile("nftset");

    let _ = bindgen::Builder::default()
        .header("include/nftset.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/ffi/nftset.rs");

    let _ = bindgen::Builder::default()
        .header("include/foo.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/ffi/foo.rs");
}
