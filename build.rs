use std::error::Error;

use spirv_builder::{SpirvBuilder, MetadataPrintout};

fn main() -> Result<(), Box<dyn Error>> {
    SpirvBuilder::new("shaders", "spirv-unknown-vulkan1.1")
        .print_metadata(MetadataPrintout::Full)
        .build()?;
    SpirvBuilder::new("gravy_helpers", "spirv-unknown-vulkan1.1")
        .print_metadata(MetadataPrintout::Full)
        .build()?;
    Ok(())
}