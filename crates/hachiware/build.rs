//! Puts `memory.x` somewhere the linker will actually look.
//!
//! cortex-m-rt's `link.x` does `INCLUDE memory.x`, and the linker resolves
//! that against its search path — which does NOT include this crate's source
//! directory. In a workspace the linker runs from the workspace root, so a
//! bare `memory.x` sitting next to Cargo.toml is invisible to it. Copying the
//! file into OUT_DIR and adding that to the search path is the standard fix.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
}
