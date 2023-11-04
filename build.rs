use std::error::Error;

use spirv_builder::{SpirvBuilder, MetadataPrintout};

fn main() -> Result<(), Box<dyn Error>> {
    // Internal shader-like dependencies, built alongside `gravylib`
    SpirvBuilder::new("gravylib_helpers", "spirv-unknown-vulkan1.1")
        .print_metadata(MetadataPrintout::Full)
        .build()?;
    
    Ok(())
}