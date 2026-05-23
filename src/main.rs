#![forbid(unsafe_code)]
use std::sync::Arc;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, DeviceId, ElementState, KeyEvent, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut App::default()).unwrap();
}

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let window_attributes = Window::default_attributes()
            .with_title("Chip8")
            .with_inner_size(size)
            .with_min_inner_size(size);
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let (window_width, window_height) = window.inner_size().into();
        let surface_texture = SurfaceTexture::new(window_width, window_height, window.clone());
        self.window = Some(window.clone());
        self.pixels = {
            match Pixels::new(WIDTH, HEIGHT, surface_texture) {
                Ok(pixels) => {
                    window.request_redraw();
                    Some(pixels)
                }
                Err(err) => {
                    log_error("Pixels::new", err);
                    event_loop.exit();
                    None
                }
            }
        };
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => match event {
                KeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    state: ElementState::Pressed,
                    repeat: false,
                    ..
                } => {
                    println!("The 'Escape' key was pressed; stopping");
                    event_loop.exit();
                }
                KeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::KeyW),
                    state: ElementState::Pressed,
                    repeat: false,
                    ..
                } => println!("The 'W' key was pressed"),
                _ => (),
            },
            _ => (),
        }
    }
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    error!("  Caused by: {:?}", err.source());
}
