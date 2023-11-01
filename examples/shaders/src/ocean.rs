// Ported to Rust from <https://www.shadertoy.com/view/MdXyzX>
// NOTE: Mouse input has been removed from the original

// ** Imports

use crate::*;
// use core::f32::consts::TAU;

// ** Constants

const DRAG_MULT: f32 = 0.28; // changes how much waves pull on the water
const WATER_DEPTH: f32 = 1.0; // how deep is the water
const CAMERA_HEIGHT: f32 = 1.5; // how high the camera should be
const ITERATIONS_RAYMARCH: u32 = 12; // waves iterations of raymarching
const ITERATIONS_NORMAL: u32 = 40; // waves iterations when calculating normals

// ** Helpers

// Calculates wave value and its derivative, 
// for the wave direction, position in space, wave frequency and time
fn wavedx(position: Vec2, direction: Vec2, frequency: f32, timeshift: f32) -> Vec2 {
  let x = direction.dot(position) * frequency + timeshift;
  let wave = (x.sin() - 1.0).exp();
  let dx = wave * x.cos();
  vec2(wave, -dx)
}

// Calculates waves by summing octaves of various waves with various parameters
fn getwaves(mut position: Vec2, iterations: u32, time: f32) -> f32 {
  let mut iter: f32 = 0.0; // this will help generating well distributed wave directions
  let mut frequency= 1.0; // frequency of the wave, this will change every iteration
  let mut time_multiplier = 2.0; // time multiplier for the wave, this will change every iteration
  let mut weight = 1.0;// weight in final sum for the wave, this will change every iteration
  let mut sum_of_values = 0.0; // will store final sum of values
  let mut sum_of_weights = 0.0; // will store final sum of weights
  for _i in 0..iterations {
    // generate some wave direction that looks kind of random
    let p = vec2(iter.sin(), iter.cos());
    // calculate wave data
    let res = wavedx(position, p, frequency, time * time_multiplier);

    // shift position around according to wave drag and derivative of the wave
    position += p * res.y * weight * DRAG_MULT;

    // add the results to sums
    sum_of_values += res.x * weight;
    sum_of_weights += weight;

    // modify next octave parameters
    weight *= 0.82;
    frequency *= 1.18;
    time_multiplier *= 1.07;

    // add some kind of random value to make next wave look random too
    iter += 1232.399963;
  }
  // calculate and return
  sum_of_values / sum_of_weights
}

// Raymarches the ray from top water layer boundary to low water layer boundary
fn raymarchwater(camera: Vec3, start: Vec3, end: Vec3, depth: f32, time: f32) -> f32 {
  let mut pos = start;
  let dir = (end - start).normalize();
  for _i in 0..64 {
    // the height is from 0 to -depth
    let height = getwaves(pos.xz(), ITERATIONS_RAYMARCH, time) * depth - depth;
    // if the waves height almost nearly matches the ray height, assume its a hit and return the hit distance
    if height + 0.01 > pos.y {
      return pos.distance(camera);
    }
    // iterate forwards according to the height mismatch
    pos += dir * (pos.y - height);
  }
  // if hit was not registered, just assume hit the top layer, 
  // this makes the raymarching faster and looks better at higher distances
  start.distance(camera)
}

// Calculate normal at point by calculating the height at the pos and 2 additional points very close to pos
fn normal(pos: Vec2, e: f32, depth: f32, time: f32) -> Vec3 {
  let ex = vec2(e, 0.0);
  let h = getwaves(pos.xy(), ITERATIONS_NORMAL, time) * depth;
  let a = vec3(pos.x, h, pos.y);

  (a - vec3(pos.x - e, getwaves(pos.xy() - ex.xy(), ITERATIONS_NORMAL, time) * depth, pos.y))
  .cross(a - vec3(pos.x, getwaves(pos.xy() + ex.yx(), ITERATIONS_NORMAL, time) * depth, pos.y + e))
  .normalize()
}

// Helper function generating a rotation matrix around the axis by the angle
fn create_rotation_matrix_axis_angle(axis: Vec3, angle: f32) -> Mat3{
  let s = angle.sin();
  let c = angle.cos();
  let oc = 1.0 - c;
  return mat3(
    vec3(oc * axis.x * axis.x + c,          oc * axis.x * axis.y - axis.z * s, oc * axis.z * axis.x + axis.y * s), 
    vec3(oc * axis.x * axis.y + axis.z * s, oc * axis.y * axis.y + c,          oc * axis.y * axis.z - axis.x * s), 
    vec3(oc * axis.z * axis.x - axis.y * s, oc * axis.y * axis.z + axis.x * s, oc * axis.z * axis.z + c         )
  );
}

// Helper function that generates camera ray based on UV and mouse
fn get_ray(frag_coord: Vec2, resf: Vec2, mouse_norm: Vec2) -> Vec3 {
  let mut uv = ((frag_coord.xy() / resf.xy()) * 2.0 - 1.0) * vec2(resf.x / resf.y, 1.0);
  uv.y = -uv.y;
  // for fisheye, uncomment following line and comment the next one
  // let proj: Vec3 = (vec3(uv.x, uv.y, 1.0) + vec3(uv.x, uv.y, -1.0) * uv.length().pow(2.0) * 0.05).normalize();  
  let proj = vec3(uv.x, uv.y, 1.5).normalize();
  if resf.x < 600.0 {
    return proj;
  }
  return create_rotation_matrix_axis_angle(vec3(0.0, -1.0, 0.0), 3.0 * ((mouse_norm.x + 0.5) * 2.0 - 1.0)) 
    * create_rotation_matrix_axis_angle(vec3(1.0, 0.0, 0.0), 0.5 + 1.5 * ((mouse_norm.y * 1.5) * 2.0 - 1.0))
    * proj;
}

// Ray-Plane intersection checker
fn intersect_plane(origin: Vec3, direction: Vec3, point: Vec3, normal: Vec3) -> f32 { 
  (normal.dot(point - origin) / normal.dot(direction)).clamp(-1.0, 9991999.0)
}

// Some very barebones but fast atmosphere approximation
fn extra_cheap_atmosphere(raydir: Vec3, mut sundir: Vec3) -> Vec3 {
  sundir.y = sundir.y.max(-0.07);
  let special_trick = 1.0 / (raydir.y * 1.0 + 0.1);
  let special_trick2 = 1.0 / (sundir.y * 11.0 + 1.0);
  let raysundt = sundir.dot(raydir).abs().powf(2.0);
  let sundt = sundir.dot(raydir).max(0.0).powf(8.0);
  let mymie = sundt * special_trick * 0.2;
  let suncolor = mix3(Vec3::ONE, (Vec3::ONE - vec3(5.5, 13.0, 22.4) / 22.4).max(Vec3::ZERO), special_trick2);
  let bluesky= vec3(5.5, 13.0, 22.4) / 22.4 * suncolor;
  let mut bluesky2 = (bluesky - vec3(5.5, 13.0, 22.4) * 0.002 * (special_trick + -6.0 * sundir.y * sundir.y)).max(Vec3::ZERO);
  bluesky2 *= special_trick * (0.24 + raysundt * 0.24);
  bluesky2 * (1.0 + 1.0 * (1.0 - raydir.y).powf(3.0)) + mymie * suncolor
} 

// Calculate where the sun should be, it will be moving around the sky
fn get_sun_direction(time: f32) -> Vec3 {
  vec3((time * 0.1).sin(), 1.0, (time * 0.1).cos()).normalize()
}

// Get atmosphere color for given direction
fn get_atmosphere(dir: Vec3, time: f32) -> Vec3 {
   extra_cheap_atmosphere(dir, get_sun_direction(time)) * 0.5
}

// Get sun color for given direction
fn get_sun(dir: Vec3, time: f32) -> f32 {
  get_sun_direction(time).dot(dir).max(0.0).powf(720.0) * 210.0
}

// Great tonemapping function from my other shader: https://www.shadertoy.com/view/XsGfWV
fn aces_tonemap(color: Vec3) -> Vec3 {  
  let m1 = mat3(
    vec3(0.59719, 0.07600, 0.02840),
    vec3(0.35458, 0.90834, 0.13383),
    vec3(0.04823, 0.01566, 0.83777),
  );
  let m2 = mat3(
    vec3( 1.60475, -0.10208, -0.00327),
    vec3(-0.53108,  1.10813, -0.07276),
    vec3(-0.07367, -0.00605,  1.07602)
  );
  let v = m1 * color;  
  let a = v * (v + 0.0245786) - 0.000090537;
  let b = v * (0.983729 * v + 0.4329510) + 0.238081;
  (m2 * (a / b)).clamp(Vec3::ZERO, Vec3::ONE)//.powf(1.0 / 2.2)
}

// ** "Entry point" (effectively)

// Main
pub fn ocean(constants: &Constants, frag_coord: Vec2) -> Vec4 {
  // get the ray
  let resf = vec2(constants.width as f32, constants.height as f32);
  // let mousef = vec2(constants.mousex as f32, constants.mousey as f32);
  // let mouse_norm = (constants.mousef.xy() / resf.xy()) // normalize mouse coords
  let ray = get_ray(frag_coord, resf, /*mouse_norm*/vec2(0.0, 0.0));
  if ray.y >= 0.0 {
    // if ray.y is positive, render the sky
    let c = get_atmosphere(ray, constants.time) + get_sun(ray, constants.time);
    return aces_tonemap(c * 2.0).extend(1.0);
  }

  // now ray.y must be negative, water must be hit
  // define water planes
  let water_plane_high = Vec3::ZERO;
  let water_plane_low = vec3(0.0, -WATER_DEPTH, 0.0);

  // define ray origin, moving around
  let origin = vec3(constants.time, CAMERA_HEIGHT, constants.time);

  // calculate intersections and reconstruct positions
  let high_plane_hit = intersect_plane(origin, ray, water_plane_high, vec3(0.0, 1.0, 0.0));
  let low_plane_hit = intersect_plane(origin, ray, water_plane_low, vec3(0.0, 1.0, 0.0));
  let high_hit_pos = origin + ray * high_plane_hit;
  let low_hit_pos = origin + ray * low_plane_hit;

  // raymatch water and reconstruct the hit pos
  let dist = raymarchwater(origin, high_hit_pos, low_hit_pos, WATER_DEPTH, constants.time);
  let water_hit_pos = origin + ray * dist;

  // calculate normal at the hit position
  let mut n = normal(water_hit_pos.xz(), 0.01, WATER_DEPTH, constants.time);

  // smooth the normal with distance to avoid disturbing high frequency noise
  n = mix3(n, vec3(0.0, 1.0, 0.0), 0.8 * ((dist*0.01).sqrt() * 1.1).min(1.0));

  // calculate fresnel coefficient
  let fresnel = 0.04 + (1.0-0.04)*((1.0 - ray.dot(-n).max(0.0)).powf(5.0));

  // reflect the ray and make sure it bounces up
  let mut r = reflect(ray, n).normalize();
  r.y = r.y.abs();
  
  // calculate the reflection and approximate subsurface scattering
  let reflection = get_atmosphere(r, constants.time) + get_sun(r, constants.time);
  let scattering = vec3(0.0293, 0.0698, 0.1717) * (0.2 + (water_hit_pos.y + WATER_DEPTH) / WATER_DEPTH);

  // return the combined result
  let c = fresnel * reflection + (1.0 - fresnel) * scattering;
  aces_tonemap(c * 2.0).extend(1.0)
}