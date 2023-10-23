#![warn(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
// crate-specific exceptions:
// #![allow()]

use winit::event_loop::EventLoopBuilder;
use crate::graphics::run;

mod graphics;

// * Execute the main loop

pub fn execute() {
    // create event loop with hot reloading (via user events)
    let mut event_loop_builder = EventLoopBuilder::with_user_event();
    env_logger::init();
    let event_loop = event_loop_builder.build();

    // create window
    let window = winit::window::WindowBuilder::new()
        .with_title("grits alpha (WIP)")
        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)
        .unwrap();

    // run the main loop
    futures::executor::block_on(run(
        event_loop,
        window,
    ));
}
