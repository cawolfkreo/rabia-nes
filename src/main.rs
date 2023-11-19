mod cpu;
use cpu::Cpu;

const INSTRUCTIONS: &'static str =
    "lda #42\nsta $0015\nnotAnInstruction\nldx #23\nldy #69\nlda 0\nlda $FF\nnop";

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
