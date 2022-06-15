#[allow(unused_imports)]
use super::*;


// ---------------------------------------------------
//                  0x40 to 0x4F
// ---------------------------------------------------
#[test]
fn _60_ld_h_b() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xBBCC;
    assert_eq!(test_cpu.register_bc, 0xBBCC); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x60), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0xBB00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _61_ld_h_c() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0x00CC;
    assert_eq!(test_cpu.register_bc, 0x00CC); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x61), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0xCC00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _62_ld_h_d() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDD00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x62), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0xDD00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _63_ld_h_e() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDDEE;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x63), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0xEE00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _64_ld_h_h() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xDD00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x64), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0xDD00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _65_ld_h_l() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0x00F3;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x65), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0xF3F3); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _66_ld_d_phl() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xC0F3;
    test_cpu.mem_set(test_cpu.register_hl as usize, 0xBB);
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x66), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0xBBF3); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _67_ld_h_a() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0xAAFF;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x67), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0xAA00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _68_ld_l_b() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xBBCC;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x68), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0x00BB); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _69_ld_e_c() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0x00CC;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x69), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0x00CC); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _6a_ld_l_d() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDD00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x6A), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0x00DD); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _6b_ld_l_e() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDDEE;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x6B), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0x00EE); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _6c_ld_l_h() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xDD00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x6C), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0xDDDD); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _6d_ld_l_l() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0x00F3;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x6D), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0x00F3); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _6e_ld_l_phl() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xC0F3;
    test_cpu.mem_set(test_cpu.register_hl as usize, 0xCC);
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x6E), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0xC0CC); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _6f_ld_l_a() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0xAA00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x6F), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_hl, 0x00AA); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}