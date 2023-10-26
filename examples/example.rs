// Example program using gravylib to build a simple shader program
// Import gravylib
use gravylib::*;
// Import shaders from the custom shader crate (with gravy-styled lib.rs)
#[allow(unused_imports)]
use shaders::{ CIRCLE, RAINBOW, OCEAN };

fn main() {

    // Build shader from raw shader
    let shader = Shader::from(
        // Tip: Try changing the shader!
        // Options: CIRCLE, RAINBOW, OCEAN
        OCEAN
    );

    // Execute shader
    shader.execute();
}