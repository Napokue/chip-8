mod cpu;

use cpu::Cpu;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cpu = Cpu::new(&args[1]);
}
