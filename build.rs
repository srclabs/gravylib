use std::error::Error;

use spirv_builder::{SpirvBuilder, MetadataPrintout};

fn main() -> Result<(), Box<dyn Error>> {
    // Internal shader-like dependencies, built alongside `gravylib`
    SpirvBuilder::new("gravylib_helpers", "spirv-unknown-vulkan1.1")
        .print_metadata(MetadataPrintout::Full)
        .build()?;

    // External shaders, should be built alongside the dependent crate
    // Currently not, since we are using `src/bin/runner.rs` as an internal testbed
    SpirvBuilder::new("examples/shaders", "spirv-unknown-vulkan1.1")
        .print_metadata(MetadataPrintout::Full)
        .build()?;

    Ok(())
}