#[allow(unused_imports)]
use super::*;


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
fn _19_add_hl_de(){
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


#[test]
fn _1f_rra(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.set_a(0b0101);
    test_cpu.set_carry_flag(true);
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x1F), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x82);
    assert_eq!(test_cpu.register_af, 0x8210); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}