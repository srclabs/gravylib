// Example program using gravylib to execute standalone shaders from a simple shader crate

// ** FOR TINKERERS & SHADER DEVS:
// See `shaders/lib.rs` for an example of how to build a compatible shader crate.

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

        OCEAN

    );

    // Execute shader
    shader.execute();
}