// * Standard imports.
// FIXME(thedocruby) These should be abstracted eventually.
#![cfg_attr(target_arch = "spirv", no_std)]
#![deny(warnings)]
use spirv_std::*;
use glam::*;
use gravy_helpers::*;
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

mod common;
mod pixel;

use common::*;
use pixel::pixel;

// * Shader constants
// FIXME(theodcruby) These should be abstracted eventually.

// FIXME(thedocruby) These should be abstracted eventually.
#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] in_frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let frag_coord = vec2(in_frag_coord.x, in_frag_coord.y);
    *output = pixel(constants, frag_coord);
}