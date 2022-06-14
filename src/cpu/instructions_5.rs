#[allow(unused_imports)]
use super::*;


// ---------------------------------------------------
//                  0x40 to 0x4F
// ---------------------------------------------------
#[test]
fn _50_ld_d_b() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xBBCC;
    assert_eq!(test_cpu.register_bc, 0xBBCC); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x50), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0xBB00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _51_ld_d_c() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0x00CC;
    assert_eq!(test_cpu.register_bc, 0x00CC); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x51), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0xCC00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _52_ld_d_d() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDD00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x52), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0xDD00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _53_ld_d_e() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDDEE;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x53), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0xEEEE); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _54_ld_d_h() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xDD00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x54), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0xDD00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _55_ld_d_l() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0x00F3;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x55), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0xF300); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _56_ld_d_phl() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xC0F3;
    test_cpu.mem_set(test_cpu.register_hl as usize, 0xBB);
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x56), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0xBB00); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _57_ld_d_a() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0xAAFF;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x57), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0xAA00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _58_ld_e_b() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xBBCC;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x58), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0x00BB); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _59_ld_e_c() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0x00CC;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x59), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0x00CC); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _5a_ld_e_d() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDD00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x5A), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0xDDDD); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _5b_ld_e_e() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDDEE;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x5B), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0xDDEE); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _5c_ld_e_h() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xDD00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x5C), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0x00DD); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _5d_ld_e_l() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0x00F3;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x5D), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0x00F3); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _5e_ld_e_phl() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xC0F3;
    test_cpu.mem_set(test_cpu.register_hl as usize, 0xCC);
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x5E), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0x00CC); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _5f_ld_c_a() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0xAA00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x5F), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_de, 0x00AA); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}