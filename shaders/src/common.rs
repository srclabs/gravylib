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

pub fn pow(v: Vec3, power: f32) -> Vec3 {
    vec3(v.x.powf(power), v.y.powf(power), v.z.powf(power))
}

pub fn exp(v: Vec3) -> Vec3 {
    vec3(v.x.exp(), v.y.exp(), v.z.exp())
}

pub fn cos(v: Vec3) -> Vec3 {
    vec3(v.x.cos(), v.y.cos(), v.z.cos())
}

pub fn sin(v: Vec3) -> Vec3 {
    vec3(v.x.sin(), v.y.sin(), v.z.sin())
}