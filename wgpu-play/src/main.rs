use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};


struct SimulApp<'app> {
    window: Window,
    surface: wgpu::Surface<'app>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

impl<'app> SimulApp<'app> {
    fn new() -> SimulApp<'app> {
        todo!();
        //Self { window: None, surface: None }  
    }
}

impl<'app> ApplicationHandler for SimulApp<'app> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = event_loop.create_window(Window::default_attributes()).unwrap();
    }

    fn window_event( &mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {

            WindowEvent::CloseRequested => {
                event_loop.exit(); 
            },
            WindowEvent::RedrawRequested => {
                self.window.request_redraw();
            },
            WindowEvent::Resized(new_size) => {
                println!("Resized! Width: {}, Height: {}", new_size.width, new_size.height);

            },
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = SimulApp::new();
    event_loop.run_app(&mut app).expect("Error in the event loop");
}
