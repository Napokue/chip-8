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

    fn generate_sprite_map(&self,
        height: usize) -> Vec<Vec<usize>> {
        let mut sprite_map : Vec<Vec<usize>> = vec![];

        for _ in 0..height {
            let inner_vec : Vec<usize> = (0..SPRITE_WIDTH)
            .map(|_|1 as usize)
            .collect();
            sprite_map.push(inner_vec);            
        }

        sprite_map
    }

    fn overlay_map(&mut self,
        sprite_x: usize,
        sprite_y: usize,
        sprite_map : Vec<Vec<usize>>,
        height: usize) {

        let mut current_y_sprite = 0;
        let mut current_x_sprite = 0;

        for y in sprite_y..sprite_y + height  {            
            for x in sprite_x..sprite_x + SPRITE_WIDTH {
                let screen_pixel = self.vram[y][x] as usize;
                let sprite_pixel = sprite_map[current_y_sprite][current_x_sprite];

                let pixel_set = screen_pixel ^ sprite_pixel != 0;
                
                if pixel_set {
                    self.vram[y][x] = 1;
                }
                else {
                    self.vram[y][x] = 0;
                }

                current_x_sprite += 1;
            }
            current_x_sprite = 0;
            current_y_sprite += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_sprite_map() {
        let driver = Driver::new();
        let sprite_map = driver.generate_sprite_map(10);

        println!("Raw map: {:?}", sprite_map);
        println!("Display map: ");
        for y in 0..sprite_map.len() {
            print!("Row {}: ", y);
            for x in sprite_map[y].iter() {
                match x {
                    1 => print!("*"),
                    _ => print!("o")
                }                
            }
            println!("");
        }
    }

    #[test]
    fn test_overlay_map() {
        let height = 10;
        let mut driver = Driver::new();
        let sprite_map = driver.generate_sprite_map(height);
        driver.overlay_map(50, 10, sprite_map, height);

        let vram = driver.vram;

        println!("Display map: ");
        for y in 0..vram.len() {
            for x in vram[y].iter() {
                match x {
                    1 => print!("*"),
                    _ => print!("o")
                }
            }
            println!("");
        }
    }
}