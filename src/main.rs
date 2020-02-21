mod cpu;

use cpu::Cpu;

use std::{env, thread, time};

use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder};
use winit::dpi::{LogicalSize};

const DPI_FACTOR : f32 = 10.0;
const SCREEN_WIDTH : usize = 64;
const SCREEN_HEIGHT : usize = 32;

const WINDOW_SIZE : (f32, f32) = (
    SCREEN_WIDTH as f32 * DPI_FACTOR, 
    SCREEN_HEIGHT as f32 * DPI_FACTOR);
const CPU_DELAY : u64 = 5000; // microseconds

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cpu = Cpu::new(&args[1]);


    let mut events_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(screen_size.0, screen_size.1))
        .with_inner_size(LogicalSize::new(WINDOW_SIZE.0, WINDOW_SIZE.1))
        .build(&events_loop)
        .unwrap();

    events_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        cpu.emulate_cycle();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state,
                            ..
                        },
                    ..
                } => match (virtual_code, state) {
                    (VirtualKeyCode::Key1, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed Key1");
                            cpu.key[0x1] = 1;
                        } else {
                            println!("Released Key1");
                            cpu.key[0x1] = 0;
                        }
                    }
                    (VirtualKeyCode::Key2, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed Key2");
                            cpu.key[0x2] = 1;
                        } else {
                            println!("Released Key2");
                            cpu.key[0x2] = 0;
                        }
                    }
                    (VirtualKeyCode::Key3, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed Key3");
                            cpu.key[0x3] = 1;
                        } else {
                            println!("Released Key3");
                            cpu.key[0x3] = 0;
                        }
                    }
                    (VirtualKeyCode::Key4, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed Key4");
                            cpu.key[0xC] = 1;
                        } else {
                            println!("Released Key4");
                            cpu.key[0xC] = 0;
                        }
                    }
                    (VirtualKeyCode::Q, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed Q");
                            cpu.key[0x4] = 1;
                        } else {
                            println!("Released Q");
                            cpu.key[0x4] = 0;
                        }
                    }
                    (VirtualKeyCode::W, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed W");
                            cpu.key[0x5] = 1;
                        } else {
                            println!("Released W");
                            cpu.key[0x5] = 0;
                        }
                    }
                    (VirtualKeyCode::E, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed E");
                            cpu.key[0x6] = 1;
                        } else {
                            println!("Released E");
                            cpu.key[0x6] = 0;
                        }
                    }
                    (VirtualKeyCode::R, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed R");
                            cpu.key[0xD] = 1;
                        } else {
                            println!("Released R");
                            cpu.key[0xD] = 0;
                        }
                    }
                    (VirtualKeyCode::A, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed A");
                            cpu.key[0x7] = 1;
                        } else {
                            println!("Released A");
                            cpu.key[0x7] = 0;
                        }
                    }
                    (VirtualKeyCode::S, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed S");
                            cpu.key[0x8] = 1;
                        } else {
                            println!("Released S");
                            cpu.key[0x8] = 0;
                        }
                    }
                    (VirtualKeyCode::D, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed D");
                            cpu.key[0x9] = 1;
                        } else {
                            println!("Released D");
                            cpu.key[0x9] = 0;
                        }
                    }
                    (VirtualKeyCode::F, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed F");
                            cpu.key[0xE] = 1;
                        } else {
                            println!("Released F");
                            cpu.key[0xE] = 0;
                        }
                    }
                    (VirtualKeyCode::Z, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed Z");
                            cpu.key[0xA] = 1;
                        } else {
                            println!("Released Z");
                            cpu.key[0xA] = 0;
                        }
                    }
                    (VirtualKeyCode::X, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed X");
                            cpu.key[0x0] = 1;
                        } else {
                            println!("Released X");
                            cpu.key[0x0] = 0;
                        }
                    }
                    (VirtualKeyCode::C, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed C");
                            cpu.key[0xB] = 1;
                        } else {
                            println!("Released C");
                            cpu.key[0xB] = 0;
                        }
                    }
                    (VirtualKeyCode::V, _) => {
                        if state == ElementState::Pressed {
                            println!("Pressed V");                                
                            cpu.key[0xF] = 1;
                        } else {
                            println!("Released V");
                            cpu.key[0xF] = 0;
                        }
                    }
                    (VirtualKeyCode::Escape, _) => *control_flow = ControlFlow::Exit,                       
                    _ => (),
                },
                _ => (),
            },
            Event::MainEventsCleared  => {

                if (cpu.draw_flag) {
                    window.request_redraw();
                    cpu.draw_flag = false;
                }

                thread::sleep(time::Duration::from_micros(CPU_DELAY));
            },
            Event::RedrawRequested(_) => {
                // Redraw the application
            },
            _ => {}
        }            
    });
}
