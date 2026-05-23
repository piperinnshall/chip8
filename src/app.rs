use crate::Chip8;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    chip8: Chip8,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = LogicalSize::new(crate::WIDTH as f64, crate::HEIGHT as f64);
        let window_attributes = Window::default_attributes()
            .with_title("Chip8")
            .with_inner_size(size)
            .with_min_inner_size(size);
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let (window_width, window_height) = window.inner_size().into();
        let surface_texture = SurfaceTexture::new(window_width, window_height, window.clone());

        match Pixels::new(crate::WIDTH, crate::HEIGHT, surface_texture) {
            Ok(pixels) => {
                // window.request_redraw();
                self.window = Some(window.clone());
                self.pixels = Some(pixels);
            }
            Err(err) => {
                crate::log_error("Pixels::new", err);
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Err(err) = self
                    .pixels
                    .as_mut()
                    .unwrap()
                    .resize_surface(size.width, size.height)
                {
                    crate::log_error("pixels.resize_surface", err);
                    event_loop.exit()
                }
            }
            WindowEvent::RedrawRequested => {
                self.chip8.update();
                self.chip8.draw(self.pixels.as_mut().unwrap().frame_mut());
                if let Err(err) = self.pixels.as_ref().unwrap().render() {
                    crate::log_error("pixels.render", err);
                    event_loop.exit();
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => match event {
                KeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    state: ElementState::Pressed,
                    repeat: false,
                    ..
                } => {
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
