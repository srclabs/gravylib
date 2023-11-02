// Example program using gravylib to build a simple shader program

// ** FOR TINKERERS & SHADER DEVS:
// See `test.rs` for an example of how to build a compatible shader program.

// Import gravylib
use gravylib::*;
// Import shaders from the custom shader crate (with gravy-styled lib.rs)
#[allow(unused_imports)]
use shaders::{ CIRCLE, RAINBOW, OCEAN };

fn main() {

    // Build shader from raw shader
    let shader = Shader::from(

        // TIP: Try changing the shader program below!
        // Options: CIRCLE, RAINBOW, OCEAN

        RAINBOW

    );

    // Execute shader
    shader.execute();
}