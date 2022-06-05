use super::*;
#[test]
fn test_memory_smart_pointers(){
    let mut my_gameboy: GameBoy = GameBoy::default();
    my_gameboy.mem_set(0xFFFE, 0xF4);
    my_gameboy.cpu.mem_set(0xC0FF, 0xEE);
    assert_eq!( my_gameboy.cpu.mem_read(0xFFFE), 0xF4);
    assert_eq!( my_gameboy.mem_read(0xC0FF), 0xEE);
}
