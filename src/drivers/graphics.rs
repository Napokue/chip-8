//! Implementation of the low level building blocks of the driver, 
//! such as the VRAM allocation and other requirements of the emulator.

use euclid::{
    Point2D
};

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SPRITE_WIDTH: usize = 8;

pub struct Driver {
    vram: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT]
}

impl Driver {
    pub fn new() -> Self {
        Driver {
            vram: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT]
        }
    }

    pub fn clear_screen(&mut self) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.vram[y][x] = 0;
            }
        }
    }

    /// Parameter: `height` - Height of the sprite, which can
    /// be between the 1 and 15 pixels.
    /// 
    /// Parameter: `location` - X is the most left position 
    /// and Y is the top left corner of the sprite.
    pub fn draw_sprite(&mut self,
        height: usize,
        location: Point2D<usize, usize>) {
        
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {                
                if location.x != x && location.y != y {
                    continue;
                }

                // Overlapping coords found

                // XOR current screen pixels with the sprite
            }
        }
    }
}