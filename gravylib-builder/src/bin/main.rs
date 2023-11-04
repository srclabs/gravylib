use gravylib_builder::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    build_shader("../gravylib-helpers")?;
    build_shader("../examples/shaders")?;
    Ok(())
}