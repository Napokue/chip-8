mod cpu;

use cpu::Cpu;

use std::{env, thread, time};

use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder};
use winit::dpi::{LogicalSize};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cpu = Cpu::new(&args[1]);

    let dpi_factor = 10.0;
    let screen_size = (64.0 * dpi_factor, 32.0 * dpi_factor);

    let mut events_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(screen_size.0, screen_size.1))
        .build(&events_loop)
        .unwrap();

        events_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
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
                            } else {
                                println!("Released Key1");
                            }
                        }
                        (VirtualKeyCode::Key2, _) => {
                            if state == ElementState::Pressed {
                                println!("Pressed Key2");
                            } else {
                                println!("Released Key2");
                            }
                        }
                        (VirtualKeyCode::Key3, _) => {
                            if state == ElementState::Pressed {
                                println!("Pressed Key3");
                            } else {
                                println!("Released Key3");
                            }
                        }
                        (VirtualKeyCode::Key4, _) => {
                            if state == ElementState::Pressed {
                                println!("Pressed Key4");
                            } else {
                                println!("Released Key4");
                            }
                        }
                        (VirtualKeyCode::Q, _) => {
                            if state == ElementState::Pressed {
                                println!("Pressed Q");
                            } else {
                                println!("Released Q");
                            }
                        }
                        (VirtualKeyCode::W, _) => {
                            if state == ElementState::Pressed {
                                println!("Pressed W");
                            } else {
                                println!("Released W");
                            }
                        }
                        (VirtualKeyCode::E, _) => {
                            if state == ElementState::Pressed {
                                println!("Pressed E");
                            } else {
                                println!("Released E");
                            }
                        }
                        (VirtualKeyCode::R, _) => {
                            if state == ElementState::Pressed {
                                println!("Pressed R");
                            } else {
                                println!("Released R");
                            }
                        }
                        (VirtualKeyCode::A, _) => {
                            if state == ElementState::Pressed {
                                println!("Pressed A");
                            } else {
                                println!("Released A");
                            }
                        }
                        (VirtualKeyCode::S, _) => {
                            if state == ElementState::Pressed {
                                println!("Pressed S");
                            } else {
                                println!("Released S");
                            }
                        }
                        (VirtualKeyCode::D, _) => {
                            if state == ElementState::Pressed {
                                println!("Pressed D");
                            } else {
                                println!("Released D");
                            }
                        }
                        (VirtualKeyCode::F, _) => {
                            if state == ElementState::Pressed {
                                println!("Pressed F");
                            } else {
                                println!("Released F");
                            }
                        }
                        (VirtualKeyCode::Z, _) => {
                            if state == ElementState::Pressed {
                                println!("Pressed Z");
                            } else {
                                println!("Released Z");
                            }
                        }
                        (VirtualKeyCode::X, _) => {
                            if state == ElementState::Pressed {
                                println!("Pressed X");
                            } else {
                                println!("Released X");
                            }
                        }
                        (VirtualKeyCode::C, _) => {
                            if state == ElementState::Pressed {
                                println!("Pressed C");
                            } else {
                                println!("Released C");
                            }
                        }
                        (VirtualKeyCode::V, _) => {
                            if state == ElementState::Pressed {
                                println!("Pressed V");
                            } else {
                                println!("Released V");
                            }
                        }
                        (VirtualKeyCode::Escape, _) => *control_flow = ControlFlow::Exit,                       
                        _ => (),
                    },
                    _ => (),
                },
                Event::EventsCleared => {
                    cpu.emulate_cycle();

                    // TODO Causing a stutter, CHIP-8 specification has a limit of ticks per second
                    thread::sleep(time::Duration::from_millis(500));
                }
                _ => {}
            }            
        });
}
