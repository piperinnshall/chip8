mod app;
mod chip8;
mod opcode;

use app::App;
use chip8::Chip8;

fn main() -> std::io::Result<()> {
    env_logger::init();
    let path = std::env::args().nth(1).expect("expected: <program> <ROM>");
    let rom = std::fs::read(path)?;
    let chip8 = Chip8::new(&rom).with_mode(false, false);
    App::new(chip8).with_debug(false).run();
    Ok(())
}

pub fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    log::error!("{method_name}() failed: {err}");
    log::error!("  Caused by: {:?}", err.source());
}
