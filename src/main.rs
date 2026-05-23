#![forbid(unsafe_code)]

mod app;

use bit_vec::BitVec;
use log::error;
use winit::event_loop::{ControlFlow, EventLoop};

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app::App::default()).unwrap();
}

pub fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    error!("  Caused by: {:?}", err.source());
}

#[derive(Default)]
pub struct Chip8 {
    display: BitVec,
}
impl Chip8 {
    fn update(&mut self) {
    }
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let rgba = if (x+y) % 2 == 0 {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
