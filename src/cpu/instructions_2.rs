#[allow(unused_imports)]
use super::*;


// ---------------------------------------------------
//                  0x20 to 0x2F
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

#[test]
fn _22_ldi_hl_a(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_pc = 0xC000;
    test_cpu.set_a(0xAA);
    test_cpu.register_hl = 0xC234;
    test_cpu.write_program(vec!(0x22), 0xC000);
    let cycles = test_cpu.tic();
    assert_eq!(test_cpu.mem_read(0xC234), 0xAA);
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);
    assert_eq!(test_cpu.register_hl, 0xC235);
}

#[test]
fn _23_inc_hl(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_pc = 0xC000;
    test_cpu.register_hl = 0x0000;
    test_cpu.write_program(vec!(0x23), 0xC000);
    let cycles = test_cpu.tic();
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);
    assert_eq!(test_cpu.register_hl, 0x0001)
}

#[test]
fn _24_inc_h(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xF0F0;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x24), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_hl, 0xF1F0); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _24_inc_h_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xFF00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x24), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_hl, 0x0000); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _25_dec_h(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xF2F0;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x25), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_hl, 0xF1F0); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _25_dec_h_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0x00F0;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x25), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_hl, 0xFFF0); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _26_ld_h_u8(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xF1F0;
    assert_eq!(test_cpu.register_hl, 0xF1F0); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x26, 0x88), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.h(), 0x88);
    assert_eq!(test_cpu.register_hl, 0x88F0); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC002);
}

#[test]
fn _27_daa_halfcarry(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.set_b(0b00011001);
    test_cpu.set_c(0b00101000);
    test_cpu.register_pc = 0xC000;
    test_cpu.set_a((test_cpu.b() + test_cpu.c()) & 0xff);
    test_cpu.set_halfcarry_flag(true);
    test_cpu.write_program(vec!(0x27), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b01000111);
    assert_eq!(cycles, 4);
}

#[test]
fn _27_daa_carry(){
    let mut test_cpu: CPU = CPU::default();
    // 91
    test_cpu.set_b(0b10010001);
    // + 82 = 173 (AD)
    test_cpu.set_c(0b10000010);
    test_cpu.register_pc = 0xC000;
    test_cpu.set_a((test_cpu.b() as u16 + test_cpu.c() as u16) as u8 & 0xff);
    test_cpu.set_carry_flag(true);
    test_cpu.write_program(vec!(0x27), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b01110011);
    assert_eq!(test_cpu.carry_flag(), 1);
    assert_eq!(cycles, 4);
}

#[test]
fn _27_daa(){
    let mut test_cpu: CPU = CPU::default();
    // 47
    test_cpu.set_b(0b01000111);
    // + 14 = 61 (AD)
    test_cpu.set_c(0b00010100);
    test_cpu.register_pc = 0xC000;
    test_cpu.set_a((test_cpu.b() as u16 + test_cpu.c() as u16) as u8 & 0xff);
    test_cpu.set_carry_flag(false);
    test_cpu.write_program(vec!(0x27), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b01100001);
    assert_eq!(cycles, 4);
}