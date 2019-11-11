use std::{fs};
use rand::Rng;

pub struct Cpu {
    memory: [u8; 4096],
    v: [u8; 16],
    i: usize,
    pc: usize,
    gfx: [[u8; 64]; 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: usize,
    key: [u8; 16]
}

impl Cpu {
    pub fn new(rom_path: &str) -> Self {
        let mut memory = [0; 4096];
        
        let rom_data = fs::read(rom_path).unwrap();

        for i in 0..80 {
            memory[i] = FONT_SET[i];
        }

        for i in 0..rom_data.len() {
            memory[i + 512] = rom_data[i];
        }

        Cpu {
            memory,
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [[0; 64]; 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16]
        }
    }

    pub fn emulate_cycle(&mut self) {
        let opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);

        println!("opcode: {}", opcode);

        // Credits to https://github.com/starrhorne/chip8-rust for this nibble match implementation
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );

        let nnn = (opcode & 0x0FFF);
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as u8;
        let y = nibbles.2 as u8;
        let n = nibbles.3 as u8;

        match nibbles {
            // 00E0 - CLS
            (0x00, 0x00, 0x0E, 0x00) => {
                println!("00E0");
                for y in 0..self.gfx.len() {
                    for x in 0..self.gfx[y].len() {
                        self.gfx[y][x] = 0;
                    }
                }

                self.next_instruction();
            },
            // 00EE - RET
            (0x00, 0x00, 0x0E, 0x0E) => {
                println!("00EE");
                self.jump_to_instruction(self.stack[self.sp] as usize);
                self.sp -= 1;
            },
            // 1nnn - JP addr
            (0x01, _, _, _) => {
                println!("1nnn");
                self.jump_to_instruction(nnn as usize);
            },
            // 2nnn - CALL addr
            (0x02, _, _, _) => {
                println!("2nnn");
                self.sp += 1;
                self.stack[self.sp] = self.pc as u16;
                self.jump_to_instruction(nnn as usize);
            },
            // 3xkk - SE Vx, byte
            (0x03, _, _, _) => {
                println!("3xkk");
                if self.v[x as usize] == kk {
                    self.skip_instruction();
                } else {
                    self.next_instruction();
                }
            },
            // 4xkk - SNE Vx, byte
            (0x04, _, _, _) => {
                println!("4xkk");
                if self.v[x as usize] != kk {
                    self.skip_instruction();
                } else {
                    self.next_instruction();
                }
            },
            // 5xy0 - SE Vx, Vy
            (0x05, _, _, 0x00) => {
                println!("5xy0");
                if self.v[x as usize] == self.v[y as usize] {
                    self.skip_instruction();
                } else {
                    self.next_instruction();
                }
            },
            // 6xkk - LD Vx, byte
            (0x06, _, _, _) => {
                println!("6xkk");
                self.v[x as usize] = kk;
                self.next_instruction();
            },
            // 7xkk - ADD Vx, byte
            (0x07, _, _, _) => {
                println!("7xkk");
                self.v[x as usize] = self.v[x as usize] + kk;
                self.next_instruction();
            },
            // 8xy0 - LD Vx, Vy
            (0x08, _, _, 0x00) => {
                println!("8xy0");
                self.v[x as usize] = self.v[y as usize];
                self.next_instruction();
            },
            // 8xy1 - OR Vx, Vy
            (0x08, _, _, 0x01) => {
                println!("8xy1");
                self.v[x as usize] = self.v[x as usize] | self.v[y as usize];
                self.next_instruction();
            },
            // 8xy2 - AND Vx, Vy
            (0x08, _, _, 0x02) => {
                println!("8xy2");
                self.v[x as usize] = self.v[x as usize] & self.v[y as usize];
                self.next_instruction();
            },
            // 8xy3 - XOR Vx, Vy
            (0x08, _, _, 0x03) => {
                println!("8xy3");
                self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize];
                self.next_instruction();
            },
            // 8xy4 - ADD Vx, Vy
            (0x08, _, _, 0x04) => {
                println!("8xy4");
                self.v[x as usize] = self.v[x as usize] + self.v[y as usize];

                if self.v[x as usize] > 255 {
                    self.v[0x0F] = 1;
                } else {
                    self.v[0x0F] = 0;
                }
                self.next_instruction();
            },
            // 8xy5 - SUB Vx, Vy
            (0x08, _, _, 0x05) => {
                println!("8xy5");
                if self.v[x as usize] > self.v[y as usize] {
                    self.v[0x0F] = 1;
                } else {
                    self.v[0x0F] = 0;
                }

                self.v[x as usize] = self.v[x as usize] - self.v[y as usize];
                self.next_instruction();
            },
            // 8xy6 - SHR VX, Vy
            (0x08, _, _, 0x06) => {
                println!("8xy6");
                self.v[0x0F] = self.v[x as usize] & 0x1;
                self.v[x as usize] >>= 1;                
                self.next_instruction();
            },
            // 8xy7 - SUBN Vx, Vy
            (0x08, _, _, 0x07) => {
                println!("8xy7");
                if self.v[y as usize] > self.v[x as usize] {
                    self.v[0x0F] = 1;
                } else {
                    self.v[0x0F] = 0;
                }

                self.v[x as usize] = self.v[y as usize] - self.v[x as usize];
                self.next_instruction();
            },
            // 8xyE - SHL Vx, Vy
            (0x08, _, _, 0x0E) => {
                self.v[0x0F] = self.v[x as usize] & 0x80;
                self.v[x as usize] <<= 1;              

                println!("8xyE");
                self.next_instruction();
            },
            // 9xy0 - SNE Vx, Vy
            (0x09, _, _, 0x00) => {
                println!("9xy0");
                if self.v[x as usize] != self.v[y as usize] {
                    self.skip_instruction();
                } else {
                    self.next_instruction();
                }
            },
            // Annn - LD I, addr
            (0x0A, _, _, _) => {
                println!("Annn");
                self.i = nnn as usize;
                self.next_instruction();
            },
            // Bnnn - JP V0, addr
            (0x0B, _, _, _) => {
                println!("Bnnn");
                self.pc += nnn as usize + self.v[0] as usize;
            },
            // Cxkk - RND Vx, byte
            (0x0C, _, _, _) => {
                println!("Cxkk");
                self.v[x as usize] = 
                    rand::thread_rng().gen_range(0, 255) &
                    kk;

                self.next_instruction();
            },
            // Dxyn - DRW Vx, Vy, nibble
            (0x0D, _, _, _) => {
                // TODO
                println!("Dxyn");
                self.next_instruction();
            },
            // Ex9E - SKP Vx
            (0x0E, _, 0x09, 0x0E) => {
                // TODO
                println!("Ex9E");
                self.next_instruction();
            },
            // ExA1 - SKNP Vx
            (0x0E, _, 0x0A, 0x01) => {
                // TODO
                println!("ExA1");
                self.next_instruction();
            },
            // Fx07 - LD Vx, DT
            (0x0F, _, 0x00, 0x07) => {
                // TODO
                println!("Fx07");
                self.next_instruction();
            },
            // Fx0A - LD Vx, K
            (0x0F, _, 0x00, 0x0A) => {
                // TODO
                println!("Fx0A");
                self.next_instruction();
            },
            // Fx15 - LD DT, Vx
            (0x0F, _, 0x01, 0x05) => {
                // TODO
                println!("Fx15");
                self.next_instruction();
            },
            // Fx18 - LD ST, Vx
            (0x0F, _, 0x01, 0x08) => {
                // TODO
                println!("Fx18");
                self.next_instruction();
            },
            // Fx1E - ADD I, Vx
            (0x0F, _, 0x01, 0x0E) => {
                // TODO
                println!("Fx1E");
                self.next_instruction();
            },
            // Fx29 - LD F, Vx
            (0x0F, _, 0x02, 0x09) => {
                // TODO
                println!("Fx29");
                self.next_instruction();
            },
            // Fx33 - LD B, Vx
            (0x0F, _, 0x03, 0x03) => {
                // TODO
                println!("Fx33");
                self.next_instruction();
            },
            // Fx55 - LD [I], Vx
            (0x0F, _, 0x05, 0x05) => {
                // TODO
                println!("Fx55");
                self.next_instruction();
            },
            // Fx65 - LD Vx, [I]
            (0x0F, _, 0x06, 0x05) => {
                // TODO
                println!("Fx65");
                self.next_instruction();
            },
            _ => println!("Unknown opcode: {:?}", nibbles)
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }        
    }

    fn next_instruction(&mut self) {
        self.pc += 2;
    }

    fn skip_instruction(&mut self) {
        self.pc += 4;
    }

    fn jump_to_instruction(&mut self, addr: usize) {
        self.pc = addr;
    }
}

pub const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];
