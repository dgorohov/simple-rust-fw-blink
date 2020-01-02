extern crate bindgen;
extern crate cc;

use std::env;
use std::path::{PathBuf, Path};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=shims/shims.h");
    println!("cargo:rerun-if-changed=shims/sdk_config.h");
    println!("cargo:rerun-if-changed=bindings.h");
    println!("cargo:rustc-link-lib=static=nrf");
    run_libmake();
    run_bindings();
}

fn run_bindings() {
    let builder = bindgen::Builder::default()
        .header("bindings.h")
        .blacklist_type("IRQn_Type")
        .blacklist_type("__va_list")
        .use_core()
        .ctypes_prefix("ctypes")
        .derive_default(true)
        .clang_args(INCLUDES.iter().map(|&x| format!("-I{}", x)).collect::<Vec<_>>())
        .clang_args(find_system_includes().iter().map(|x| format!("-I{}", x)).collect::<Vec<_>>())
        .clang_args(DEFINES.iter().map(|&x| format!("-D{}", x)).collect::<Vec<_>>())
        .clang_args(FLAGS)
        .clang_arg("-nostdlib")
        .clang_arg("-nostdinc")
        .clang_arg("-ffreestanding")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    builder
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn run_libmake() {
    let mut builder = cc::Build::new();

    builder.file("shims/shim.c")
        .debug(false);
    for inc in INCLUDES {
        builder.include(Path::new(inc));
    }
    for inc in find_system_includes() {
        builder.include(Path::new(&inc));
    }

    for define in DEFINES {
        builder.define(define, None);
    }

    for flag in FLAGS {
        builder.flag(flag);
    }

    builder
        .static_flag(true)
        .warnings(false)
        .out_dir(env::var("OUT_DIR").unwrap())
        .compile("libnrf.a")
}

fn find_system_includes() -> Vec<String> {
    let output = Command::new("arm-none-eabi-gcc")
        .arg("-E")
        .arg("-Wp,-v")
        .arg("-xc")
        .arg("/dev/null")
        .arg("-o/dev/null")
        .output()
        .expect("failed to invoke arg-none-eabi-gcc; it needs to be in your PATH");

    let mut res = Vec::new();
    for line in String::from_utf8_lossy(&output.stderr).split("\n") {
        if line.starts_with(" ") {
            res.push(format!("{}", line.trim()));
        }
    }

    res
}

static INCLUDES: &[&str] = &[
    "shims",
    "nRF5-sdk/modules/nrfx",
    "nRF5-sdk/modules/nrfx/mdk",
    "nRF5-sdk/modules/nrfx/hal",
    "nRF5-sdk/modules/nrfx/drivers",
    "nRF5-sdk/modules/nrfx/drivers/include",
    "nRF5-sdk/integration/nrfx",
    "nRF5-sdk/components/libraries/delay",
    "nRF5-sdk/components/libraries/util",
    "nRF5-sdk/components/toolchain/cmsis/include",
    "nRF5-sdk/components/drivers_nrf/nrf_soc_nosd"
];


static DEFINES: &[&str] = &[
    "CONFIG_GPIO_AS_PINRESET",
    "FLOAT_ABI_HARD",
    "NRF52",
    "NRF52840_XXAA",
    "SWI_DISABLE0",
    "__CMSIS_GCC_H",
    "SVCALL_AS_NORMAL_FUNCTION"
];

static FLAGS: &[&str] = &[
    "-ffreestanding",
    "-std=c99",
    "-mcpu=cortex-m4",
    "-mthumb",
    "-mabi=aapcs",
    "-mfloat-abi=hard",
    "-mfpu=fpv4-sp-d16",
    "-ffunction-sections",
    "-fdata-sections",
    "-fno-strict-aliasing",
    "-fno-builtin",
    "-Wno-unused-parameter",
    "-Wno-sign-compare",
    "-Wno-missing-field-initializers",
    "-fshort-enums"
];
