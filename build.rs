use std::{
    env,
    path::PathBuf,
    fs::copy,
};

#[cfg(feature = "stm32_rt")]
const MEMORY_MAP: &'static str = "memory.stm32_rt.x";

#[cfg(feature = "nrf_rt")]
const MEMORY_MAP: &'static str = "memory.nrf_rt.x";

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    copy(PathBuf::from(MEMORY_MAP), out.join("memory.x")).unwrap();
    println!("cargo:rerun-if-changed=memory.stm32_rt.x");
    println!("cargo:rerun-if-changed=memory.nrf_rt.x");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rustc-link-search={}", out.display());
}