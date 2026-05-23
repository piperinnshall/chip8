#![forbid(unsafe_code)]

mod app;

use log::error;
use winit::event_loop::{ControlFlow, EventLoop};

pub const WIDTH: u32 = 320;
pub const HEIGHT: u32 = 240;

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
