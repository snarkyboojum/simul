use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::{ControlFlow, EventLoop, ActiveEventLoop}, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowId}
};

#[derive(Default)]
struct SimulApp {
    window: Option<Window>,
}

impl ApplicationHandler for SimulApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event( &mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {

            WindowEvent::CloseRequested => {
                event_loop.exit(); 
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            },
            WindowEvent::Resized(new_size) => {
                //println!("Resized! Width: {}, Height: {}", new_size.width, new_size.height);

            },
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = SimulApp::default();
    event_loop.run_app(&mut app);
}


/*use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => { elwt.exit(); }
            _ => {}
        },
        _ => {}
    });
}

fn main() {
    println!("Welcome to wgpu!");

    run();
}
*/
