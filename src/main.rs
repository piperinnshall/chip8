mod app;
mod chip8;

use log::error;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

fn main() {
    let chip8 = chip8::Chip8::default();
    env_logger::init();
    app::init(chip8);
}



pub fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    error!("  Caused by: {:?}", err.source());
}

