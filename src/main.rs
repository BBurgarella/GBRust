mod gui;
mod gameboy;
mod gbdisassembler;
mod memory;
mod cpu;
mod bus;


use gameboy::GameBoy;
 
pub fn main() {
    let mut testgb = GameBoy::default();
    testgb.boot("src/data/snake.gb");
    }