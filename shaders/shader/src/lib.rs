// * Standard imports.
// FIXME(thedocruby) These should be abstracted eventually.
#![cfg_attr(target_arch = "spirv", no_std)]
#![deny(warnings)]
use spirv_std::*;
use glam::*;
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

// * Start pretty shader :>
    // Ported to Rust from <https://www.shadertoy.com/view/mtyGWy>

    // Imports
    use common::*;
    use core::f32::consts::TAU;

    // Helpers
    pub fn palette(t: f32) -> Vec3 {
        let a = vec3(0.5, 0.5, 0.5);
        let b = vec3(0.5, 0.5, 0.5);
        let c = vec3(1.0, 1.0, 1.0);
        let d = vec3(0.263,0.416,0.557);

        cos(TAU * (c * t + d)).mul_add(b, a)
    }

    // "Entry point" (effectively)
    pub fn pixel( 
        constants: &ShaderConstants,
        frag_coord: Vec2,
    ) -> Vec4 {
        let mut uv = (frag_coord * 2.0 - vec2(constants.width as f32, constants.height as f32))
            / constants.height as f32;

        let uv0 = uv;
        let mut final_color = Vec3::splat(0.0);
        
        for i in 0..4 {
            uv = (uv * 1.5).fract() - 0.5;

            let mut d = uv.length() * (-1.0 * uv0.length()).exp();
            d = (d * 8.0 + constants.time).sin() / 8.0;
            d = d.abs();
            d = (0.01 / d).powf(2.0);

            let col = palette(
                uv0.length() +
                (i as f32)*0.4 +
                constants.time*0.4
            );

            final_color += col * d;
        }
            
        final_color.extend(1.0)
    }

// * See? Pretty shader :>

// * Actual entry points.
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