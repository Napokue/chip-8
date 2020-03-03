//! Implementation of the abstraction on top of the graphic driver.
use winit::{
    event,
    event_loop::{ControlFlow, EventLoop}
};

use crate::drivers::{
    graphics::Driver
};

pub struct Engine {
    driver: Driver
}

impl Engine {
    pub fn new() -> Self {        
        let driver = Driver::new();

        Engine {
            driver
        }
    }

    fn clear_screen(&mut self) {
        self.driver.clear_screen();
    }

    fn draw_sprite(&mut self) {
        let height = 10;
        let sprite_map = self.driver.generate_sprite_map(height);
        self.driver.overlay_map(10, 10, sprite_map, height);

        let event_loop = EventLoop::new();

        
        let (window, size, surface) = {
            let window = winit::window::Window::new(&event_loop).unwrap();
            let size = window.inner_size();
            let surface = wgpu::Surface::create(&window);
            (window, size, surface)
        };

        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
            },
            wgpu::BackendBit::PRIMARY,
        ).unwrap();

        


        
    }
}