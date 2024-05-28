
use winit::{
    dpi::LogicalSize, event::{self, ElementState, Event, KeyEvent, WindowEvent}, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowBuilder}
};

use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};


#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],

}

const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];


struct Simul<'app> {
    surface: wgpu::Surface<'app>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    challenge: bool,
    window: &'app Window,

    render_pipeline: wgpu::RenderPipeline,
    challenge_render_pipeline: wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,
}

impl<'app> Simul<'app> {
    async fn new(window: &'app Window) -> Simul<'app> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        // TODO: should this be unsafe?
        let surface = instance.create_surface(window).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                // power_preference: wgpu::PowerPreference::HighPerformance(),
                power_preference: wgpu::PowerPreference::default(),
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None
        ).await.unwrap();

        // println!("Adapter features: {:?}", adapter.features());
        println!("Adapter info: {:?}", adapter.get_info());
        // println!("Size: ({}, {})", size.width, size.height);

        let surface_capability = surface.get_capabilities(&adapter);
        let surface_format = surface_capability.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capability.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            // present_mode: wgpu::PresentMode::AutoVsync,
            present_mode: surface_capability.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_capability.alpha_modes[0],
            view_formats: vec![]
        };

        // println!("The alpha mode being used is: {:?}", config.alpha_mode);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Standard shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let challenge_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Challenge shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("challenge_shader.wgsl").into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:  Some("Render pipeline layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: Default::default(),

            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            depth_stencil: None,
            multiview: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::default(), // Ccw is the default
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
                unclipped_depth: false,
            },
        });

        let challenge_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Challenge render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &challenge_shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: Default::default(),

            },
            fragment: Some(wgpu::FragmentState {
                module: &challenge_shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            depth_stencil: None,
            multiview: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::default(), // Ccw is the default
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
                unclipped_depth: false,
            },
        });


        Self {
            surface: surface,
            device: device,
            queue: queue,
            config: config,
            size: size,
            challenge: false,

            window: window,

            render_pipeline: render_pipeline,
            challenge_render_pipeline: challenge_render_pipeline,
            
            vertex_buffer: vertex_buffer,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;

            // println!("New width and height: ({},{})", self.config.width, self.config.height);
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {

        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::Space),
                    state: ElementState::Pressed,
                    ..
                },
                ..
             } => {
                if self.challenge {
                    self.challenge = false;
                }
                else {
                    self.challenge = true;
                }
                return true
            },
            _ => { return false }
        }
    }

    fn update(&mut self) {

    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        // copy pasta from the tutorial - https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/#render
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let color = wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            };

            let vertices = 0..3u32;

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });


            if self.challenge {
                render_pass.set_pipeline(&self.challenge_render_pipeline);
            }
            else {
                render_pass.set_pipeline(&self.render_pipeline);
            }
            render_pass.draw(vertices, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {

    // setup window
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
                        .with_title("Simul app")
                        .with_inner_size(LogicalSize::new(800,600))
                        .build(&event_loop)
                        .unwrap();

    // create app
    let mut app = Simul::new(&window).await;

    // respond to events
    let _ = event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            }
            // capture events in app.input() if we've matched a Event::WindowEvent
            if !app.input(event) => {
                match event {
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                        event: KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                        ..
                    } => elwt.exit(),
                    WindowEvent::Resized(physical_size) => {
                        app.resize(*physical_size);
                    },
                    WindowEvent::RedrawRequested => {
                        app.window.request_redraw();
                        app.update();

                        match app.render() {
                            Ok(_) => {},
                            Err(e) => eprintln!("{:?}", e),
                        }
                    },
                    _ => ()
                }
            }
        _ => (),
        }
    });
}