use gravylib_builder::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Internal shader-like dependencies, built alongside `gravylib`
    build_shader("../gravylib-helpers")?;

    // External shaders, should be built alongside the dependent crate
    // Built alongside `gravylib` because it's needed for the tests/examples
    // TODO: Look into a way to build this only when needed (i.e when running tests/examples)
    build_shader("../examples/shaders")?;

    Ok(())
}