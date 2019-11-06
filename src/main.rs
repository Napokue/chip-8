mod cpu;

use cpu::Cpu;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cpu = Cpu::new(&args[1]);

    loop {
        cpu.emulate_cycle();
    }
}
