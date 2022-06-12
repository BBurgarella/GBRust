#[allow(unused_imports)]
use super::*;


// ---------------------------------------------------
//                  0x40 to 0x4F
// ---------------------------------------------------
#[test]
fn _40_ld_b_b() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0x00CC;
    assert_eq!(test_cpu.register_bc, 0x00CC); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x40), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0x00CC); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _41_ld_b_c() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0x00CC;
    assert_eq!(test_cpu.register_bc, 0x00CC); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x41), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0xCCCC); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _41_ld_b_d() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDD00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x42), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0xDD00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _42_ld_b_d() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDD00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x42), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0xDD00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _43_ld_b_e() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDDEE;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x43), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0xEE00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _44_ld_b_h() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xDD00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x44), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0xDD00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _45_ld_b_l() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0x00F3;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x45), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0xF300); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _46_ld_b_phl() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xC0F3;
    test_cpu.mem_set(test_cpu.register_hl as usize, 0xBB);
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x46), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0xBB00); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _47_ld_b_a() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0xAAFF;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x47), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0xAA00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _48_ld_c_b() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xBBCC;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x48), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0xBBBB); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _49_ld_c_c() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0x00CC;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x49), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0x00CC); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _4a_ld_c_d() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDD00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x4A), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0x00DD); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _4b_ld_c_e() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDDEE;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x4B), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0x00EE); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _4c_ld_c_h() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xDD00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x4C), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0x00DD); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _4d_ld_c_l() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0x00F3;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x4D), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0x00F3); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _4e_ld_c_phl() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xC0F3;
    test_cpu.mem_set(test_cpu.register_hl as usize, 0xCC);
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x4E), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0x00CC); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _4f_ld_c_a() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0xAA00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x4F), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_bc, 0x00AA); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}