use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn build() {
    // Rerun if the c-ares source code has changed.
    println!("cargo:rerun-if-changed=c-ares");

    // We'll compile from source.  Clean up previous build, if any.
    let outdir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let build = outdir.join("build");
    let _ = fs::remove_dir_all(&build);
    fs::create_dir(&build).unwrap();

    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("c-ares");

    // Export the include path for crates dependending on c-ares
    println!("cargo:include={}", src.join("include").display());

    // Need libresolv on macos.
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=resolv");
    }

    compile(&src)
}

fn compile(src_dir: &Path) {
    let dst = cmake::Config::new(src_dir)
        .define("CARES_STATIC", "ON")
        .define("CARES_SHARED", "OFF")
        .define("CARES_BUILD_TOOLS", "OFF")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .build();

    println!("cargo:rustc-link-search={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=cares");
}
