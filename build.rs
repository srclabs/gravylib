use std::error::Error;

use spirv_builder::{SpirvBuilder, MetadataPrintout};

fn main() -> Result<(), Box<dyn Error>> {
    // Internal shader-like dependencies, built alongside `gravylib`
    SpirvBuilder::new("gravylib_helpers", "spirv-unknown-vulkan1.1")
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