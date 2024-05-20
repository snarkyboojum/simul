use std::default;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};


struct SimulApp<'app> {
    window: Option<Window>,
    surface: Option<wgpu::Surface<'app>>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    surface_config: Option<wgpu::SurfaceConfiguration>,
    size: winit::dpi::PhysicalSize<u32>,
}

impl<'app> SimulApp<'app> {

    // ensure new() work without needing a handle on window etc.
    fn new() -> Self {
        Self {
            window: None,
            surface: None,
            device: None,
            queue: None,
            surface_config: None,
            size: Default::default()

        }
    }

    // initialise the rest of the Simul if we have a window 
    async fn init(&mut self) {

        if let Some(window) = &self.window {
            let size = window.inner_size();

            // instance is the backend, e.g. Vulkan
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::PRIMARY,
                ..Default::default()
            });
        
            // surface is created from the instance, and targets the window?
            let surface = unsafe { instance.create_surface(&window)}.unwrap();

            let adapter = instance.request_adapter(
                &wgpu::RequestAdapterOptions {
                    compatible_surface: Some(&surface),
                    power_preference: wgpu::PowerPreference::default(),
                    force_fallback_adapter: false,
                },
            ).await.unwrap();

            let (device, queue) = adapter.request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),    // this won't work for web, as WebGL doesn't support all of wgpu's features
                    label: None,
                },
                None,   // Trace path
            ).await.unwrap();

            let surface_caps = surface.get_capabilities(&adapter);

            let surface_format = surface_caps.formats.iter()
                .copied()
                .filter(|f| f.is_srgb())
                .next()
                .unwrap_or(surface_caps.formats[0]);

            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width: size.width,
                height: size.height,
                present_mode: surface_caps.present_modes[0],
                alpha_mode: surface_caps.alpha_modes[0],
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };

            println!("Graphics device: {:?}", device);
        }
    }

    
    fn window(&self) -> &Window {
        todo!()
        //&self.window.unwrap()
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!()
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}


impl<'app> ApplicationHandler for SimulApp<'app> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // create a window if it doesn't already exist
        if self.window.is_none() {
            let window = event_loop.create_window(Window::default_attributes()).unwrap();
            self.window = Some(window);

            pollster::block_on(self.init());
        }
    }

    fn window_event( &mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit(); 
            },
            WindowEvent::RedrawRequested => {
                //self.window().request_redraw();
            },
            WindowEvent::Resized(new_size) => {
                //println!("Resized! Width: {}, Height: {}", new_size.width, new_size.height);
            },
            _ => (),
        }
    }
}

async fn run() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = SimulApp::new();
    event_loop.run_app(&mut app).expect("Application ran successfully with the event loop");
}

fn main() {
    pollster::block_on(run());
}
