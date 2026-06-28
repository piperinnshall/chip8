mod app;
mod chip8;
mod opcode;

use app::App;
use chip8::Chip8;
use log::error;
use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    env_logger::init();

    let path = env::args().nth(1).expect("expected: <program> <ROM>");
    let mut file = File::open(path)?;
    let mut rom = Vec::new();
    file.read_to_end(&mut rom)?;

    let mut chip8 = Chip8::new(&rom);
    chip8.ambiguous(false, false);

    let mut app = App::new(chip8);
    app.debug(false);
    app.run();

    Ok(())
}

pub fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    error!("  Caused by: {:?}", err.source());
}
