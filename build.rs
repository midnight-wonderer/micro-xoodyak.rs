use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    println!("cargo:rustc-check-cfg=cfg(thumb1, thumb2, riscv32_std, riscv32_e)");
    println!("cargo:rerun-if-env-changed=FORCE_THUMB1");
    println!("cargo:rerun-if-env-changed=CROSS_FORCE_THUMB1");
    println!("cargo:rerun-if-env-changed=FORCE_THUMB2");
    println!("cargo:rerun-if-env-changed=CROSS_FORCE_THUMB2");
    println!("cargo:rerun-if-env-changed=FORCE_RISCV32_STD");
    println!("cargo:rerun-if-env-changed=CROSS_FORCE_RISCV32_STD");
    println!("cargo:rerun-if-env-changed=FORCE_RISCV32_E");
    println!("cargo:rerun-if-env-changed=CROSS_FORCE_RISCV32_E");

    let force_thumb1 = env::var("FORCE_THUMB1").is_ok() || env::var("CROSS_FORCE_THUMB1").is_ok();
    let force_thumb2 = env::var("FORCE_THUMB2").is_ok() || env::var("CROSS_FORCE_THUMB2").is_ok();
    let force_riscv32_std =
        env::var("FORCE_RISCV32_STD").is_ok() || env::var("CROSS_FORCE_RISCV32_STD").is_ok();
    let force_riscv32_e =
        env::var("FORCE_RISCV32_E").is_ok() || env::var("CROSS_FORCE_RISCV32_E").is_ok();

    if force_thumb1 {
        println!("cargo:rustc-cfg=thumb1");
    } else if force_thumb2 {
        println!("cargo:rustc-cfg=thumb2");
    } else if target.contains("thumbv7")
        || target.contains("thumbv8m.main")
        || target.contains("thumbv8r")
        || target.contains("armv7")
        || target.contains("armv8")
        || target.contains("armv9")
    {
        println!("cargo:rustc-cfg=thumb2");
    } else if target.contains("thumbv4")
        || target.contains("thumbv5")
        || target.contains("thumbv6")
        || target.contains("thumbv8m.base")
        || target.contains("armv4")
        || target.contains("armv5")
        || target.contains("armv6")
    {
        println!("cargo:rustc-cfg=thumb1");
    }

    if target.contains("riscv32") {
        if force_riscv32_e {
            println!("cargo:rustc-cfg=riscv32_e");
        } else if force_riscv32_std {
            println!("cargo:rustc-cfg=riscv32_std");
        } else {
            let abi = env::var("CARGO_CFG_TARGET_ABI").unwrap_or_default();
            if abi == "ilp32e" {
                println!("cargo:rustc-cfg=riscv32_e");
            } else {
                println!("cargo:rustc-cfg=riscv32_std");
            }
        }
    }
}
