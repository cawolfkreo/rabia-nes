use std::error::Error;
use std::fs;

mod cpu;
mod controller;

use cpu::Cpu;
use controller::{Controller, ControllerState}; //TODO: we should not bring the enum into scope like this, but we can leave it for now while we add more functionality

const INSTRUCTIONS: &str =
    "lda #42\nsta $0015\nnotAnInstruction\nldx #23\nldy #69\nlda 0\nlda $FF\nnop";

#[derive(Default)]
pub struct Config {
    pub rom_path: Option<String>,
}

impl Config {
    pub fn new(rom_path: Option<String>) -> Self{
        Self {
            rom_path,
            ..Default::default()
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //The ram should be it's own structure to be honest. I'll keep it like this for now
    let mut ram: [u8; 2048] = [0; 2048];

    let mut cpu = Cpu::new();

    let instructions = get_instructions(&config.rom_path);

    for line in instructions.lines() {
        let mut iter = line.split(' ');
        
        let Some(instruction) = iter.next() else {
            return Err("couldn't retrieve an instruction from the line!".into());
        };

        let arguments = iter.next().unwrap_or("");
        cpu.execute_instruction(instruction, arguments, &mut ram);
    }

    let mut controller1 = Controller::new();
    controller1.set_controller_flag(controller::BUTTON_A, ControllerState::On);
    controller1.set_controller_flag(controller::BUTTON_B, ControllerState::Off);
    controller1.set_controller_flag(controller::BUTTON_SELECT, ControllerState::On);
    controller1.set_controller_flag(controller::BUTTON_START, ControllerState::On);
    controller1.set_controller_flag(controller::BUTTON_UP, ControllerState::On);
    controller1.set_controller_flag(controller::BUTTON_DOWN, ControllerState::On);
    controller1.set_controller_flag(controller::BUTTON_LEFT, ControllerState::On);
    controller1.set_controller_flag(controller::BUTTON_RIGHT, ControllerState::On);
    println!("Controller state is {:08b}", controller1.get_controller_state());

    Ok(())
}

fn get_instructions(rom_path: &Option<String>) -> String {
    if let Some(ref path) = rom_path {
        println!("Reading from file {path}");

        if let Ok(file_contents) = read_rom_file(path) {
           return file_contents;
        }

        println!("we couldn't read from file.");

    }

    println!("Using internal instruction test");

    INSTRUCTIONS.to_string()
}

pub fn read_rom_file(path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}
