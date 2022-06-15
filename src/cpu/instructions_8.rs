#[allow(unused_imports)]
use super::*;


// ---------------------------------------------------
//                  0x40 to 0x4F
// ---------------------------------------------------
#[test]
fn _80_add_a_b() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xBBCC;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x80), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0xBD00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _81_add_a_c() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xBBCC;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x81), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0xCE00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _82_add_a_d() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xDDEE;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x82), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0xDF00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _83_add_a_e() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0x0002;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x83), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0400); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}


#[test]
fn _84_add_a_h() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x84), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0xC200); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _85_add_a_l() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x85), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0E00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}


#[test]
fn _86_add_a_phl() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_hl = 0xC00C;
    test_cpu.mem_set(0xC00C, 0x02);
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x86), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0400); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _87_add_a_a() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x87), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0400); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _80_add_a_b_carry() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xFF00;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x80), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x01); 
    assert!(test_cpu.carry_flag()!=0);
    assert!(test_cpu.half_carry_flag()!=0);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _81_add_a_c_carry() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xFFFF;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x81), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x01); 
    assert!(test_cpu.carry_flag()!=0);
    assert!(test_cpu.half_carry_flag()!=0);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _82_add_a_d_carry() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xFF00;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x82), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x01); 
    assert!(test_cpu.carry_flag()!=0);
    assert!(test_cpu.half_carry_flag()!=0);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _83_add_a_e_carry() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0x00FF;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x83), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x01); 
    assert!(test_cpu.carry_flag()!=0);
    assert!(test_cpu.half_carry_flag()!=0);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}


#[test]
fn _84_add_a_h_carry() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xFF00;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x84), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x01); 
    assert!(test_cpu.carry_flag()!=0);
    assert!(test_cpu.half_carry_flag()!=0);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _85_add_a_l_carry() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xFFFF;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x85), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x01); 
    assert!(test_cpu.carry_flag()!=0);
    assert!(test_cpu.half_carry_flag()!=0);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}


#[test]
fn _85_add_a_phl_carry() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xC00C;
    test_cpu.mem_set(0xC00C, 0xFF);
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x86), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x01); 
    assert!(test_cpu.carry_flag()!=0);
    assert!(test_cpu.half_carry_flag()!=0);
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _87_add_a_a_carry() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0xFF00;
    assert_eq!(test_cpu.register_af, 0xFF00); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x87), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xFE); 
    assert!(test_cpu.carry_flag()!=0);
    assert!(test_cpu.half_carry_flag()!=0);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);    
}