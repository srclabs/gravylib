use spirv_builder::{SpirvBuilder, MetadataPrintout};

use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");
    // While OUT_DIR is set for both build.rs and compiling the crate, PROFILE is only set in
    // build.rs. So, export it to crate compilation as well.
    let profile = env::var("PROFILE").unwrap();
    println!("cargo:rustc-env=PROFILE={profile}");

    let mut dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    // Strip `$profile/build/*/out`.
    let ok = dir.ends_with("out")
        && dir.pop()
        && dir.pop()
        && dir.ends_with("build")
        && dir.pop()
        && dir.ends_with(profile)
        && dir.pop();
    assert!(ok);
    // NOTE(eddyb) this needs to be distinct from the `--target-dir` value that
    // `spirv-builder` generates in a similar way from `$OUT_DIR` and `$PROFILE`,
    // otherwise repeated `cargo build`s will cause build script reruns and the
    // rebuilding of `rustc_codegen_spirv` (likely due to common proc macro deps).
    let dir = dir.join("gravylib-builder");
    let status = std::process::Command::new("cargo")
        .args([
            "run",
            "--release",
            "-p",
            "gravylib-builder",
            "--target-dir",
        ])
        .arg(dir)
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .status()?;
    if !status.success() {
        if let Some(code) = status.code() {
            std::process::exit(code);
        } else {
            std::process::exit(1);
        }
    }

    // Internal shader-like dependencies, built alongside `gravylib`
    SpirvBuilder::new("gravylib-helpers", "spirv-unknown-vulkan1.1")
        .print_metadata(MetadataPrintout::Full)
        .build()?;

    // External shaders, should be built alongside the dependent crate
    // Built alongside `gravylib` because it's needed for the tests/examples
    // TODO: Look into a way to build this only when needed (i.e when running tests/examples)
    // TODO: This is kinda boilerplate... Can we abstract this for devs with `gravylib_macros`?
    SpirvBuilder::new("examples/shaders", "spirv-unknown-vulkan1.1")
        .print_metadata(MetadataPrintout::Full)
        .build()?;

    Ok(())
}