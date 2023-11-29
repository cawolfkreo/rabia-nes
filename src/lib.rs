use std::error::Error;

mod cpu;
mod controller;

use cpu::Cpu;
use crate::controller::{Controller, ControllerState};

const INSTRUCTIONS: &str =
    "lda #42\nsta $0015\nnotAnInstruction\nldx #23\nldy #69\nlda 0\nlda $FF\nnop";

pub fn run() -> Result<(), Box<dyn Error>> {
    //The ram should be it's own structure to be honest. I'll keep it like this for now
    let mut ram: [u8; 2048] = [0; 2048];

    let mut cpu = Cpu::new();

    for line in INSTRUCTIONS.lines() {
        let mut iter = line.split(' ');
        let instruction_split = iter.next();
        let arg_split = iter.next();

        if let Some(instruction) = instruction_split {
            let arguments = arg_split.unwrap_or("");
            cpu.execute_instruction(instruction, arguments, &mut ram);
        }
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
