use crate::{
    boilerplate::Application
};

mod engines;
mod drivers;

mod cpu;

use cpu::Cpu;

use std::{env, thread, time};

#[path = "./boilerplate.rs"]
mod boilerplate;

const CPU_DELAY : u64 = 5000; // microseconds

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cpu = Cpu::new(&args[1]);

    let mut harness = boilerplate::Harness::new("Chip 8 Emulator");

    let renderer = engines::graphics::Engine::new();
}
