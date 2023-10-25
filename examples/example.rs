// Example program using gravylib to build a simple shader program
// Import gravylib
use gravylib::*;
// Import shaders from the custom shader crate (with gravy-styled lib.rs)
use shaders::{ CIRCLE, RAINBOW };

fn main() {

    // Build shader from raw shader
    let shader = Shader::from(
        // Tip: Try changing the shader!
        // `CIRCLE` points to the shader in `shaders/src/circle.rs`
        // `RAINBOW` points to the shader in `shaders/src/rainbow.rs`
        CIRCLE
    );

    // Execute shader
    shader.execute();
}