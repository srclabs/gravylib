// ?? Can we abstract this file?
#![cfg_attr(target_arch = "spirv", no_std)]
#![deny(warnings)]
use bytemuck::{Pod, Zeroable};
use spirv_std::*;
use glam::*;
use gravylib_helpers::*;
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

mod common;
use common::*;

// ** RAINBOW
    mod rainbow;

    #[derive(Copy, Clone, Pod, Zeroable)]
    #[repr(C)]
    pub struct RainbowConstants {
        pub width: u32,
        pub height: u32,
        pub time: f32,
    }

    impl From<Constants> for RainbowConstants {
        fn from(constants: Constants) -> Self {
            Self {
                width: constants.width,
                height: constants.height,
                time: constants.time,
            }
        }
    }

    #[spirv(fragment)]
    pub fn rainbow(
        #[spirv(frag_coord)] in_frag_coord: Vec4,
        #[spirv(push_constant)] constants: &RainbowConstants,
        output: &mut Vec4,
    ) {
        let frag_coord = vec2(in_frag_coord.x, in_frag_coord.y);
        *output = rainbow::rainbow(constants, frag_coord);
    }

    #[cfg(not(target_arch = "spirv"))]
    #[allow(dead_code)]
    pub const RAINBOW: &RawShader<RainbowConstants> = &RawShader {
        shader_type: ShaderType::Pixel,
        crate_name: env!("CARGO_CRATE_NAME"),
        entry_point: "rainbow",
        phantom: std::marker::PhantomData,
    };
// ** RAINBOW

// ** CIRCLE
    mod circle;

    #[derive(Copy, Clone, Pod, Zeroable)]
    #[repr(C)]
    pub struct CircleConstants {
        pub width: u32,
        pub height: u32,
        pub time: f32,
    }

    impl From<Constants> for CircleConstants {
        fn from(constants: Constants) -> Self {
            Self {
                width: constants.width,
                height: constants.height,
                time: constants.time,
            }
        }
    }

    #[spirv(fragment)]
    pub fn circle(
        #[spirv(frag_coord)] in_frag_coord: Vec4,
        #[spirv(push_constant)] constants: &CircleConstants,
        output: &mut Vec4,
    ) {
        let frag_coord = vec2(in_frag_coord.x, in_frag_coord.y);
        *output = circle::circle(constants, frag_coord);
    }

    #[cfg(not(target_arch = "spirv"))]
    #[allow(dead_code)]
    pub const CIRCLE: &RawShader<CircleConstants> = &RawShader {
        shader_type: ShaderType::Pixel,
        crate_name: env!("CARGO_CRATE_NAME"),
        entry_point: "circle",
        phantom: std::marker::PhantomData,
    };
// ** CIRCLE

// ** OCEAN
    mod ocean;
    
    #[derive(Copy, Clone, Pod, Zeroable)]
    #[repr(C)]
    pub struct OceanConstants {
        pub width: u32,
        pub height: u32,
        pub time: f32,
    }

    impl From<Constants> for OceanConstants {
        fn from(constants: Constants) -> Self {
            Self {
                width: constants.width,
                height: constants.height,
                time: constants.time,
            }
        }
    }

    #[spirv(fragment)]
    pub fn ocean(
        #[spirv(frag_coord)] in_frag_coord: Vec4,
        #[spirv(push_constant)] constants: &OceanConstants,
        output: &mut Vec4,
    ) {
        let frag_coord = vec2(in_frag_coord.x, in_frag_coord.y);
        *output = ocean::ocean(constants, frag_coord);
    }

    #[cfg(not(target_arch = "spirv"))]
    #[allow(dead_code)]
    pub const OCEAN: &RawShader<OceanConstants> = &RawShader {
        shader_type: ShaderType::Pixel,
        crate_name: env!("CARGO_CRATE_NAME"),
        entry_point: "ocean",
        phantom: std::marker::PhantomData,
    };

// ** OCEAN

//  ~~~~~ FORMAT ~~~~~
//     Example for the shader `test.rs`:
//
//     mod test; // <--- This is the name of the shader in snake_case
//
       // Constants are unique for each shader, they can be any subset of `gravylib_helpers::Constants`
//     #[derive(Copy, Clone, Pod, Zeroable)]
//     #[repr(C)]
//     pub struct TestConstants { // Replace "Test" with the name of the shader in PascalCase
//         pub width: u32,
//         pub height: u32,
//         pub time: f32,
//     }
//
//     impl From<Constants> for TestConstants { // Here again, replace "Test" with the name of the shader in PascalCase
//         fn from(constants: Constants) -> Self {
//             Self {
//                 width: constants.width,
//                 height: constants.height,
//                 time: constants.time,
//             }
//         }
//     }
//
       // This is the entry point of the shader. It must be named the same as the shader file, in snake_case
//     #[spirv(fragment)]
//     pub fn test( // Replace "test" with the name of the shader in snake_case
//         #[spirv(frag_coord)] in_frag_coord: Vec4,
//         #[spirv(push_constant)] constants: &TestConstants, // Replace "Test" with the name of the shader in PascalCase
//         output: &mut Vec4,
//     ) {
//         let frag_coord = vec2(in_frag_coord.x, in_frag_coord.y);
           // Call the shader function from the `test.rs` file.
           // The name of the function must be the same as the shader file, in snake_case
           // The shader function must take the constants and the frag_coord (a Vec2) as arguments, and return a Vec4 RGBA color
//         *output = test::test(constants, frag_coord); // Replace "test" with the name of the shader in snake_case (both times)
//     }
//
       // This is the RawShader struct. It must be named the same as the shader file, in CONSTANT_CASE
//     #[cfg(not(target_arch = "spirv"))]
//     #[allow(dead_code)]
//     pub const TEST: &RawShader<TestConstants> = &RawShader { // Replace "TEST" and "Test" with the name of the shader in CONSTANT_CASE and PascalCase, respectively.
//         shader_type: ShaderType::Pixel,
//         crate_name: env!("CARGO_CRATE_NAME"),
//         entry_point: "test", // Replace "test" with the name of the shader in snake_case
//         phantom: std::marker::PhantomData,
//     };
