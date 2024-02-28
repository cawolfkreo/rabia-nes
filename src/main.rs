use std::env;
use std::process;

use rabia_nes::Config;

fn main() {
    let config: Config = create_config();

    let emulation_result = rabia_nes::run(config);

    if let Err(e) = emulation_result {
        eprintln!("Emulation error triggered!!");
        eprintln!("The error encountered was: {e}");
        process::exit(1);
    }
}

fn create_config() -> Config {
    let args: Vec<String> = env::args().collect();

    let rom_path = args.get(1).map(|path| path.to_string());

    Config::new(rom_path)
}
