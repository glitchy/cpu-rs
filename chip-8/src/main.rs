pub mod cpu;

use cpu::Chip8;

fn main() {
    let mut cpu = Chip8 {
        memory: [0; 4096],
        program_counter: 0,
        registers: [0; 16],
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    // load 6 opcodes into 12 registers: 
    // 0x2100, 0x2100, 0x0000, 0x8014, 0x8014, 0x00EE
    //   |/\|    |/\|    |/\|    |/\|    |/\|    |/\|
    //   |  |    |  |    |  |    |  |    |  |    |  |
    //  [0][1]  [2][3]  [4][5]  [6][7]  [8][9] [10][11] 
    //
    // ##### opcode definitions #####
    // 0x2100 = CALL fn at 0x100
    cpu.memory[0x000] = 0x21; cpu.memory[0x001] = 0x00; 
    // 0x2100 = CALL fn at 0x100
    cpu.memory[0x002] = 0x21; cpu.memory[0x003] = 0x00;
    // 0x8014 = ADD r1 to r0
    cpu.memory[0x100] = 0x80; cpu.memory[0x101] = 0x14; 
    // 0x8014 = ADD r1 to r0
    cpu.memory[0x102] = 0x80; cpu.memory[0x103] = 0x14;
    // 0x00EE = RETURN
    cpu.memory[0x104] = 0x00; cpu.memory[0x105] = 0xEE;


    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
    }
