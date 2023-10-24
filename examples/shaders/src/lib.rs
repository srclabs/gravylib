// ?? Can we abstract these?
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

// ?? Can we abstract these?

// ** mod rainbow
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
// ** mod rainbow

// ** mod circle
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


// ** mod circle