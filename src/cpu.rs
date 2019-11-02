pub struct Cpu {
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    gfx: [[u8; 64]; 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: u16,
    sp: u16,
    key: [u8; 16]
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [[0; 64]; 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: 0,
            sp: 0,
            key: [0; 16]
        }
    }
}