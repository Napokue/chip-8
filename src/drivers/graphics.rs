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

    fn generate_sprite_map(self,
        height: usize) -> Vec<Vec<usize>>{
        let mut sprite_map : Vec<Vec<usize>> = vec![];

        for _ in 0..height {
            let inner_vec : Vec<usize> = (0..SPRITE_WIDTH)
            .map(|_|1 as usize)
            .collect();
            sprite_map.push(inner_vec);            
        }

        sprite_map
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
                    _ => print!("")
                }                
            }
            println!("");
        }
    }
}