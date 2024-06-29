use std::process::Command;

fn main() {
    // rerun if foo.rs or bar.rs changes
    println!("cargo:rerun-if-changed=src/foo.rs");
    println!("cargo:rerun-if-changed=src/bar.rs");

    // creates the output directory in target/out
    std::fs::create_dir_all("target/out").unwrap();

    // Compile foo.rs and bar.rs into a static library
    Command::new("rustc")
        .args(&[
            "--crate-type=staticlib",
            "src/foo.rs",
            "-o",
            "target/out/libfoo.a",
        ])
        .status()
        .unwrap();

    Command::new("rustc")
        .args(&[
            "--crate-type=staticlib",
            "src/bar.rs",
            "-o",
            "target/out/libbar.a",
        ])
        .status()
        .unwrap();

    // Tell cargo to tell rustc to link the system foo library
    // shared library.
    println!("cargo:rustc-link-search=native=target/out");
    println!("cargo:rustc-link-lib=static=foo");
    println!("cargo:rustc-link-lib=static=bar");
}
