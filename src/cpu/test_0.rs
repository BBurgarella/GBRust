#[allow(unused_imports)]
use super::*;

// ---------------------------------------------------
//                  0x00 to 0x0F
// ---------------------------------------------------

#[test]
fn test_write_program(){
    // instantiate the CPU
    let mut test_cpu: CPU = CPU::default();
    // Write a simple programm just to check that it works
    test_cpu.write_program(vec!(0xFF, 0x11, 0x22), 0xC000);

    // check that the values written in RAM are right
    assert_eq!(test_cpu.mem_read(0xC000), 0xFF);
    assert_eq!(test_cpu.mem_read(0xC001), 0x11);
    assert_eq!(test_cpu.mem_read(0xC002), 0x22);
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
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x01, 0xCC, 0xBB), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xBBCC); 
    assert_eq!(test_cpu.b(), 0xBB); 
    assert_eq!(test_cpu.c(), 0xCC); 
    assert_eq!(cycles, 12);
    assert_eq!(test_cpu.register_pc, 0xC003);
}

#[test]
fn _02_ld_pbc_a(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.set_a(0xAA);
    test_cpu.register_bc = 0xCBCC;
    test_cpu.register_pc = 0xC0C0;
    test_cpu.write_program(vec!(0x02), 0xC0C0);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.mem_read(0xCBCC), 0xAA); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC0C1);
}

#[test]
fn _03_inc_bc(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xFFF0;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x03), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xFFF1); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _04_inc_b(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xF0F0;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x04), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xF1F0); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _04_inc_b_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xFF00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x04), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0x0000); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _05_dec_b(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xF2F0;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x05), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xF1F0); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _05_dec_b_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0x00F0;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x05), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xFFF0); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _06_ld_b_u8(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xF1F0;
    assert_eq!(test_cpu.register_bc, 0xF1F0); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x06, 0x88), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.b(), 0x88);
    assert_eq!(test_cpu.register_bc, 0x88F0); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC002);
}

#[test]
fn _07_rcla(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0xAA00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x07), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x55);
    assert_eq!(test_cpu.carry_flag(), 1);
    assert_eq!(test_cpu.register_af,  0x5510); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _08_ld_pu_16_sp(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.mem_set(0xC234, 0x56);
    test_cpu.mem_set(0xC235, 0x65);
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x08, 0x34, 0xC2), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_sp,  0x6556); 
    assert_eq!(cycles, 20);
    assert_eq!(test_cpu.register_pc, 0xC003);
}

#[test]
fn _09_add_hl_bc(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0x0000;
    test_cpu.register_bc = 0x0001;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x09), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_hl,  0x0001); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _09_add_hl_bc_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xFFFF;
    test_cpu.register_bc = 0x00F1;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x09), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_hl,  0x00F0); 
    assert_eq!(test_cpu.carry_flag(),  1);
    assert_eq!(test_cpu.half_carry_flag(),  1); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _0a_ld_a_bc(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.mem_set(0xCAAA, 0xAA);
    test_cpu.register_bc = 0xCAAA;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x0A), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(),  0xAA); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _0b_dec_bc(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xFFF1;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x0B), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xFFF0); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _0c_inc_c(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xF0F0;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x0c), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xF0F1); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _0c_inc_c_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0x00FF;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x0c), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0x0000); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _0d_dec_c(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xF0F2;
    assert_eq!(test_cpu.register_bc, 0xF0F2); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x0d), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0xF0F1); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _0d_dec_c_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0x0000;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x0d), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_bc, 0x00FF); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _0e_ld_c_u8(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xF1F0;
    assert_eq!(test_cpu.register_bc, 0xF1F0); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x0e, 0x88), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.c(), 0x88);
    assert_eq!(test_cpu.register_bc, 0xF188); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC002);
}

#[test]
fn _0f_rrla(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0x5500;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x0F), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0xAA);
    assert_eq!(test_cpu.carry_flag(), 1);
    assert_eq!(test_cpu.register_af,  0xAA10); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

// ---------------------------------------------------
//                  0x10 to 0x1F
// ---------------------------------------------------

#[test]
fn _10_stop(){
    // instantiate the CPU
    let mut test_cpu: CPU = CPU::default();
    // reset the programm counter
    test_cpu.register_pc = 0xC000;
    // write a simple programm: NOP
    test_cpu.write_program(vec!(0x10), 0xC000);
    // let the CPU tick
    let cycles = test_cpu.tic();  
    // and check that the returned number of cycles is right
    assert_eq!(cycles, 4);
    assert!(test_cpu.standbymode);  
    assert_eq!(test_cpu.register_pc, 0xC001);   
}

#[test]
fn _11_ld_de_u16(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x11, 0xEE, 0xDD), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_de, 0xDDEE); 
    assert_eq!(test_cpu.d(), 0xDD); 
    assert_eq!(test_cpu.e(), 0xEE); 
    assert_eq!(cycles, 12);
    assert_eq!(test_cpu.register_pc, 0xC003);
}

#[test]
fn _12_ld_pde_a(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.set_a(0xAA);
    test_cpu.register_de = 0xDDEE;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x12), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.mem_read(0xDDEE), 0xAA); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _13_inc_de(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xFFF0;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x13), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_de, 0xFFF1); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _14_inc_d(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xF0F0;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x14), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_de, 0xF1F0); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _14_inc_d_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xFF00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x14), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_de, 0x0000); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _15_dec_d(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xF2F0;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x15), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_de, 0xF1F0); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _15_dec_d_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0x00F0;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x15), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_de, 0xFFF0); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _16_ld_d_u8(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xF1F0;
    assert_eq!(test_cpu.register_de, 0xF1F0); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x16, 0x88), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.d(), 0x88);
    assert_eq!(test_cpu.register_de, 0x88F0); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC002);
}

#[test]
fn _17_rla(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.set_a(0b0101);
    test_cpu.set_carry_flag(true);
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x17), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x0B);
    assert_eq!(test_cpu.register_af, 0x0B00); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _18_jr_i8(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_pc = 0xC0FF;
    test_cpu.write_program(vec!(0x18,0xD6), test_cpu.register_pc);
    let cycles = test_cpu.tic(); 
    assert_eq!(cycles, 12);
    assert_eq!(test_cpu.register_pc, 0xC0D5);
}

#[test]
fn _19_add_hl_bde(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0x0000;
    test_cpu.register_de = 0x0001;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x19), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_hl,  0x0001); 
    assert_eq!(cycles, 8);
}

#[test]
fn _19_add_hl_de_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xFFFF;
    test_cpu.register_de = 0x00F1;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x19), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_hl,  0x00F0); 
    assert_eq!(test_cpu.carry_flag(),  1);
    assert_eq!(test_cpu.half_carry_flag(),  1); 
    assert_eq!(cycles, 8);
}

#[test]
fn _1a_ld_a_de(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.mem_set(0xCAAA, 0xAA);
    test_cpu.register_de = 0xCAAA;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x1A), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(),  0xAA); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _1b_dec_de(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xFFF1;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x1B), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_de, 0xFFF0); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _1c_inc_e(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xF0F0;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x1c), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_de, 0xF0F1); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _1c_inc_e_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0x00FF;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x1c), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_de, 0x0000); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _1d_dec_e(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xF0F2;
    assert_eq!(test_cpu.register_de, 0xF0F2); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x1d), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_de, 0xF0F1); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _1d_dec_e_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0x0000;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x1d), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_de, 0x00FF); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _1e_ld_e_u8(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_de = 0xF1F0;
    assert_eq!(test_cpu.register_de, 0xF1F0); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x1e, 0x88), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.e(), 0x88);
    assert_eq!(test_cpu.register_de, 0xF188); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC002);
}

// ---------------------------------------------------
//                  0x20 to 0x2F
// ---------------------------------------------------


// ---------------------------------------------------
//                  0x30 to 0x3F
// ---------------------------------------------------


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


// ---------------------------------------------------
//                  0x50 to 0x5F
// ---------------------------------------------------


// ---------------------------------------------------
//                  0x60 to 0x6F
// ---------------------------------------------------



// ---------------------------------------------------
//                  0x70 to 0x7F
// ---------------------------------------------------



// ---------------------------------------------------
//                  0x80 to 0x8F
// ---------------------------------------------------



// ---------------------------------------------------
//                  0x90 to 0x9F
// ---------------------------------------------------



// ---------------------------------------------------
//                  0xA0 to 0xAF
// ---------------------------------------------------



// ---------------------------------------------------
//                  0xB0 to 0xBF
// ---------------------------------------------------



// ---------------------------------------------------
//                  0xC0 to 0xCF
// ---------------------------------------------------



// ---------------------------------------------------
//                  0xD0 to 0xDF
// ---------------------------------------------------



// ---------------------------------------------------
//                  0xE0 to 0xEF
// ---------------------------------------------------



// ---------------------------------------------------
//                  0xF0 to 0xFF
// ---------------------------------------------------