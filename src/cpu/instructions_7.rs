#[allow(unused_imports)]
use super::*;


// ---------------------------------------------------
//                  0x40 to 0x4F
// ---------------------------------------------------
#[test]
fn _70_ld_phl_b() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xBBCC;
    test_cpu.register_hl = 0xC00C;
    assert_eq!(test_cpu.register_bc, 0xBBCC); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x70), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.mem_read(0xC00C), 0xBB); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _71_ld_phl_c() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xBBCC;
    test_cpu.register_hl = 0xC00C;
    assert_eq!(test_cpu.register_bc, 0xBBCC); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x71), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.mem_read(0xC00C), 0xCC); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _72_ld_phl_d() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDDEE;
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x72), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.mem_read(0xC00C), 0xDD); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _73_ld_phl_e() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDDEE;
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x73), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.mem_read(0xC00C), 0xEE); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _74_ld_phl_e() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x74), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.mem_read(0xC00C), 0xC0); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _75_ld_phl_l() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x75), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.mem_read(0xC00C), 0x0C); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _76_halt() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x76), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.standbymode, true); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _77_ld_phl_a() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0xAAFF;
    test_cpu.register_pc = 0xC000;
    test_cpu.register_hl = 0xC00C;
    test_cpu.write_program(vec!(0x77), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.mem_read(0xC00C), 0xAA); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _78_ld_a_b() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xBBCC;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x78), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_af, 0xBB00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _79_ld_a_c() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0x00CC;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x79), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_af, 0xCC00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _7a_ld_a_d() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDD00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x7A), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_af, 0xDD00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _7b_ld_a_e() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDDEE;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x7B), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_af, 0xEE00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _7c_ld_a_h() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xDD00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x7C), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_af, 0xDD00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _7d_ld_a_l() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0x00F3;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x7D), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_af, 0xF300); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _7e_ld_a_phl() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xC0F3;
    test_cpu.mem_set(test_cpu.register_hl as usize, 0xCC);
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x7E), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_af, 0xCC00); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _7f_ld_a_a() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0xAA00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x6F), 0xC000);
    let cycles = test_cpu.tic(); 
    //assert_eq!(test_cpu.b(), 0xCC);
    assert_eq!(test_cpu.register_af, 0xAA00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}