use std::borrow::Cow;

use gravylib_helpers::Constants;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::Shader;

fn build_pipeline(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, shader: &Shader) -> wgpu::RenderPipeline {

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[wgpu::PushConstantRange {
            stages: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            range: 0..std::mem::size_of::<Constants>() as u32,
        }],
    });

    fn load_shader(device: &wgpu::Device, path: &str) -> wgpu::ShaderModule {
        let spirv = &std::fs::read(path).unwrap_or_else(|_| panic!("Failed to read shader at {}!", path));
        let spirv = Cow::Owned(wgpu::util::make_spirv_raw(spirv).into_owned());
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::SpirV(spirv),
        })
    }

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&layout),
        // TODO: Don't hardcode `pixel_vs`, parse `shader_type` instead.
        vertex: wgpu::VertexState {
            module: &load_shader(device, env!("gravylib_helpers.spv")),
            entry_point: "pixel_vs",
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
        // TODO: Figure out how to render to an `ImageBuffer` instead of a `Surface`
        fragment: Some(wgpu::FragmentState {
            module: &load_shader(device, &shader.path),
            entry_point: &shader.entry_point,
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    })
}

// ** Program state

// TODO: Generalize this to fit with a `RenderGraphBuilder` structure for the external interface
// ?? How should we chunk this up?
#[allow(dead_code)]
struct State {
    instance: wgpu::Instance,
    // TODO: Figure out how to use `ImageBuffer`s as shader inputs and outputs
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    shader: Shader,
    // TODO: Research `wgpu` render pipelines to determine how to generalize them
    pipeline: wgpu::RenderPipeline,
}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new(window: Window, shader: Shader) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::util::backend_bits_from_env()
                .unwrap_or(wgpu::Backends::VULKAN | wgpu::Backends::METAL | wgpu::Backends::DX12),
            dx12_shader_compiler: wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default(),
        });
        
        let surface = unsafe { instance.create_surface(&window) }
            .expect("Failed to create surface from window!");

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await
        .expect("Failed to find an appropriate adapter!");
    
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
            .expect("Failed to create device!");

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);

        let mut config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        // NOTE(eddyb) VSync was disabled in the past, but without VSync,
        // especially for simpler shaders, you can easily hit thousands
        // of frames per second, stressing GPUs for no reason.
        config.present_mode = wgpu::PresentMode::AutoVsync;

        surface.configure(&device, &config);

        let pipeline = build_pipeline(&device, &config, &shader);

        Self {
            instance,
            window,
            surface,
            adapter,
            device,
            queue,
            config,
            size,
            shader,
            pipeline
        }
    }

    fn window(&self) -> &Window {
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
            _ => false, // TODO: Add input handling
        }
    }

    fn render(&mut self, time: f32) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        { // ?? Why is this scoped?
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

            let push_constants = Constants {
                width: self.window().inner_size().width,
                height: self.window().inner_size().height,
                time,
                gravylib: [0, 1, 0],
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
}

// ** Run the main loop

pub(crate) async fn run(
    event_loop: EventLoop<()>,
    window: Window,
    shader: Shader
) {
    // ** Create the state

    let mut state = State::new(window, shader).await;

    // ** Start main loop

    let start = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        // ?? Is this still needed?
        let _ = &state;

        // ** Handle events
        match event {
            Event::MainEventsCleared =>
                state.window().request_redraw(),

            // ** Handle window events
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => if !state.input(event) {
                match event {

                    // ** Close window
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

                    // ** Toggle fullscreen
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

                    // ** Resize window
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }

                    // ** Ignore other window events
                    _ => {}
                }
            },

            // ** Redraw window
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
                    }
                },
            
            // ** Ignore other events
            _ => {}
        }
    });
}