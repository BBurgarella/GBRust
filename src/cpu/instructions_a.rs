#[allow(unused_imports)]
use super::*;


// ---------------------------------------------------
//                  0x40 to 0x4F
// ---------------------------------------------------
#[test]
fn _a0_and_a_b() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_b(0b11110000);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xA0), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b01010000);
    assert_eq!(test_cpu.f(), 0b00100000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _a1_and_a_c() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_c(0b11110000);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xA1), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b01010000);
    assert_eq!(test_cpu.f(), 0b00100000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}


#[test]
fn _a2_and_a_d() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_d(0b11110000);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xA2), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b01010000);
    assert_eq!(test_cpu.f(), 0b00100000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _a3_and_a_b() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_e(0b11110000);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xA3), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b01010000);
    assert_eq!(test_cpu.f(), 0b00100000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _a4_and_a_h() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_h(0b11110000);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xA4), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b01010000);
    assert_eq!(test_cpu.f(), 0b00100000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _a5_and_a_b() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_l(0b11110000);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xA5), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b01010000);
    assert_eq!(test_cpu.f(), 0b00100000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _a6_and_a_phl() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.register_hl = 0xC00C;
    test_cpu.mem_set(0xC00C, 0b11110000);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xA6), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b01010000);
    assert_eq!(test_cpu.f(), 0b00100000); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _a7_and_a_a() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xA7), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b01010101);
    assert_eq!(test_cpu.f(), 0b00100000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _a8_xor_a_b() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_b(0b10100101);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xA8), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xF0);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _a9_xor_a_c() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_c(0b10100101);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xA9), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xF0);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _aa_xor_a_b() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_d(0b10100101);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xAA), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xF0);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _ab_xor_a_e() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_e(0b10100101);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xAB), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xF0);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _ac_xor_a_b() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_h(0b10100101);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xAC), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xF0);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _ad_xor_a_l() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_l(0b10100101);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xAD), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xF0);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _ae_xor_a_phl() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.register_hl = 0xC00C;
    test_cpu.mem_set(0xC00C, 0b10100101);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xAE), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xF0);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _ad_xor_a_a() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xAD), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b01010101);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}