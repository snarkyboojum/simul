mod point;

use winit::{
    error::OsError, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowBuilder}
};

use ash::{vk, Entry, Instance };

struct VkApp<'a> {
    name: &'a str,
    _entry: ash::Entry,
    instance: ash::Instance,

}

impl VkApp<'_> {
    pub fn new(app_name: &str) -> VkApp {
        // TODO: get the linked version of ash working - will this allow us to remove the unsafe block?
        let entry = unsafe { Entry::load().unwrap() };

        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            ..Default::default()
        };

        // TODO: can I get rid of this unsafe block as well?
        let instance = unsafe { entry.create_instance(&create_info, None).unwrap() };

        VkApp { 
            name: app_name,
            _entry: entry,
            instance: instance,
        }
    }

    fn init_window(&self, event_loop: &EventLoop<()>) -> winit::window::Window {

        let window = WindowBuilder::new().build(event_loop).unwrap();

        return window;
    }

    fn main_loop(&self, event_loop: EventLoop<()>, window: Window) {

        event_loop.set_control_flow(ControlFlow::Poll);
        let _ = event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent { 
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The window close button was pressed; exiting");
                    elwt.exit();
                },
                _ => ()
            }
        });
    }

    fn cleanup(&self) {

    }

}

fn main() -> Result<(), OsError> {
    println!("This package implements multi-view geometry.");

    let app = VkApp::new("Point cloud renderer");
    let event_loop = EventLoop::new().unwrap();

    let window = app.init_window(&event_loop);
    app.main_loop(event_loop, window);

    Ok(())

}
