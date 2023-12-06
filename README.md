# Gravy (WIP)

###### _Ya know, my momma always said everything's better with Gravy..._

**Gravy** is a shader programming framework and toolkit built on the union of GPU programming and the [Rust Programming Language](https://www.rust-lang.org/). Gravy brings amazing features to shader development, such as **custom data structures**, **node-based pipelining**, and even **package manager support**. With Gravy, you can implement existing techniques with a simple package install, and construct novel techniques with the latest programming tools, all while effortlessly building and testing your render pipeline. **This is the future of the shaderdev.**

## What is Gravy?

**Gravy** sits in a niche with the likes of [**Shadertoy**](https://www.shadertoy.com/) and Unity's [**ShaderGraph**](https://unity.com/features/shader-graph) but stands apart in some key ways:

- **Gravy is an framework *and* a toolkit.** You can build applications with Gravy in flexible ways. Whether using all of `gravylib` as a development framework (like e.g. `bevyengine`), or using `gravylib` modules directly as a GPU toolkit (like e.g. `raylib`), the creative possibilities are endless.
- **Gravy is a development environment.** Like Shadertoy, Gravy gives you the power to build amazing and beautiful shader programs, without having to worry about asset loading, windowing, i/o, graphics APIs, etc. You can focus on building your masterpiece, and leave the dirty work to us.
- **Gravy is a node graph.** Like ShaderGraph, `gravylib` allows you to build your shader program quickly and intuitively with a simple node-oriented API. Once the API is stable, `gravylib` will be used to build an IDE-like experience for Gravy, with a visual node editor for easily building managing your render graph.
- **Gravy is Rusty.** Featuring a core built on Embark's [`rust-gpu`](https://github.com/EmbarkStudios/rust-gpu), `gravylib` lets you leverage the power of the modern and powerful [Rust Programming Language](https://www.rust-lang.org/). With Gravy, you can make more complex and beautiful shaders than ever possible before, with the most developer friendly programming experience of our age.[^1]

#### *For the nerds:*

*More technically, `gravylib` is an attempt to build a modern programming framework and development toolkit to enable GPU-first development. It builds on [`rust-gpu`](https://github.com/EmbarkStudios/rust-gpu) as a backend to compile Rust syntax into SPIR-V bytecode, and then makes use of [`wgpu`](https://github.com/gfx-rs/wgpu) and [`winit`](https://github.com/rust-windowing/winit) to run the program on the GPU with cross-platform support. Gravy is usable as a standalone Rust library crate (`gravylib`), but the goal is to build an IDE-like experience on top of `gravylib` to make GPU-first development simple and accessible for all developers.*

## Why Gravy?

Perhaps the most compelling feature of Gravy is **shader crates**. Using Rust's [official package manager](https://github.com/rust-lang/cargo) and [public user repository](https://crates.io/), you can build your own Rust packages (called **crates**) with Gravy's shader framework, and **publish them for anyone to use**. Gone are the days of copy-pasting code from another dev's blog just to implement a standard technique. With package manager support, you can use the latest tools and techniques in your own piplene just by adding the package to your project. When you start using shader crates, shader development quickly becomes more powerful and more dev-friendly at the same time.

Still, Rust has more to give to shader devs; it's also the most developer friendly language of our age, **rated highest in developer satisfaction for the last 8 years!**[^1] For an industry plagued by confusing errors and complex code bases, this is a much needed improvement. It also lowers the barrier to entry for learning shader development, by using a language people already know and love, instead of requiring developers to learn an entirely new language.

But it doesn't end there. Gravy empowers the devs of today to create the **digital masterpieces of tomorrow**, with powerful features like **node-based render graphs**, **platform-agnostic APIs**, and more. **This is the future of shader dev.**

## Where do I start?

If you would like to try it out, head over to the [official example](gravylib_example) for a tutorial of sorts.
**_However_**, this project is still a **Work In Progress**: The structure and API are in a highly fluid state and are constantly being reworked. Keep this in mind before starting any big projects with `gravylib`.

<!-- To learn more about the project, you can join the [**`_src` Discord**](https://discord.gg/7cBw5KHe6q). -->

## How can I help?

As stated above, the structure is constantly changing, so Pull Requests are discouraged until the initial alpha release.

**That being said,** if you think you could help with any of the unfinished tasks below, please reach out to me on [**Discord**](https://discord.gg/7cBw5KHe6q). I'd love to work with you!

## Alpha release checklist (v0.1.0)
**(SUBJECT TO CHANGE, NO ETA)**

- [x] Eliminate custom constants system; use `gravylib_helpers::Constants` instead
- [x] Implement `helpers` as a module expansion of `gravylib`
- [x] Use `lib.rs` as the common module instead of `common.rs`
- [x] Use macros to reduce boilerplate
- [x] Upgrade wgpu version 
- [x] Upgrade winit version
- [x] Isolate examples (they live [here](https://github.com/srclabs/gravylib_example) now)
  - [x] Add back as a git submodule :P
- [x] Start a company (lolol, welcome to `_src`! [Website](https://srclabs.dev) | [GitHub](https://github.com/srclabs))
- [ ] Reprinciple backend
- [ ] Implement hot reloading
- [ ] Basic image buffer support
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

[^1]: According to the [Annual Stack Overflow Developer Survey](https://survey.stackoverflow.co/), Rust has been voted as most loved/most admired language by developers every year since 2015.
