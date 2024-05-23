
use std::default;

use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{WindowBuilder, Window}
};

struct Simul<'app> {
    surface: wgpu::Surface<'app>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
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

        Self {
            surface: surface,
            adapter: adapter,
            device: device,
            queue: queue,
        }
    }
}

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let app = Simul::new(&window).await;

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
            _ => ()
        }
    });
}