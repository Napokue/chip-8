use std::fs;

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

        println!("{}", opcode);

        match opcode & 0x0F000 {
            0x0000 => {
                println!("{}", 0x0000);
            },
            0x1000 => {
                println!("{}", 0x0000);
            },
            0x2000 => {
                println!("{}", 0x2000);
            },
            0x3000 => {
                println!("{}", 0x3000);
            },
            0x4000 => {
                println!("{}", 0x4000);
            },
            0x5000 => {
                println!("{}", 0x5000);
            },
            0x6000 => {
                println!("{}", 0x6000);
            },
            0x7000 => {
                println!("{}", 0x7000);
            },
            0x8000 => {
                println!("{}", 0x8000);
            },
            0x9000 => {
                println!("{}", 0x9000);
            }
            0xA000 => {
                self.i = (opcode & 0x0FFF) as usize;
                self.pc += 2;
                println!("{}", 0xA000);
            },
            0xB000 => {
                println!("{}", 0xB000);
            },
            0xC000 => {
                println!("{}", 0xC000);
            },
            0xD000 => {
                println!("{}", 0xD000);
            },
            0xE000 => {
                println!("{}", 0xE000);
            },
            0xF000 => {
                println!("{}", 0xF000);
            },
            _ => println!("Unknown opcode: {}", opcode)
        }
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
