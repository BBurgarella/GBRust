#[allow(unused_imports)]
use super::*;

#[test]
fn test_write_programm(){
    // instantiate the CPU
    let mut test_cpu: CPU = CPU::default();
    // Write a simple programm just to check that it works
    test_cpu.write_program(vec!(0xFF, 0x11, 0x22), 0x0000);

    // check that the values written in RAM are right
    assert_eq!(test_cpu.mem_read(0x0000), 0xFF);
    assert_eq!(test_cpu.mem_read(0x0001), 0x11);
    assert_eq!(test_cpu.mem_read(0x0002), 0x22);
}

#[test]
fn _00_nop0(){
    // instantiate the CPU
    let mut test_cpu: CPU = CPU::default();
    // reset the programm counter
    test_cpu.register_pc = 0x0000;
    // write a simple programm: NOP
    test_cpu.write_program(vec!(0x00), 0x0000);
    // let the CPU tick
    let cycles = test_cpu.tic();  
    // and check that the returned number of cycles is right
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0x0001);
}

#[test]
fn _01_ld_bc_u16(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x01, 0xCC, 0xBB), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xBBCC); 
    assert_eq!(test_cpu.b(), 0xBB); 
    assert_eq!(test_cpu.c(), 0xCC); 
    assert_eq!(cycles, 12);
    assert_eq!(test_cpu.register_pc, 0x0003);
}

#[test]
fn _02_ld_pbc_a(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.set_a(0xAA);
    test_cpu.register_bc = 0xBBCC;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x02), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.mem_read(0xBBCC), 0xAA); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0x0001);
}

#[test]
fn _03_inc_bc(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xFFF0;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x03), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xFFF1); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0x0001);
}

#[test]
fn _04_inc_b(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xF0F0;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x04), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xF1F0); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0x0001);
}

#[test]
fn _04_inc_b_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xFF00;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x04), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0x0000); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0x0001);
}

#[test]
fn _05_dec_b(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xF2F0;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x05), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xF1F0); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0x0001);
}

#[test]
fn _05_dec_b_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0x00F0;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x05), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xFFF0); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0x0001);
}

#[test]
fn _06_ld_b_u8(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xF1F0;
    assert_eq!(test_cpu.register_bc, 0xF1F0); 
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x06, 0x88), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.b(), 0x88);
    assert_eq!(test_cpu.register_bc, 0x88F0); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0x0002);
}

#[test]
fn _07_rcla(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0xAA00;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x07), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x55);
    assert_eq!(test_cpu.carry_flag(), 1);
    assert_eq!(test_cpu.register_af,  0x5510); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0x0001);
}

#[test]
fn _08_ld_pu_16_sp(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.mem_set(0x1234, 0x56);
    test_cpu.mem_set(0x1235, 0x65);
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x08, 0x34, 0x12), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_sp,  0x6556); 
    assert_eq!(cycles, 20);
    assert_eq!(test_cpu.register_pc, 0x0003);
}

#[test]
fn _09_add_hl_bc(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0x0000;
    test_cpu.register_bc = 0x0001;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x09), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_hl,  0x0001); 
    assert_eq!(cycles, 8);
}

#[test]
fn _09_add_hl_bc_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xFFFF;
    test_cpu.register_bc = 0x00F1;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x09), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_hl,  0x00F0); 
    assert_eq!(test_cpu.carry_flag(),  1);
    assert_eq!(test_cpu.half_carry_flag(),  1); 
    assert_eq!(cycles, 8);
}

#[test]
fn _0a_ld_a_bc(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.mem_set(0xAAAA, 0xAA);
    test_cpu.register_bc = 0xAAAA;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x0A), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(),  0xAA); 
    assert_eq!(cycles, 8);
}

#[test]
fn _0b_dec_bc(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xFFF1;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x0B), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xFFF0); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0x0001);
}

#[test]
fn _0c_inc_c(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xF0F0;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x0c), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xF0F1); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0x0001);
}

#[test]
fn _0c_inc_c_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0x00FF;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x0c), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0x0000); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0x0001);
}

#[test]
fn _0d_dec_c(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xF0F2;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x0d), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xF0F1); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0x0001);
}

#[test]
fn _0d_dec_c_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0x0000;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x0d), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0x00FF); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0x0001);
}

#[test]
fn _0e_ld_c_u8(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xF1F0;
    assert_eq!(test_cpu.register_bc, 0xF1F0); 
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x0e, 0x88), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.c(), 0x88);
    assert_eq!(test_cpu.register_bc, 0xF188); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0x0002);
}

#[test]
fn _0f_rrla(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0x5500;
    test_cpu.register_pc = 0x0000;
    test_cpu.write_program(vec!(0x0F), 0x0000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xAA);
    assert_eq!(test_cpu.carry_flag(), 1);
    assert_eq!(test_cpu.register_af,  0xAA10); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0x0001);
}