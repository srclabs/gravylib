#![warn(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
// TODO: Add lints (clippy?)

use winit::event_loop::EventLoopBuilder;

mod graphics;

use graphics::run;
pub use gravylib_helpers as helpers;
use helpers::{ ShaderType, RawShader };

pub struct Shader {
    #[allow(dead_code)]
    shader_type: ShaderType,
    path: String,
    entry_point: String,
}

impl Shader{
    pub fn execute(self) {
        // create event loop
        let mut event_loop_builder = EventLoopBuilder::with_user_event();
        let event_loop = event_loop_builder.build();

        // create window
        let window = winit::window::WindowBuilder::new()
            .with_title("gravylib alpha (WIP)")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
            .build(&event_loop)
            .expect("Failed to create window!");

        // run the main loop
        futures::executor::block_on(run(
            event_loop,
            window,
            self
        ));
    }
}

impl From<&RawShader> for Shader {
    fn from(raw: &RawShader) -> Self {
        Self {
            shader_type: raw.shader_type,
            path: std::env::var(raw.crate_name.to_owned() + ".spv").expect("Invalid shader configuration!"),
            entry_point: raw.entry_point.to_owned(),
        }
    }
}

// TODO: Use a `RenderGraphBuilder` for the external interface


// Prepare for initial alpha release: (SUBJECT TO CHANGE)
// done: eliminate custom constants system. use gravylib_helpers::Constants instead
// done: implement helpers as a module expansion of gravylib main crate

// todo: explore traits, annotations, macros, etc. to reduce boilerplate
// todo: explore using `lib.rs` in place of `common.rs`
// todo: upgrade winit
// todo: upgrade wgpu
// todo: re-implement hot reloading
// todo: more examples from shadertoy
// todo: make a custom example or two to showcase rust features in shaders
// todo: consider adding an example shader library crate with some helper SDF primitives.
// todo: cleanup, refactoring, documentation
// todo: Shiny new README with images
// todo: Branding? Logo? Website? Promotion?
// todo: Release on crates.io
