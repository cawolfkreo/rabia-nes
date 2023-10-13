use std::str::FromStr;
use num_traits::Num;

const INSTRUCTIONS: &'static str = "lda #42\nsta $0015\nnotAnInstruction\nldx #23\nldy #69\nlda 0\nlda $FF";

const ZERO_FLAG: u8 = 1 << 1;

const NEGATIVE_FLAG: u8 = 1 << 6;

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

    let insturction_lines = INSTRUCTIONS.split("\n");
    for line in insturction_lines {
        let mut iter = line.split(" ");
        let instruction = iter.next();
        let arg = iter.next();

        match instruction {
            Some(inst) => {
                let arg = if arg.is_none() { "" } else { arg.unwrap() };
                execute_instruction(inst, arg, &mut cpu, &mut ram);
            }
            None => continue,
        }
    }
}


fn execute_instruction(instruction: &str, arguments: &str, cpu: &mut Cpu, ram: &mut[u8]) {

    match instruction {
        "lda" => {
            cpu.a_register = parse_from_str(arguments);
            let a = cpu.a_register;
            println!("The a register is: {a}");
            set_zero_flag(cpu.a_register, cpu);
            set_negative_flag(cpu.a_register, cpu);
        },
        "sta" => {
            let index: usize = parse_from_str(arguments);

            ram[index] = cpu.a_register;

            let memory_peaked = ram[index];
            println!("The ram at index {index} is: {memory_peaked}")
        },
        "ldx" => {
            cpu.x_register = parse_from_str(arguments);
            let x = cpu.x_register;
            println!("The x register is: {x}");
        },
        "ldy" => {
            cpu.y_register = parse_from_str(arguments);
            let y = cpu.y_register;
            println!("The y register is: {y}");
        }
        _ => println!("This isn't a instruction!!"),
    }

}


fn parse_from_str<T>(num_in_str: &str) -> T where T: Num + FromStr {
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

fn set_zero_flag(register: u8, cpu: &mut Cpu) {
    if register == 0 {
        cpu.p_register |= ZERO_FLAG;
    } else {
        cpu.p_register &= !ZERO_FLAG;
    }
}

fn set_negative_flag(register: u8, cpu: &mut Cpu) {
    let x = cpu.p_register;
    println!("cpu.p_register pre is {x:08b}");
    let mask_proc = register & NEGATIVE_FLAG;
    let or_equals = cpu.p_register | mask_proc;
    cpu.p_register = or_equals;
    let x = cpu.p_register;
    println!("cpu.p_register pos is {x:08b}");
}
