#[allow(unused_imports)]
use super::*;


// ---------------------------------------------------
//                  0x10 to 0x1F
// ---------------------------------------------------

#[test]
fn _20_jr_nz_i8(){
    // instantiate the CPU
    let mut test_cpu: CPU = CPU::default();
    // reset the programm counter
    test_cpu.register_pc = 0xC000;
    test_cpu.set_zero_flag(true);
    // write a simple programm: NOP
    test_cpu.write_program(vec!(0x20, 0x00, 0x20, 0x10), 0xC000);
    // let the CPU tick
    let cycles = test_cpu.tic(); 
    test_cpu.set_zero_flag(false);
    assert_eq!(test_cpu.register_pc, 0xC002);  
    // and check that the returned number of cycles is right
    assert_eq!(cycles, 8);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_pc, 0xC012);   
    assert_eq!(cycles, 12);
}

#[test]
fn _11_ld_hl_u16(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x21, 0xEE, 0xDD), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_hl, 0xDDEE); 
    assert_eq!(test_cpu.h(), 0xDD); 
    assert_eq!(test_cpu.l(), 0xEE); 
    assert_eq!(cycles, 12);
    assert_eq!(test_cpu.register_pc, 0xC003);
}