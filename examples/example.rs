// Example program using gravylib to build a simple shader program

use gravylib::*;
use shaders::RainbowConstants;

fn main() {
    // Tip: Try changing the `entry_point`!
    // "circle" points to the shader in `shaders/src/circle.rs`
    // "rainbow" points to the shader in `shaders/src/rainbow.rs`
    let shader = Shader::<RainbowConstants>::new(
        ShaderType::Pixel,
        env!("shaders.spv"),
        //// "rainbow"
        "circle"
    );
    execute(shader);
}