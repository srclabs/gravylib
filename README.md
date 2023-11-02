# Gravy (WIP)
###### *Ya know, my momma always said everything's better with Gravy...*

**Gravy** is an attempt to build a modern programming framework for GPU-first development. It builds on [`rust-gpu`](https://github.com/EmbarkStudios/rust-gpu) as a backend to convert Rust syntax into a GPU program, and then makes use of [`wgpu`](https://github.com/gfx-rs/wgpu) and [`winit`](https://github.com/rust-windowing/winit) to run the program with cross-platform support.

It will be usable as a standalone Rust library crate (`gravylib`), but the goal is to build an IDE-like experience on top of `gravylib` to make GPU-first development simple and accessible for all Rust developers.

**NOTE:** This project is still a Work In Progress, so it's not recommended to use it in it's current state.
***However***, if you would like to try it out, head over to [`example.rs`](./examples/shaders/src/example.rs) to find a tutorial of sorts.

Pull requests are discouraged until the initial alpha release. However, if you want to learn more about the project, you can join the [Discord](https://discord.gg/7cBw5KHe6q).

### Alpha release checklist (SUBJECT TO CHANGE, NO ETA)

- [x] Eliminate custom constants system. use gravylib_helpers::Constants instead
- [x] Implement helpers as a module expansion of gravylib
- [ ] Use `lib.rs` in place of `common.rs` main crate
- [ ] Explore traits, annotations, **macros**, etc. to reduce boilerplate
- [ ] Upgrade winit
- [ ] Upgrade wgpu
- [ ] Re-implement hot reloading
- [ ] More examples from shadertoy
- [ ] Make a custom example or two to showcase rust features in shaders
- [ ] Consider adding an example shader library crate with some helper SDF primitives.
- [ ] Add linter (clippy?)
- [ ] Cleanup, refactoring, documentation
- [ ] Shiny new README with images
- [ ] Branding? Logo? Website? Promotion?
- [ ] Release on crates.io