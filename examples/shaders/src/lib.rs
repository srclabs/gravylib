// ** BOILERPLATE **
// This chunk of code is almost entirely boilerplate,
//   and will be abstracted in a later version.
// If you are looking for the shader module boilerplate, check further down.
    #![cfg_attr(target_arch = "spirv", no_std)]
    #![deny(warnings)]
    #![allow(dead_code)]
    use spirv_std::*;
    use glam::*;
    // use `gravylib::helpers::*` instead when creating your own shaders
    use gravylib_helpers::*;
    #[cfg(target_arch = "spirv")]
    use spirv_std::num_traits::Float;
// ** BOILERPLATE **


// ** This is a shader crate
// It is a rust crate consisting of a single library,
//   which contains a number of shader modules and/or library modules.
// The `lib.rs` file is the heart of the crate.
// This can be thought of as equivalent to `common.glsl` in Shadertoy.
// All functions and constants defined/imported here
//   will be available to every shader module in the crate.
// Below you can find some examples of useful functions and constants.

// Imports
use core::f32::consts::PI;

// Constants

const BLOOM: bool = true;

// Helpers
pub fn saturate(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    // Scale, bias and saturate x to 0..1 range
    let x = saturate((x - edge0) / (edge1 - edge0));
    // Evaluate polynomial
    x * x * (3.0 - 2.0 * x)
}

pub fn mix(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

pub fn mix3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a.mul_add(vec3(1.0, 1.0, 1.0) - vec3(t, t, t), b * vec3(t, t, t))
}

pub fn pow3(v: Vec3, power: f32) -> Vec3 {
    vec3(v.x.powf(power), v.y.powf(power), v.z.powf(power))
}

pub fn exp3(v: Vec3) -> Vec3 {
    vec3(v.x.exp(), v.y.exp(), v.z.exp())
}

pub fn cos3(v: Vec3) -> Vec3 {
    vec3(v.x.cos(), v.y.cos(), v.z.cos())
}

pub fn sin3(v: Vec3) -> Vec3 {
    vec3(v.x.sin(), v.y.sin(), v.z.sin())
}

pub fn reflect(ray: Vec3, normal: Vec3) -> Vec3 {
    ray - normal * 2.0 * ray.dot(normal)
}

fn to_linear_f32(color: f32) -> f32 {
    if color <= 0.04045 {
        color / 12.92
    } else {
        ((color + 0.055) / 1.055).powf(2.4)
    }
}

/// NOTE: This function is for converting particularly stubborn Shadertoy shaders to the proper linear color space.
///
/// If you are porting a a GLSL shader from Shadertoy, and the colors look wrong,
/// first try to find any part of the code that is doing something like this:
///   `pow(color, vec3(1.0/2.2));`
/// Or, alternatively:
///   `pow(color, vec3(.4545));`
///
/// If you find any similar bits of code,
/// you can simply replace the entire thing with `color`. This will fix the colors.
///
/// If you do not find any bits like that, then to fix the colors,
/// pass the final color through this function before returning it.
///
/// (example in `rainbow.rs`)
pub fn to_linear(color: Vec4) -> Vec4 {
    vec4(
        to_linear_f32(color.x),
        to_linear_f32(color.y),
        to_linear_f32(color.z),
        color.w,
    )
}


// ** WARNING: BOILERPLATE AHEAD
// The code below is almost entirely boilerplate,
//   and will be abstracted in a later version.
// For now, if you are building your own shader crate with Gravy,
//   it is recommended to copy and paste the generic code below.
// See `test.rs` for an example of how to build a compatible shader program.

/*  ~~~~~ FORMAT ~~~~~  //

    // Example boilerplate for the shader file `test.rs`:

    // This is the shader module. It must be named the same as the shader file, in snake_case
    mod test; // Replace "test" with the name of the shader file, in snake_case

    // This is the entry point of the shader. It must be named the same as the shader file, in snake_case
    #[spirv(fragment)]
    pub fn test( // Replace "test" with the name of the shader file, in snake_case
        #[spirv(frag_coord)] in_frag_coord: Vec4,
        #[spirv(push_constant)] constants: &Constants,
        output: &mut Vec4,
    ) {
        let frag_coord = vec2(in_frag_coord.x, in_frag_coord.y);
        // Call the shader function from the shader file.
        // The name of the function must be the same as the shader file, in snake_case
        // The shader function must take the constants and the frag_coord (a Vec2) as arguments, and return a Vec4 RGBA color
        *output = test::test(constants, frag_coord); // Replace "test" with the name of the shader file, in snake_case **(both times)**
    }

    // This is the RawShader struct. It must be named the same as the shader file, in CONSTANT_CASE
    #[cfg(not(target_arch = "spirv"))]
    #[allow(dead_code)]
    pub const TEST: &RawShader = &RawShader { // Replace "TEST" with the name of the shader file, in CONSTANT_CASE
        shader_type: ShaderType::Pixel,
        crate_name: env!("CARGO_CRATE_NAME"),
        entry_point: "test", // Replace "test" with the name of the shader file, in snake_case
    };

    // Once you have copied and updated the code above,
    //   you can import your `&RawShader` into `example.rs` and test it out!

//  ~~~~~ FORMAT ~~~~~  */

// ** RAINBOW
    mod rainbow;

    #[spirv(fragment)]
    pub fn rainbow(
        #[spirv(frag_coord)] in_frag_coord: Vec4,
        #[spirv(push_constant)] constants: &Constants,
        output: &mut Vec4,
    ) {
        let frag_coord = vec2(in_frag_coord.x, in_frag_coord.y);
        *output = rainbow::rainbow(constants, frag_coord);
    }

    #[cfg(not(target_arch = "spirv"))]
    #[allow(dead_code)]
    pub const RAINBOW: &RawShader = &RawShader {
        shader_type: ShaderType::Pixel,
        crate_name: env!("CARGO_CRATE_NAME"),
        entry_point: "rainbow",
    };
// ** RAINBOW

// ** CIRCLE
    mod circle;

    #[spirv(fragment)]
    pub fn circle(
        #[spirv(frag_coord)] in_frag_coord: Vec4,
        #[spirv(push_constant)] constants: &Constants,
        output: &mut Vec4,
    ) {
        let frag_coord = vec2(in_frag_coord.x, in_frag_coord.y);
        *output = circle::circle(constants, frag_coord);
    }

    #[cfg(not(target_arch = "spirv"))]
    #[allow(dead_code)]
    pub const CIRCLE: &RawShader = &RawShader {
        shader_type: ShaderType::Pixel,
        crate_name: env!("CARGO_CRATE_NAME"),
        entry_point: "circle",
    };
// ** CIRCLE

// ** OCEAN
    mod ocean;

    #[spirv(fragment)]
    pub fn ocean(
        #[spirv(frag_coord)] in_frag_coord: Vec4,
        #[spirv(push_constant)] constants: &Constants,
        output: &mut Vec4,
    ) {
        let frag_coord = vec2(in_frag_coord.x, in_frag_coord.y);
        *output = ocean::ocean(constants, frag_coord);
    }

    #[cfg(not(target_arch = "spirv"))]
    #[allow(dead_code)]
    pub const OCEAN: &RawShader = &RawShader {
        shader_type: ShaderType::Pixel,
        crate_name: env!("CARGO_CRATE_NAME"),
        entry_point: "ocean",
    };

// ** OCEAN
