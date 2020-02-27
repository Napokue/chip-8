//! Implementation of the low level building blocks of the driver, 
//! such as the VRAM allocation and other requirements of the emulator.

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
        x: usize,
        y: usize,
        height: usize) {

        // TODO Create property for this variable, so it can be used elsewhere
        let mut carry_flag = false;

        for screen_y in 0..SCREEN_HEIGHT {
            for screen_x in 0..SCREEN_WIDTH {                
                if x != screen_x && y != screen_y {
                    continue;
                }

                // TODO Set screen pixels to set, so we known where to draw
                
                // TODO Create a map of coordinates for the sprite

                // TODO Pass map of coordinates to WGPU to draw                
                
                // We won't need to XOR anymore, 
                // because the flag has already been set.
                if carry_flag {
                    break;
                }

                // XOR current screen pixel with the sprite
                let screen_pixel = self.vram[screen_y][screen_x] as usize;

                if screen_pixel ^ x == 0 {
                    carry_flag = true;
                }
            }
        }
    }
}