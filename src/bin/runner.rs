// Example program using gravylib to build a simple shader program

use gravylib::*;
use gravylib_helpers::ShaderConstants;

fn main() {
    let shader = Shader::<ShaderConstants>::new(
        ShaderType::Pixel,
        env!("shaders.spv").to_string(),
        "main_fs".to_string(),
    );
    execute(shader);
}