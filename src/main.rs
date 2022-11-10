pub mod cpu;

use cpu::CPU;

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        program_counter: 0,
        memory: [0; 4096],
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    // load 6 opcodes into 12 registers: 
    // 0x2100, 0x2100, 0x0000, 0x8014, 0x8014, 0x00EE
    //   |/\|    |/\|    |/\|    |/\|    |/\|    |/\|
    //   |  |    |  |    |  |    |  |    |  |    |  |
    //  [0][1]  [2][3]  [4][5]  [6][7]  [8][9] [10][11] 
    mem[0x000] = 0x21; mem[0x001] = 0x00;
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    mem[0x004] = 0x00; mem[0x005] = 0x00;

    mem[0x100] = 0x80; mem[0x101] = 0x14;
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    mem[0x104] = 0x00; mem[0x105] = 0xEE;


    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
