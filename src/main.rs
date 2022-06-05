// Main file of GBRust
mod cpu;
mod memory;
mod gameboy;
mod gbdisassembler;
use std::time::Instant;

use crate::gameboy::GameBoy;

fn main(){
    let now = Instant::now();
    
    let mut my_gameboy: GameBoy = GameBoy::default();
    for i in &my_gameboy.cpu.instruction_set{
        println!("{}", i);
    }

    my_gameboy.load_rom("src/data/snake.gb");


    my_gameboy.mem.borrow().dump(0x0000, 0x0200);

    let elapsed = now.elapsed();
    println!("\nEnd of main function, total time: {:.2?}", elapsed);
}
