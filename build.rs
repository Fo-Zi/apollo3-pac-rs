use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    // device.x: interrupt vector PROVIDE() lines. cortex-m-rt's link.x does
    // `INCLUDE device.x;` when the `device` feature is on (enabled via our `rt`).
    fs::write(out.join("device.x"), include_bytes!("device.x")).unwrap();

    // memory.x: Apollo3 FLASH/RAM map. Only consumed when an example/binary
    // is linked (cortex-m-rt's link.x includes it); harmless for downstream
    // PAC consumers that ship their own memory.x.
    fs::write(out.join("memory.x"), include_bytes!("memory.x")).unwrap();

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=device.x");
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
