use crate::{maybe_watch, CompiledShaderModules, Options};

use common::ShaderConstants;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::Window,
};

// * Helpers

// ? Do we need this?
// TODO: Understand this
mod shaders {
    // The usual usecase of code generation is always building in build.rs, and so the codegen
    // always happens. However, we want to both test code generation (on android) and runtime
    // compilation (on desktop), so manually fill in what would have been codegenned for desktop.
    #[allow(non_upper_case_globals)]
    pub const main_fs: &str = "main_fs";
    #[allow(non_upper_case_globals)]
    pub const main_vs: &str = "main_vs";
}

// TODO: Understand this
fn build_pipeline(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, shaders: &CompiledShaderModules) -> wgpu::RenderPipeline {

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[wgpu::PushConstantRange {
            stages: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            range: 0..std::mem::size_of::<ShaderConstants>() as u32,
        }],
    });

    // FIXME(eddyb) automate this decision by default.
    let create_module = |module| {
        let wgpu::ShaderModuleDescriptorSpirV { label, source } = module;
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label,
            source: wgpu::ShaderSource::SpirV(source),
        })
    };

    let vs_entry_point = shaders::main_vs;
    let fs_entry_point = shaders::main_fs;

    let vs_module_descr = shaders.spv_module_for_entry_point(vs_entry_point);
    let fs_module_descr = shaders.spv_module_for_entry_point(fs_entry_point);

    // HACK(eddyb) avoid calling `device.create_shader_module` twice unnecessarily.
    let vs_fs_same_module = std::ptr::eq(&vs_module_descr.source[..], &fs_module_descr.source[..]);

    let vs_module = &create_module(vs_module_descr);
    let fs_module;
    let fs_module = if vs_fs_same_module {
        vs_module
    } else {
        fs_module = create_module(fs_module_descr);
        &fs_module
    };

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: vs_module,
            entry_point: vs_entry_point,
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(wgpu::FragmentState {
            module: fs_module,
            entry_point: fs_entry_point,
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    })
}

// * Program state

#[allow(dead_code)]
struct State {
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    pipeline: wgpu::RenderPipeline,
}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new(window: Window, shaders: CompiledShaderModules) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::util::backend_bits_from_env()
                .unwrap_or(wgpu::Backends::VULKAN | wgpu::Backends::METAL | wgpu::Backends::DX12),
            dx12_shader_compiler: wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default(),
        });
        
        let surface = unsafe { instance.create_surface(&window) }
            .expect("Failed to create surface from window");

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await
        .expect("Failed to find an appropriate adapter");
    
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::PUSH_CONSTANTS,
                    limits: wgpu::Limits {
                        max_push_constant_size: 128,
                        ..Default::default()
                    },
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let pipeline = build_pipeline(&device, &config, &shaders);

        Self {
            instance,
            window,
            surface,
            adapter,
            device,
            queue,
            config,
            size,
            pipeline,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent<'_>) -> bool {
        match event {
            _ => false,
        }
    }

    fn render(&mut self, time: f32) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 0.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            let push_constants = ShaderConstants {
                width: self.window().inner_size().width,
                height: self.window().inner_size().height,
                time,
            };

            pass.set_pipeline(&self.pipeline);
            pass.set_push_constants(
                wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                0,
                bytemuck::bytes_of(&push_constants),
            );
            pass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }

    fn rebuild(&mut self, new_shaders: &CompiledShaderModules) {
        self.pipeline = build_pipeline(&self.device, &self.config, new_shaders);
    }
}

// * Run the main loop

async fn run(
    event_loop: EventLoop<CompiledShaderModules>,
    window: Window,
    shaders: CompiledShaderModules,
) {
    // * Create the state

    let mut state = State::new(window, shaders).await;

    // * Start main loop

    let start = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        // ? Is this needed?
        let _ = &state;

        // * Handle events
        match event {
            Event::MainEventsCleared =>
                state.window().request_redraw(),

            // * Handle window events
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => if !state.input(event) {
                match event {

                    // * Close window
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,

                    // * Toggle fullscreen
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::F11),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        if state.window().fullscreen().is_some() {
                            state.window().set_fullscreen(None);
                        } else {
                            state.window().set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
                        }
                    },

                    // * Resize window
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }

                    // * Ignore other window events
                    _ => {}
                }
            },

            // * Redraw window
            Event::RedrawRequested(_) => 
                match state.render(
                    start.elapsed().as_secs_f32()
                ) {
                    Ok(()) => (),
                    Err(err) => {
                        eprintln!("Error! Could not find surface texture to display to: {err:?}");
                        match err {
                            wgpu::SurfaceError::Lost => {
                                state.surface.configure(&state.device, &state.config);
                            }
                            wgpu::SurfaceError::OutOfMemory => {
                                *control_flow = ControlFlow::Exit;
                            }
                            _ => (),
                        }
                        return;
                    }
                },

            // * Rebuild pipeline when shaders are modified
            Event::UserEvent(new_module) => {
                state.rebuild(&new_module);
                state.window().request_redraw();
                *control_flow = ControlFlow::Poll;
            }
            
            // * Ignore other events
            _ => {}
        }
    });
}

// * Initialize the main loop

#[allow(clippy::match_wild_err_arm)]
pub fn start(
    options: &Options, // TODO: Eliminate this
) { 
    // create event loop with hot reloading (via user events)
    let mut event_loop_builder = EventLoopBuilder::with_user_event();
    env_logger::init();
    let event_loop = event_loop_builder.build();

    // build the shaders and watch for changes
    let shaders = maybe_watch(
        options,
        { // send reloaded shader modules to event loop (via a user event)
            let proxy = event_loop.create_proxy();
            Some(Box::new(move |res| match proxy.send_event(res) {
                Ok(it) => it,
                // ShaderModuleDescriptor is not `Debug`, so can't use unwrap/expect
                Err(_err) => panic!("Event loop dead"),
            }))
        },
    );

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
        shaders,
    ));
}