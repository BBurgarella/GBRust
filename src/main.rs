// Main file of GBRust
mod cpu;
mod memory;
mod gameboy;
mod bus;
mod gbdisassembler;
use std::time::Instant;

use crate::gameboy::GameBoy;

fn main(){
    let now = Instant::now();
    
    let mut my_gameboy: GameBoy = GameBoy::default();

    my_gameboy.boot("src/Data/Pokemon_red.gb");

    let elapsed = now.elapsed();
    println!("\nEnd of main function, total time: {:.2?}", elapsed);
}
