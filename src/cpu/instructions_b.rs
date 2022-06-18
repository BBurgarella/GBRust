#[allow(unused_imports)]
use super::*;


// ---------------------------------------------------
//                  0xB0 to 0xBF
// ---------------------------------------------------
#[test]
fn _b0_or_a_b() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_b(0b11110000);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xB0), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b11110101);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _b1_or_a_c() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_c(0b11110000);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xB1), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b11110101);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _b2_or_a_d() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_d(0b11110000);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xB2), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b11110101);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _b3_or_a_e() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_e(0b11110000);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xB3), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b11110101);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _b4_or_a_h() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_h(0b11110000);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xB4), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b11110101);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _b5_or_a_l() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.set_l(0b11110000);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xB5), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b11110101);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _b6_or_a_phl() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    test_cpu.mem_set(0xC00C, 0b11110000);
    test_cpu.register_hl = 0xC00C;
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xB6), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b11110101);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _b7_or_a_a() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0b01010101);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0xB7), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0b01010101);
    assert_eq!(test_cpu.f(), 0b00000000); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}