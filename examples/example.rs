// Example program using gravylib to build a simple shader program

use gravylib::*;
#[allow(unused_imports)]
use shaders::{ RainbowConstants, CircleConstants };

fn main() {
    // Tip: Try changing the shader!
    // "circle" uses CircleConstants and points to the shader in `shaders/src/circle.rs`
    // "rainbow" uses RainbowConstants and points to the shader in `shaders/src/rainbow.rs`
    let shader = 
    
    Shader::<CircleConstants /*RainbowConstants*/>::new(
        ShaderType::Pixel,
        env!("shaders.spv"),
        "circle" //"rainbow"
    );
    execute(shader);
}