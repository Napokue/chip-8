mod cpu;

use cpu::Cpu;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cpu = Cpu::new(&args[1]);

    for i in 0..10 {
        cpu.emulate_cycle();
    }
}
