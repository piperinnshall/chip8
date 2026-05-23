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

const MEMORY_SIZE: usize = 0x1000;
const MEMORY_START: usize = 0x200;

pub struct Chip8 {
    display: BitVec,
    memory: [u16; MEMORY_SIZE],
}

impl Chip8 {
    fn update(&mut self) {}
    fn draw(&self, frame: &mut [u8]) {
        frame
            .chunks_exact_mut(4)
            .zip(self.display.iter())
            .for_each(|(pixel, bit)| {
                pixel.copy_from_slice(&if bit {
                    [0x5e, 0x48, 0xe8, 0xff]
                } else {
                    [0x48, 0xb2, 0xe8, 0xff]
                })
            });
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        let display = BitVec::from_fn((WIDTH * HEIGHT) as usize, |i| {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;
            (x + y) % 2 == 0
        });
        let mut memory = [0; MEMORY_SIZE];
        memory[0x050..=0x09F].copy_from_slice(&[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ]);
        Self { display, memory }
    }
}
