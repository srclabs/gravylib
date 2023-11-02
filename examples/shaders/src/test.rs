/*  ~~~~~ FORMAT ~~~~~  //

// This is an example of what a shader program looks like.
// You can uncomment the code for syntax highlighting.
// Most of it is just a recommended layout,
//   but some parts (like the entry point signature) are required.
// The boilerplate is moved to the bottom of `lib.rs` for readability;
//   this is temporary, and will be abstracted in a later version.
// When you are ready to test your new shader, head over to `lib.rs`
//   and copy/paste the boilerplate code according to the instructions there.

// ** Imports

use crate::*; // this should always be included
use core::f32::consts::TAU; // common example of a useful import

// ** Constants

// example of a constant (this one is used to enable/disable bloom)
const BLOOM: bool = true;

// ** Helpers

pub fn tonemap(col: Vec3) -> Vec3 {
    
    // example of a helper function (this one is used for tone mapping)
    
    todo!() 
}

// NOTE: This is also where you would include fancy rust things like structs, enums, traits, etc.

// ** "Entry point" (effectively)

pub fn test(
    constants: &Constants,
    frag_coord: Vec2,
) -> Vec4 {

    // this is the main function of the shader.
    // it must be named the same as the shader file, in snake_case
    // it must take the constants and the frag_coord (a Vec2) as arguments,
    //   and return a Vec4 RGBA color

    todo!()
}

// That's it! You can add more functions if you want,
//   but for now only one entry point per file is supported

// You'll notice that there is little to no boilerplate,
//   and the layout & structure is very similar to GLSL and other shader languages.
// This is by design, and is intended to make porting shaders from other languages as easy as possible.

// ** Welcome to Gravy! Have fun! **

//  ~~~~~ FORMAT ~~~~~  */