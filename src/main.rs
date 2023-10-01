// Purpose: Main entry point for the emulator.
mod engines;
mod drivers;

mod cpu;

use cpu::Cpu;

use std::{env};

#[path = "./boilerplate.rs"]
mod boilerplate;

const CPU_DELAY : u64 = 5000; // microseconds

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cpu = Cpu::new(&args[1]);

    let renderer = engines::graphics::Engine::new();
}
