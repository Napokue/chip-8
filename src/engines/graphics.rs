//! Implementation of the abstraction on top of the graphic driver.
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

    fn clear_screen(&self) {
        self.driver.clear_screen();
    }

    fn draw_sprite(&self) {
        
    }
}