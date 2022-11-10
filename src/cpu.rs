pub struct Chip8 {
    pub memory: [u8; 4096],
    pub program_counter: usize,
    pub registers: [u8; 16],
    pub stack: [u16; 16],
    pub stack_pointer: usize,
}

impl Chip8 {
    pub fn run(&mut self) {
        loop {
            let op_byte1 = self.memory[self.program_counter] as u16;
            let op_byte2 = self.memory[self.program_counter + 1] as u16;
            let opcode: u16 = op_byte1 << 8 | op_byte2;

            // ##### xykk #####
            // let x = ((opcode & 0xF000) >>  8) as u8;
            // let y = ((opcode & 0x0F00) >>  4) as u8;
            // let kk = (opcode & 0x00FF) as u8;
            // let nnn = opcode & 0x0FFF;

            self.program_counter += 2;

            match opcode {
                0x0000 => { return; },
                0x00E0 => { /* clear screen */ },
                // RETURN from subroutine
                0x00EE => { self.return(); },
                // JUMP to address NNN
                0x1FFF => { 
                    self.jump(
                        opcode & 0x0FFF // nnn
                    ); 
                },
                // CALL subroutine starting at address 'nnn'
                0x2FFF => { 
                    self.call(
                        opcode & 0x0FFF // nnn
                    ); 
                },
                // SKIP the following instruction if the value of register[VX] is EQUAL to 'kk'
                0x3FFF => {
                    self.skip_if_equal(
                        ((opcode & 0xF000) >> 8) as u8, // x
                        (opcode & 0x00FF) as u8 // kk
                    ); 
                },
                // SKIP the following instruction if the value of rVX is NOT EQUAL NN
                0x4FFF => { 
                    self.skip_if_not_equal(
                        ((opcode & 0xF000) >>  8) as u8, // x
                        (opcode & 0x00FF) as u8 // kk
                    );
                },
                // SKIP the following instruction if the value of rVX is EQUAL to NN
                0x5FFF => {
                    self.skip_if_equal(
                        ((opcode & 0xF000) >>  8) as u8, // x
                        ((opcode & 0x0F00) >>  4) as u8 // y
                    );
                },
                // store (LOAD) NN in register VX
                0x6FFF => {
                    self.load(
                        ((opcode & 0xF000) >>  8) as u8, // x
                        (opcode & 0x00FF) as u8 // kk
                    );
                },
                0x7FFF => {
                    self.add(
                        ((opcode & 0xF000) >>  8) as u8, // x
                        (opcode & 0x00FF) as u8 // kk
                    );
                },
                0x8FFF => {
                    match (opcode & 0x000F) as u8 {
                        0 => {
                            self.load(
                                ((opcode & 0xF000) >>  8) as u8, // x
                                self.registers[((opcode & 0x0F00) >>  4) as usize]) 
                        },
                        1 => { 
                            self.or_xy(
                                ((opcode & 0xF000) >>  8) as u8, // x
                                ((opcode & 0x0F00) >>  4) as u8, // y
                            ) 
                        },
                        2 => {
                            self.and_xy(
                                ((opcode & 0xF000) >>  8) as u8, // x
                                ((opcode & 0x0F00) >>  4) as u8, // y
                            ) 
                        },
                        3 => {
                            self.xor_xy(
                                ((opcode & 0xF000) >>  8) as u8, // x
                                ((opcode & 0x0F00) >>  4) as u8, // y
                            ) 
                        },
                        4 => { 
                            self.add_xy(
                                ((opcode & 0xF000) >>  8) as u8, // x
                                ((opcode & 0x0F00) >>  4) as u8, // y
                            ) 
                        },
                        _ => { todo!("opcode: {:04x}", opcode); },
                    }
                },
                _ => { todo!("opcode: {:04x}", opcode); },
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("stack overflow!");
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.program_counter = addr as usize;
    }

    fn read_opcode(&self) -> u16 {
        let p = self.program_counter;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }
}
