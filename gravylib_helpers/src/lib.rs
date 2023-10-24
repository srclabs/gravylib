#![cfg_attr(target_arch = "spirv", no_std)]
#![deny(warnings)]
use spirv_std::*;
use glam::*;

use bytemuck::{Pod, Zeroable};
#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Constants {
    pub width: u32,
    pub height: u32,
    pub time: f32,
    pub gravylib: [u32; 3]
}

#[spirv(vertex)]
pub fn pixel_vs(#[spirv(vertex_index)] vert_idx: i32, #[spirv(position)] builtin_pos: &mut Vec4) {
    // Create a "full screen triangle" by mapping the vertex index.
    // Ported from https://www.saschawillems.de/blog/2016/08/13/vulkan-tutorial-on-rendering-a-fullscreen-quad-without-buffers/
    let uv = vec2(((vert_idx << 1) & 2) as f32, (vert_idx & 2) as f32);
    let pos = 2.0 * uv - Vec2::ONE;

    *builtin_pos = pos.extend(0.0).extend(1.0);
}