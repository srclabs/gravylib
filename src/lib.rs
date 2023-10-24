#![warn(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]

use winit::event_loop::EventLoopBuilder;

mod graphics;

use graphics::run;

// TODO: implement a `Shader` struct that can be used to represent a shader

// TODO: Use a `RenderGraphBuilder` for the external interface
pub fn execute() {
    // create event loop
    let mut event_loop_builder = EventLoopBuilder::with_user_event();
    let event_loop = event_loop_builder.build();

    // create window
    let window = winit::window::WindowBuilder::new()
        .with_title("grits alpha (WIP)")
        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)
        .expect("Failed to create window!");

    // run the main loop
    futures::executor::block_on(run(
        event_loop,
        window,
    ));
}
