// Purpose: Main entry point for the emulator.
mod engines;
mod drivers;

mod cpu;

use cpu::Cpu;

use std::{env};
use bevy::DefaultPlugins;
use bevy::prelude::{App, Plugin, Update};

#[path = "./boilerplate.rs"]
mod boilerplate;

const CPU_DELAY : u64 = 5000; // microseconds

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hello_world);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
    // let mut cpu = Cpu::new(&args[1]);
    // let renderer = engines::graphics::Engine::new();

}

fn hello_world() {
    println!("Hello, world!");
}


