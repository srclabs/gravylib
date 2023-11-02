# Gravy (WIP)

###### _Ya know, my momma always said everything's better with Gravy..._

**Gravy** is a shader programming framework built on the union of the GPU and the [Rust Programming Language](https://www.rust-lang.org/). Gravy brings amazing features to shader development, such as **custom data structures** and even **dependency/library support**, to name a few. With Gravy, you can build off of the work of other shader devs without the need to copy-paste code from a blog, or fork someone else's code. This is the future of the shaderdev.

**Gravy** sits in a niche with the likes of [**Shadertoy**](https://www.shadertoy.com/) and Unity's [**ShaderGraph**](https://unity.com/features/shader-graph) but stands apart in some key ways:

- **Gravy is a library.** You can build other applications on top of `gravylib` like building a game on a game engine, or like building a web browser on `curl`. The possibilities are endless
- **Gravy is a development environment.** Like Shadertoy, Gravy gives you the power to build amazing and beautiful shader programs, without having to worry about asset loading, windowing, i/o, graphics APIs, etc. You can focus on building your masterpiece, and leave the dirty work to us.
- **Gravy is a node graph.** Like ShaderGraph, `gravylib` allows you to build your shader program quickly and intuitively with a simple node-based API. Eventually, the Gravy IDE will even have a built-in visual node editor!
- **Gravy is Rusty.** Featuring a core built on Embark's [`rust-gpu`](https://github.com/EmbarkStudios/rust-gpu), `gravylib` lets you leverage the power of the beautiful [Rust Programming Language](https://www.rust-lang.org/), to make more complex and powerful shaders than ever possible before, and all without the need to write everything by hand.

*More technically, `gravylib` is an attempt to build a modern programming framework for GPU-first development. It builds on [`rust-gpu`](https://github.com/EmbarkStudios/rust-gpu) as a backend to convert Rust syntax into a GPU program, and then makes use of [`wgpu`](https://github.com/gfx-rs/wgpu) and [`winit`](https://github.com/rust-windowing/winit) to run the program with cross-platform support. Gravy is usable as a standalone Rust library crate (`gravylib`), but the goal is to build an IDE-like experience on top of `gravylib` to make GPU-first development simple and accessible for all developers.*

#### NOTE
This project is still a **Work In Progress**, so it's not recommended to use it in it's current state.
**_However_**, if you would like to try it out, head over to [`example.rs`](./examples/shaders/src/example.rs) to find a tutorial of sorts. Pull requests are discouraged until the initial alpha release.

To learn more about the project, you can join the [**Discord**](https://discord.gg/7cBw5KHe6q).

### Alpha release checklist (SUBJECT TO CHANGE, NO ETA)

- [x] Eliminate custom constants system; use `gravylib_helpers::Constants` instead
- [x] Implement `helpers` as a module expansion of `gravylib`
- [x] Use `lib.rs` as the common module instead of `common.rs`
- [x] Use macros to reduce boilerplate
- [ ] Upgrade dependency versions (winit, wgpu)
- [ ] Implement hot reloading
- [ ] More examples from shadertoy
- [ ] Make a custom example or two to showcase rust features in shaders
- [ ] Add an example library shader crate to showcase dependency powers
- [ ] Add linter (clippy?)
- [ ] Add automated tests
- [ ] Testing & refactoring
- [ ] Cleanup codebase
- [ ] Final quality check
- [ ] Add documentation
- [ ] Prepare for contributors
- [ ] Shiny new README (with images!)
- [ ] Branding? Logo? Website? mdBook?
- [ ] Release on crates.io
- [ ] Tell people about the project.
