use std::env;
use std::process;

use rusty_nes::Config;

fn main() {
    let config: Config = create_config();

    let emulation_result = rusty_nes::run(config);

    if let Err(e) = emulation_result {
        eprintln!("Emulation error triggered!!");
        eprintln!("The error encountered was: {e}");
        process::exit(1);
    }
}

fn create_config() -> Config {
    let args: Vec<String> = env::args().collect();

    let rom_path = match args.get(1) {
        Some(ref path) => Some(path.to_string()),
        None => None
    };

    Config::new(rom_path)
}