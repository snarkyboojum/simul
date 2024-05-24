
use std::default;

use wgpu::Surface;

use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent, InnerSizeWriter},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{WindowBuilder, Window},
    dpi::PhysicalSize,
};

struct Simul<'app> {
    surface: wgpu::Surface<'app>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    window: &'app Window,
    // adapter: wgpu::Adapter,
}

impl<'app> Simul<'app> {
    async fn new(window: &'app Window) -> Simul<'app> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                power_preference: wgpu::PowerPreference::HighPerformance,
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::default(),
                required_limits: wgpu::Limits::default(),
            },
            None
        ).await.unwrap();

        // println!("Adapter features: {:?}", adapter.features());
        println!("Adapter info: {:?}", adapter.get_info());

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
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_capability.alpha_modes[0],
            view_formats: vec![]
        };

        println!("The alpha mode being used is: {:?}", config.alpha_mode);

        Self {
            surface: surface,
            device: device,
            queue: queue,
            config: config,
            size: size,
            window: window
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        
        self.config.width = new_size.width;
        self.config.height = new_size.height;

        // println!("New width and height: ({},{})", self.config.width, self.config.height);
        self.surface.configure(&self.device, &self.config);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut app = Simul::new(&window).await;

    event_loop.set_control_flow(ControlFlow::Poll);

    let _ = event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                    event: KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                    ..
                },
                ..
            } => {
                println!("The close button or escape key has been pressed. Closing window.");
                elwt.exit();
            },
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                app.resize(physical_size);
            },
            _ => ()
        }
    });
}