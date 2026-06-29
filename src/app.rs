use crate::chip8::{self, Chip8};
use pixels::{Pixels, SurfaceTexture};
use std::cmp;
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, KeyEvent, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

const FPS: u64 = 60;
const UPS: u64 = 700;
const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000 / FPS);
const UPDATE_TIME: Duration = Duration::from_nanos(1_000_000_000 / UPS);

pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    update_target: Instant,
    render_target: Instant,
    check: Instant,
    debug: bool,
    frame: i32,
    update: i32,
    chip8: Chip8,
}

impl App {
    pub fn new(chip8: Chip8) -> Self {
        Self {
            chip8,
            ..Default::default()
        }
    }

    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self).unwrap();
    }

    fn scancode(keycode: KeyCode) -> Option<usize> {
        match keycode {
            KeyCode::Digit1 => Some(0),
            KeyCode::Digit2 => Some(1),
            KeyCode::Digit3 => Some(2),
            KeyCode::Digit4 => Some(3),
            KeyCode::KeyQ => Some(4),
            KeyCode::KeyW => Some(5),
            KeyCode::KeyE => Some(6),
            KeyCode::KeyR => Some(7),
            KeyCode::KeyA => Some(8),
            KeyCode::KeyS => Some(9),
            KeyCode::KeyD => Some(10),
            KeyCode::KeyF => Some(11),
            KeyCode::KeyZ => Some(12),
            KeyCode::KeyX => Some(13),
            KeyCode::KeyC => Some(14),
            KeyCode::KeyV => Some(15),
            _ => None,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
            update_target: Instant::now(),
            render_target: Instant::now(),
            check: Instant::now(),
            debug: false,
            frame: 0,
            update: 0,
            chip8: Chip8::default(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = LogicalSize::new(chip8::WIDTH as f64, chip8::HEIGHT as f64);
        let window_attributes = Window::default_attributes()
            .with_title("Chip8")
            .with_inner_size(size)
            .with_min_inner_size(size);
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let (window_width, window_height) = window.inner_size().into();
        let surface_texture = SurfaceTexture::new(window_width, window_height, window.clone());
        match Pixels::new(chip8::WIDTH as u32, chip8::HEIGHT as u32, surface_texture) {
            Ok(pixels) => {
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
                let now = Instant::now();
                if self.update_target <= now {
                    self.update += 1;

                    // let start = Instant::now();
                    // self.chip8.fetch_decode_execute();
                    // println!("Fetch, Decode, Execute{:?}", start.elapsed());

                    self.update_target += UPDATE_TIME;
                    self.window.as_ref().map(|window| window.request_redraw());
                }
                if self.render_target <= now {
                    self.frame += 1;
                    // self.chip8.draw(self.pixels.as_mut().unwrap().frame_mut());
                    if let Err(err) = self.pixels.as_ref().unwrap().render() {
                        crate::log_error("pixels.render", err);
                        event_loop.exit();
                    }
                    self.render_target += FRAME_TIME;
                    self.window.as_ref().map(|window| window.request_redraw());
                }
                if now - self.check >= Duration::from_secs(1) {
                    if self.debug {
                        println!("UPS: {:?}, FPS: {:?}", self.update, self.frame);
                    }
                    self.update = 0;
                    self.frame = 0;
                    self.check = now;
                }
            }
            WindowEvent::KeyboardInput { event, .. } => match event {
                KeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    state: ElementState::Pressed,
                    repeat: false,
                    ..
                } => event_loop.exit(),
                KeyEvent {
                    physical_key: PhysicalKey::Code(keycode),
                    state,
                    repeat: false,
                    ..
                } => match Self::scancode(keycode) {
                    Some(key) => match state {
                        ElementState::Pressed => self.chip8.key_down(key),
                        ElementState::Released => self.chip8.key_up(key),
                    },
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }

    fn new_events(&mut self, _: &ActiveEventLoop, _: StartCause) {
        if self.render_target <= Instant::now() || self.update_target <= Instant::now() {
            self.window.as_ref().map(|window| window.request_redraw());
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::WaitUntil(cmp::min(
            self.render_target,
            self.update_target,
        )));
    }
}
