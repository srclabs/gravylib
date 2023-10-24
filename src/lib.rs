#![warn(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]

use bytemuck::{Pod, Zeroable};
use winit::event_loop::EventLoopBuilder;

mod graphics;

use graphics::run;
use gravylib_helpers::Constants;

pub enum ShaderType {
    Pixel,
    // One day...
    //// Compute,
    //// Audio,
    //// Mesh,
    //// Task,
}

pub struct Shader<T: From<Constants> + Copy + Clone + Pod + Zeroable> {
    #[allow(dead_code)]
    shader_type: ShaderType,
    path: String,
    entry_point: String,
    phantom: std::marker::PhantomData<T>,
}

impl<T: From<Constants> + Copy + Clone + Pod + Zeroable> Shader<T> {
    pub fn new(shader_type: ShaderType, path: &str, entry_point: &str) -> Self {
        Self {
            shader_type,
            path: path.to_string(),
            entry_point: entry_point.to_string(),
            phantom: std::marker::PhantomData,
        }
    }
}

// TODO: Use a `RenderGraphBuilder` for the external interface
pub fn execute<T: From<Constants> + Copy + Clone + Pod + Zeroable>(shader: Shader<T>) {
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
        shader
    ));
}
