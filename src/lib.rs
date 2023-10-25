#![warn(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
// TODO: Add lints (clippy?)

use bytemuck::{Pod, Zeroable};
use winit::event_loop::EventLoopBuilder;

mod graphics;

use graphics::run;
use gravylib_helpers::{Constants, ShaderType, RawShader};

pub struct Shader<T: From<Constants> + Copy + Clone + Pod + Zeroable> {
    #[allow(dead_code)]
    shader_type: ShaderType,
    path: String,
    entry_point: String,
    phantom: std::marker::PhantomData<T>,
}

impl<T: From<Constants> + Copy + Clone + Pod + Zeroable> Shader<T>{
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

impl<T: From<Constants> + Copy + Clone + Pod + Zeroable> From<&RawShader<T>> for Shader<T> {
    fn from(raw: &RawShader<T>) -> Self {
        Self {
            shader_type: raw.shader_type,
            path: std::env::var(raw.crate_name.to_owned() + ".spv").expect("Invalid shader configuration!"),
            entry_point: raw.entry_point.to_owned(),
            phantom: raw.phantom,
        }
    }
}

// TODO: Use a `RenderGraphBuilder` for the external interface
