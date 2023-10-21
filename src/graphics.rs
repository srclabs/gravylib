use crate::{maybe_watch, CompiledShaderModules, Options};

use common::ShaderConstants;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::Window,
};

// * Helpers

mod shaders {
    // The usual usecase of code generation is always building in build.rs, and so the codegen
    // always happens. However, we want to both test code generation (on android) and runtime
    // compilation (on desktop), so manually fill in what would have been codegenned for desktop.
    #[allow(non_upper_case_globals)]
    pub const main_fs: &str = "main_fs";
    #[allow(non_upper_case_globals)]
    pub const main_vs: &str = "main_vs";
}

// * Run the main loop

async fn run(
    options: Options,
    event_loop: EventLoop<CompiledShaderModules>,
    window: Window,
    compiled_shader_modules: CompiledShaderModules,
) {
    // * Create the `wgpu` instance

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::util::backend_bits_from_env()
        .unwrap_or(wgpu::Backends::VULKAN | wgpu::Backends::METAL),
        dx12_shader_compiler: wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default(),
    });

    // * Create the initial surface

    let initial_surface = unsafe { instance.create_surface(&window) }
            .expect("Failed to create surface from window");
    
    // * Initialize the adapter

    let adapter = wgpu::util::initialize_adapter_from_env_or_default(
        &instance,
        // Request an adapter which can render to our surface
        Some(&initial_surface),
    )
    .await
    .expect("Failed to find an appropriate adapter");

    // * Configure the device features & limits

    let mut features = wgpu::Features::PUSH_CONSTANTS;
    if options.force_spirv_passthru {
        features |= wgpu::Features::SPIRV_SHADER_PASSTHROUGH;
    }

    let limits = wgpu::Limits {
        max_push_constant_size: 128,
        ..Default::default()
    };

    // * Create the logical device and command queue with the adapter

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features,
                limits,
            },
            None,
        )
        .await
        .expect("Failed to create device");

    // * Configure surface with device & adapter

    let auto_configure_surface =
        |adapter: &_, device: &_, surface: wgpu::Surface, size: winit::dpi::PhysicalSize<_>| {
            let mut surface_config = surface
                .get_default_config(adapter, size.width, size.height)
                .unwrap_or_else(|| {
                    panic!(
                        "Missing formats/present modes in surface capabilities: {:#?}",
                        surface.get_capabilities(adapter)
                    )
                });

            // FIXME(eddyb) should this be toggled by a CLI arg?
            // NOTE(eddyb) VSync was disabled in the past, but without VSync,
            // especially for simpler shaders, you can easily hit thousands
            // of frames per second, stressing GPUs for no reason.
            surface_config.present_mode = wgpu::PresentMode::AutoVsync;

            surface.configure(device, &surface_config);

            (surface, surface_config)
        };
    let mut surface_with_config = auto_configure_surface(&adapter, &device, initial_surface, window.inner_size());

    // * Create pipeline layout from the shaders on disk

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[wgpu::PushConstantRange {
            stages: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            range: 0..std::mem::size_of::<ShaderConstants>() as u32,
        }],
    });

    // * Create the render pipeline

    let mut render_pipeline = create_pipeline(
        &options,
        &device,
        &pipeline_layout,
        surface_with_config.1.format,
        compiled_shader_modules,
    );

    // * Start main loop

    let start = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        let _ = (&instance, &adapter, &pipeline_layout);
        let render_pipeline = &mut render_pipeline;

        // * Handle events
        *control_flow = ControlFlow::Wait;
        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            // * Resize window
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                if size.width != 0 && size.height != 0 {
                    // Recreate the swap chain with the new size
                    let (surface, surface_config) = &mut surface_with_config;
                    surface_config.width = size.width;
                    surface_config.height = size.height;
                    surface.configure(&device, surface_config);
                }
            }

            // * Redraw window
            Event::RedrawRequested(_) => {
                // FIXME(eddyb) only the mouse shader *really* needs this, could
                // avoid doing wasteful rendering by special-casing each shader?
                // (with VSync enabled this can't be *too* bad, thankfully)
                // FIXME(eddyb) is this the best way to do continuous redraws in
                // `winit`? (or should we stop using `ControlFlow::Wait`? etc.)
                window.request_redraw();

                let (surface, surface_config) = &mut surface_with_config;
                let output = match surface.get_current_texture() {
                    Ok(surface) => surface,
                    Err(err) => {
                        eprintln!("get_current_texture error: {err:?}");
                        match err {
                            wgpu::SurfaceError::Lost => {
                                surface.configure(&device, surface_config);
                            }
                            wgpu::SurfaceError::OutOfMemory => {
                                *control_flow = ControlFlow::Exit;
                            }
                            _ => (),
                        }
                        return;
                    }
                };
                let output_view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &output_view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });

                    let time = start.elapsed().as_secs_f32();

                    let push_constants = ShaderConstants {
                        width: window.inner_size().width,
                        height: window.inner_size().height,
                        time,
                    };

                    rpass.set_pipeline(render_pipeline);
                    rpass.set_push_constants(
                        wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        0,
                        bytemuck::bytes_of(&push_constants),
                    );
                    rpass.draw(0..3, 0..1);
                }

                queue.submit(Some(encoder.finish()));
                output.present();
            }

            // * Close window on escape
            Event::WindowEvent {
                event:
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,

            // * F11 Fullscreen toggle
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::F11),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if window.fullscreen().is_some() {
                    window.set_fullscreen(None);
                } else {
                    window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
                }
            }

            // * Shader hot-reloading?
            Event::UserEvent(new_module) => {
                *render_pipeline = create_pipeline(
                    &options,
                    &device,
                    &pipeline_layout,
                    surface_with_config.1.format,
                    new_module,
                );
                window.request_redraw();
                *control_flow = ControlFlow::Poll;
            }
            
            // * Ignore other events
            _ => {}
        }
    });
}

// * Create the render pipeline

fn create_pipeline(
    options: &Options,
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    surface_format: wgpu::TextureFormat,
    compiled_shader_modules: CompiledShaderModules,
) -> wgpu::RenderPipeline {
    // FIXME(eddyb) automate this decision by default.
    let create_module = |module| {
        if options.force_spirv_passthru {
            unsafe { device.create_shader_module_spirv(&module) }
        } else {
            let wgpu::ShaderModuleDescriptorSpirV { label, source } = module;
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label,
                source: wgpu::ShaderSource::SpirV(source),
            })
        }
    };

    let vs_entry_point = shaders::main_vs;
    let fs_entry_point = shaders::main_fs;

    let vs_module_descr = compiled_shader_modules.spv_module_for_entry_point(vs_entry_point);
    let fs_module_descr = compiled_shader_modules.spv_module_for_entry_point(fs_entry_point);

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
        layout: Some(pipeline_layout),
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
                format: surface_format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    })
}

#[allow(clippy::match_wild_err_arm)]
pub fn start(
    options: &Options,
) {
    let mut event_loop_builder = EventLoopBuilder::with_user_event();
    env_logger::init();
    let event_loop = event_loop_builder.build();

    // Build the shader before we pop open a window, since it might take a while.
    let initial_shader = maybe_watch(
        options,
        {
            let proxy = event_loop.create_proxy();
            Some(Box::new(move |res| match proxy.send_event(res) {
                Ok(it) => it,
                // ShaderModuleDescriptor is not `Debug`, so can't use unwrap/expect
                Err(_err) => panic!("Event loop dead"),
            }))
        },
    );

    let window = winit::window::WindowBuilder::new()
        .with_title("Rust GPU - wgpu")
        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)
        .unwrap();
    futures::executor::block_on(run(
        options.clone(),
        event_loop,
        window,
        initial_shader,
    ));
}