mod app;
mod chip8;
mod opcode;

use chip8::Chip8;
use log::error;
use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    env_logger::init();

    let path = env::args().nth(1).expect("expected: <program> <ROM>");
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut chip8 = Chip8::default();
    chip8.load(&buffer);
    chip8.ambiguous(false, false);

    app::init(chip8);
    Ok(())
}

pub fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    error!("  Caused by: {:?}", err.source());
}
