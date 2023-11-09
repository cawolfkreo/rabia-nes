use num_traits::Num;
use std::str::FromStr;

const INSTRUCTIONS: &'static str =
    "lda #42\nsta $0015\nnotAnInstruction\nldx #23\nldy #69\nlda 0\nlda $FF\nnop";

const CARRY_FLAG: u8 = 1;

const ZERO_FLAG: u8 = 1 << 1;

const NEGATIVE_FLAG: u8 = 1 << 7;

struct Cpu {
    a_register: u8,
    x_register: u8,
    y_register: u8,
    p_register: u8,
}

fn main() {
    //The ram should be it's own structure to be honest. I'll keep it like this for now
    let mut ram: [u8; 2048] = [0; 2048];

    let mut cpu = Cpu {
        a_register: 0,
        x_register: 0,
        y_register: 0,
        p_register: 0x34,
    };

    let instruction_lines = INSTRUCTIONS.split("\n");
    for line in instruction_lines {
        let mut iter = line.split(" ");
        let instruction = iter.next();
        let arg = iter.next();

        match instruction {
            Some(inst) => {
                let arg = if arg.is_none() { "" } else { arg.unwrap() };
                cpu.execute_instruction(inst, arg, &mut ram);
            }
            None => continue,
        }
    }
}

fn parse_from_str<T>(num_in_str: &str) -> T
where
    T: Num + FromStr,
{
    let wrapped = {
        if num_in_str.starts_with('$') {
            let prefixed_string = num_in_str.strip_prefix('$').unwrap();
            <T>::from_str_radix(prefixed_string, 16)
        } else if num_in_str.starts_with('#') {
            let prefixed_string = num_in_str.strip_prefix('#').unwrap();
            <T>::from_str_radix(prefixed_string, 10)
        } else {
            <T>::from_str_radix(num_in_str, 10)
        }
    };

    wrapped.ok().expect("I cannot parse that!!!")
}

impl Cpu {
    fn execute_instruction(&mut self, instruction: &str, arguments: &str, ram: &mut [u8]) {
        match instruction {
            "lda" => {
                self.a_register = parse_from_str(arguments);
                println!("The a register is: {}", self.a_register);
                self.set_zero_flag(self.a_register);
                self.set_negative_flag(self.a_register);
                self.clear_carry_flag();
                self.set_carry_flag();
            }
            "sta" => {
                let index: usize = parse_from_str(arguments);

                ram[index] = self.a_register;

                let memory_peaked = ram[index];
                println!("The ram at index {index} is: {memory_peaked}")
            }
            "ldx" => {
                self.x_register = parse_from_str(arguments);
                let x = self.x_register;
                println!("The x register is: {x}");
                self.set_zero_flag(self.a_register);
                self.set_negative_flag(self.a_register);
            }
            "ldy" => {
                self.y_register = parse_from_str(arguments);
                let y = self.y_register;
                println!("The y register is: {y}");
                self.set_zero_flag(self.a_register);
                self.set_negative_flag(self.a_register);
            }
            "nop" => (), //What did you expect? it's "no operation!!!"
            _ => println!("This isn't a instruction!!"),
        }
    }

    fn set_zero_flag(&mut self, register: u8) {
        if register == 0 {
            self.p_register |= ZERO_FLAG;
        } else {
            self.p_register &= !ZERO_FLAG;
        }
    }

    fn set_negative_flag(&mut self, register: u8) {
        //println!("input register ___ is {register:08b}");
        self.p_register |= register & NEGATIVE_FLAG;
    }

    fn set_carry_flag(&mut self) {
        self.p_register |= CARRY_FLAG;
    }

    fn clear_carry_flag(&mut self) {
        self.p_register &= !CARRY_FLAG;
    }
}
