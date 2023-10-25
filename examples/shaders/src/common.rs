#![allow(dead_code)]

// Imports
use crate::*;
// use core::f32::consts::PI; (example)

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

// NOTE: This function is for converting particularly stubborn Shadertoy shaders to the proper linear color space.
pub fn to_linear(color: Vec4) -> Vec4 {
    vec4(
        to_linear_f32(color.x),
        to_linear_f32(color.y),
        to_linear_f32(color.z),
        color.w,
    )
}

fn to_linear_f32(color: f32) -> f32 {
    if color <= 0.04045 {
        color / 12.92
    } else {
        ((color + 0.055) / 1.055).powf(2.4)
    }
}