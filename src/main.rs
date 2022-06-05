// Main file of GBRust
mod cpu;
mod memory;
mod gameboy;

fn main() {
    let mut my_gameboy: gameboy::GameBoy = gameboy::GameBoy::default();
    my_gameboy.cpu.set_a(0xAA);
    my_gameboy.cpu.set_f(0b11110000);
    my_gameboy.cpu.set_b(0xBB);
    my_gameboy.cpu.set_c(0xCC);
    my_gameboy.cpu.set_d(0xDD);
    my_gameboy.cpu.set_e(0xEE);
    my_gameboy.cpu.set_h(0xF3);
    my_gameboy.cpu.set_l(0x3F);
    println!("{}", my_gameboy.cpu);
    my_gameboy.cpu.set_zero_flag(false);
    println!("{}", my_gameboy.cpu);
    my_gameboy.cpu.set_subtract_flag(false);
    println!("{}", my_gameboy.cpu);
    println!("{:?}", my_gameboy.mem_read(0xFFFE));
    my_gameboy.mem_set(0xFFFE, 0xF4);
    my_gameboy.cpu.mem_set(0xC0FF, 0xEE);
    println!("0xFFFE -> {:#04X}", my_gameboy.cpu.mem_read(0xFFFE));
    println!("0xC0FF -> {:#04X}", my_gameboy.mem_read(0xC0FF));
}
