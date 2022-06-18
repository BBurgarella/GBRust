#[allow(unused_imports)]
use super::*;


// ---------------------------------------------------
//                  0x40 to 0x4F
// ---------------------------------------------------
#[test]
fn _90_sub_a_b() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(15);
    test_cpu.set_b(10);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x90), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 5); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _90_sub_a_b_carry() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(10);
    test_cpu.set_b(15);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x90), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xFA); 
    assert_eq!(test_cpu.carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _90_sub_a_b_hcarry() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0x1E);
    test_cpu.set_b(0x0F);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x90), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x0F); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _91_sub_a_c() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(15);
    test_cpu.set_c(10);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x91), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 5); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _91_sub_a_c_carry() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(10);
    test_cpu.set_c(15);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x91), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xFA); 
    assert_eq!(test_cpu.carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _91_sub_a_c_hcarry() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0x1E);
    test_cpu.set_c(0x0F);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x91), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x0F); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _92_sub_a_d() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(15);
    test_cpu.set_d(10);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x92), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 5); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _92_sub_a_c_carry() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(10);
    test_cpu.set_d(15);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x92), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xFA); 
    assert_eq!(test_cpu.carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _92_sub_a_d_hcarry() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0x1E);
    test_cpu.set_d(0x0F);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x92), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x0F); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _93_sub_a_e() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(15);
    test_cpu.set_e(10);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x93), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 5); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _93_sub_a_e_carry() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(10);
    test_cpu.set_e(15);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x93), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xFA); 
    assert_eq!(test_cpu.carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _93_sub_a_e_hcarry() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0x1E);
    test_cpu.set_e(0x0F);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x93), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x0F); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _94_sub_a_h() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(15);
    test_cpu.set_h(10);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x94), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 5); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _94_sub_a_h_carry() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(10);
    test_cpu.set_h(15);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x94), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xFA); 
    assert_eq!(test_cpu.carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _94_sub_a_h_hcarry() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0x1E);
    test_cpu.set_h(0x0F);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x94), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x0F); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _95_sub_a_l() {
    // b = 10, a = 15
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(15);
    test_cpu.set_l(10);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x95), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 5); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _95_sub_a_l_carry() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(10);
    test_cpu.set_l(15);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x95), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xFA); 
    assert_eq!(test_cpu.carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _95_sub_a_l_hcarry() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0x1E);
    test_cpu.set_l(0x0F);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x95), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x0F); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _96_sub_a_phl() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(15);
    test_cpu.register_hl = 0xC00F;
    test_cpu.mem_set(0xC00F, 10);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x96), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 5); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}


#[test]
fn _96_sub_a_phl_carry() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(10);
    test_cpu.register_hl = 0xC00F;
    test_cpu.mem_set(0xC00F, 15);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x96), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xFA); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _96_sub_a_phl_hcarry() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0x1E);
    test_cpu.register_hl = 0xC00F;
    test_cpu.mem_set(0xC00F, 0x0F);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x96), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x0F); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _97_sub_a_a() {
    // b = 15, a = 10
    // SUB A,B -> 5
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.set_a(0x1E);
    // set the program counter
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x97), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x00); 
    assert_eq!(test_cpu.zero_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);   
}