// Ported to Rust from <https://www.shadertoy.com/view/3ltSW2>

// ** Imports

use crate::*;
use core::f32::consts::PI;

// ** Helpers

fn circle_sdf(p: Vec2, r: f32) -> f32 {
    p.length()-r
}

// ** "Entry point" (effectively)

pub fn circle( constants: &CircleConstants, frag_coord: Vec2) -> Vec4 {
	let p: Vec2 = (2.0 * frag_coord
        - vec2(constants.width as f32,
            constants.height as f32))
        / (constants.height as f32);

	let d = circle_sdf(p,0.5);
    
	// coloring
    let mut col: Vec3 = if d > 0.0 {
        vec3(0.9,0.6,0.3)
    } else {
        vec3(0.65,0.85,1.0)
    };
    
    col *= 1.0 - (-6.0*d.abs()).exp();
	col *= 0.8 + 0.2*(150.0*(d.abs() + (constants.time * PI).cos() * 0.1)).cos();
	col = mix3(
        col,
        Vec3::ONE,
        1.0-smoothstep(0.0,0.01,d.abs())
    );

	to_linear(col.extend(1.0))
}