use std::process;

use rusty_nes;

fn main() {

    let emulation_result = rusty_nes::run();
    
    if let Err(e) = emulation_result {
        eprintln!("Emulation error triggered!!");
        eprintln!("The error encountered was: {e}");
        process::exit(1);
    }
}
