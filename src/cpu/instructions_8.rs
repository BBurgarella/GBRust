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

#[test]
fn _88_adc_a_b() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xFFCC;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x88, 0x88), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0130); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);  
    test_cpu.register_bc = 0x0000; 
    test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0200); 
    assert_eq!(test_cpu.register_pc, 0xC002);   
}

#[test]
fn _89_adc_a_c() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0x00FF;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x89, 0x89), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0130); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);  
    test_cpu.register_bc = 0x0000; 
    test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0200); 
    assert_eq!(test_cpu.register_pc, 0xC002);   
}

#[test]
fn _8a_adc_a_d() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xFF00;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x8A, 0x8A), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0130); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);  
    test_cpu.register_de = 0x0000; 
    test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0200); 
    assert_eq!(test_cpu.register_pc, 0xC002);   
}

#[test]
fn _8b_adc_a_e() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0x00FF;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_hl = 0xC00C;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x8b, 0x8b), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0130); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);  
    test_cpu.register_de = 0x0000; 
    test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0200); 
    assert_eq!(test_cpu.register_pc, 0xC002);   
}

#[test]
fn _8c_adc_a_h() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xFF00;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x8c, 0x8c), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0130); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);  
    test_cpu.register_hl = 0x0000; 
    test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0200); 
    assert_eq!(test_cpu.register_pc, 0xC002);   
}

#[test]
fn _8d_adc_a_l() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0x00FF;
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x8d, 0x8d), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0130); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);  
    test_cpu.register_hl = 0x0000; 
    test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0200); 
    assert_eq!(test_cpu.register_pc, 0xC002);   
}

#[test]
fn _8e_adc_a_phl() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xC00C;
    test_cpu.mem_set(0xC00C, 0xFF);
    test_cpu.register_af = 0x0200;
    assert_eq!(test_cpu.register_af, 0x0200); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x8e, 0x8e), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0130); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);  
    test_cpu.mem_set(0xC00C, 0x00);
    test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0200); 
    assert_eq!(test_cpu.register_pc, 0xC002);   
}

#[test]
fn _8f_adc_a_a() {
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0xFF00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x8f, 0x8f, 0x8F), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0xFE30); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);  
    test_cpu.register_af = 0x0030; 

    test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0100); 
    assert_eq!(test_cpu.register_pc, 0xC002);   
    test_cpu.register_af = 0x0000; 

    test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x0080); 
    assert_eq!(test_cpu.register_pc, 0xC003);   
}