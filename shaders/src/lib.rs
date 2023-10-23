// * Standard imports.
// FIXME(thedocruby) These should be abstracted eventually.
#![cfg_attr(target_arch = "spirv", no_std)]
#![deny(warnings)]
use spirv_std::*;
use glam::*;
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

mod common;
mod pixel;

use common::*;
use pixel::pixel;

// * Shader constants
// FIXME(theodcruby) These should be abstracted eventually.
use bytemuck::{Pod, Zeroable};
#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub time: f32,
}

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