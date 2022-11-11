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
                0x00EE => { self.ret(); },
                // JUMP to address NNN
                0x1FFF => { 
                    self.jump(
                        opcode & 0x0FFF // nnn
                    ); 
                },
                // CALL subroutine starting at address 'nnn'
                0x2000..=0x2FFF => { 
                    self.call(
                        opcode & 0x0FFF // nnn
                    ); 
                },
                // (3xkk) SKIP the following instruction if the value of register 'Vx' is EQUAL to 'kk'
                0x3000..=0x3FFF => {
                    self.skip_next_if_equal(
                        ((opcode & 0xF000) >> 8) as u8, // x
                        (opcode & 0x00FF) as u8,        // kk
                    ); 
                },
                // (4xkk) SKIP the following instruction if the value of register 'Vx' is NOT EQUAL 'kk'
                0x4000..=0x4FFF => { 
                    self.skip_next_if_not_equal(
                        ((opcode & 0xF000) >>  8) as u8, // x
                        (opcode & 0x00FF) as u8,         // kk
                    );
                },
                // (5xkk) SKIP the following instruction if the value of register 'Vx' is EQUAL to 'Vy'
                0x5000..=0x5FF0 => {
                    self.skip_next_if_equal(
                        ((opcode & 0xF000) >>  8) as u8, // x
                        ((opcode & 0x0F00) >>  4) as u8, // y
                    );
                },
                // (6xkk) store (LOAD) value 'kk' in register 'vx'
                0x6000..=0x6FFF => {
                    self.load(
                        ((opcode & 0xF000) >>  8) as u8, // x
                        (opcode & 0x00FF) as u8,         // kk
                    );
                },
                // ADD value 'kk' to register 'vx'
                0x7000..=0x7FFF => {
                    self.add(
                        ((opcode & 0xF000) >>  8) as u8, // x
                        (opcode & 0x00FF) as u8,         // kk
                    );
                },
                0x8000..=0x8FFF => {
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
                            // (8xy4) ADD 'Vx' to 'Vy'--
                            // if result > 8 bits, 'VF' is set to 1, else 0
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

    /// (00ee) RETURN from the current sub-routine
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("stack overflow!");
        }

        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
    }

    /// (1nnn) JUMP to location 'nnn'
    fn jump(&mut self, addr: u16) { 
        self.program_counter = addr as usize;
    }

    /// (2nnn) CALL subroutine starting at address 'nnn'
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

    /// (3xkk | 5xy0) SKIP the following instruction if the value of register 'Vx' is EQUAL to 't'.
    /// Parameter 't' can be either 'kk' or 'y'.
    fn skip_next_if_equal(&mut self, vx: u8, t: u8) {
        if vx == t {
            self.program_counter += 2;
        }
    }

    /// (4xkk) SKIP the following instruction if the value of register 'Vx' is NOT EQUAL 'kk'
    fn skip_next_if_not_equal(&mut self, vx: u8, kk: u8) {
        if vx != kk {
            self.program_counter += 2;
        }
    }

    /// (6xkk) LOAD value 'kk' in register 'Vx'
    fn load(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] = kk; 
    }

    /// (7xkk) ADD value 'kk' to register 'Vx'
    fn add(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] += kk; 
    }

    /// (8xy1) perform bitwise OR on 'Vx' and 'Vy' and store result in 'Vx'.
    fn or_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ | y_;
    }

    /// (8xy2) perform bitwise AND on 'Vx' and 'Vy' and store result in 'Vx'.
    fn and_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ & y_;
    }

    /// (8xy3) perform bitwise exclusive OR on 'Vx' and 'Vy' and store result in 'Vx'.
    fn xor_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ ^ y_;
    }

    /// (8xy4) ADD 'Vx' to 'Vy'--if result > 8 bits, 'VF' is set to 1, else 0.
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
}
