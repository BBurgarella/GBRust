#[allow(unused_imports)]
use super::*;


// ---------------------------------------------------
//                  0x20 to 0x2F
// ---------------------------------------------------

#[test]
fn _30_jr_nc_i8(){
    // instantiate the CPU
    let mut test_cpu: CPU = CPU::default();
    // reset the programm counter
    test_cpu.register_pc = 0xC000;
    test_cpu.set_carry_flag(true);
    // write a simple programm: NOP
    test_cpu.write_program(vec!(0x30, 0x00, 0x30, 0x10), 0xC000);
    // let the CPU tick
    let cycles = test_cpu.tic(); 
    test_cpu.set_carry_flag(false);
    assert_eq!(test_cpu.register_pc, 0xC002);  
    // and check that the returned number of cycles is right
    assert_eq!(cycles, 8);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_pc, 0xC012);   
    assert_eq!(cycles, 12);
}

#[test]
fn _32_ldd_hl_a(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_pc = 0xC000;
    test_cpu.set_a(0xAA);
    test_cpu.register_hl = 0xC234;
    test_cpu.write_program(vec!(0x32), 0xC000);
    let cycles = test_cpu.tic();
    assert_eq!(test_cpu.mem_read(0xC234), 0xAA);
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);
    assert_eq!(test_cpu.register_hl, 0xC233);
}

#[test]
fn _33_inc_sp(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_pc = 0xC000;
    test_cpu.register_sp = 0x0000;
    test_cpu.write_program(vec!(0x33), 0xC000);
    let cycles = test_cpu.tic();
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);
    assert_eq!(test_cpu.register_sp, 0x0001)
}

#[test]
fn _34_inc_phl(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xC0F0;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x34), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.mem_read(test_cpu.register_hl as usize), 0x01); 
    assert_eq!(cycles, 12);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _34_inc_phl_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xCF00;
    test_cpu.mem_set(0xCF00, 0xFF);
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x34), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.mem_read(test_cpu.register_hl as usize), 0x00); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 12);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _35_dec_phl(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xC3F0;
    test_cpu.mem_set(0xC3F0, 0x11);
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x35), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.mem_read(0xC3F0), 0x10); 
    assert_eq!(cycles, 12);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _35_dec_phl_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xC3F0;
    test_cpu.register_pc = 0xC000;
    test_cpu.mem_set(0xC3F0, 0x00);
    test_cpu.write_program(vec!(0x35), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.mem_read(0xC3F0), 0xFF); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 12);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _36_ld_phl_u8(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xC1F0;
    assert_eq!(test_cpu.register_hl, 0xC1F0); 
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x36, 0x88), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.mem_read(0xC1F0), 0x88);
    assert_eq!(cycles, 12);
    assert_eq!(test_cpu.register_pc, 0xC002);
}

#[test]
fn _37_scf(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x37), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.carry_flag(), 1);
    assert_eq!(test_cpu.half_carry_flag(), 0);
    assert_eq!(test_cpu.subtract_flag(), 0);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _38_jr_c_i8(){
    // instantiate the CPU
    let mut test_cpu: CPU = CPU::default();
    // reset the programm counter
    test_cpu.register_pc = 0xC000;
    test_cpu.set_carry_flag(false);
    // write a simple programm: NOP
    test_cpu.write_program(vec!(0x38, 0x00, 0x38, 0x10), 0xC000);
    // let the CPU tick
    let cycles = test_cpu.tic(); 
    test_cpu.set_carry_flag(true);
    assert_eq!(test_cpu.register_pc, 0xC002);  
    // and check that the returned number of cycles is right
    assert_eq!(cycles, 8);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_pc, 0xC012);   
    assert_eq!(cycles, 12);
}

#[test]
fn _39_add_hl_de(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0x0000;
    test_cpu.register_sp = 0x0001;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x39), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_hl,  0x0001); 
    assert_eq!(cycles, 8);
}

#[test]
fn _39_add_hl_de_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_hl = 0xFFFF;
    test_cpu.register_sp = 0x00F1;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x39), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_hl,  0x00F0); 
    assert_eq!(test_cpu.carry_flag(),  1);
    assert_eq!(test_cpu.half_carry_flag(),  1); 
    assert_eq!(cycles, 8);
}


#[test]
fn _3a_ldd_a_hl(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_pc = 0xC000;
    test_cpu.mem_set( 0xC234, 0xAA);
    test_cpu.register_hl = 0xC234;
    test_cpu.write_program(vec!(0x3A), 0xC000);
    let cycles = test_cpu.tic();
    assert_eq!(test_cpu.a(), 0xAA);
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);
    assert_eq!(test_cpu.register_hl, 0xC233);
}

#[test]
fn _3b_dec_sp(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_sp = 0xFFF1;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x3B), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_sp, 0xFFF0); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _3c_inc_a(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0xF000;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x3C), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0xF100); 
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _3c_inc_a_carries(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0xFF00;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x3C), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_af, 0x00A0); 
    assert_eq!(test_cpu.half_carry_flag(), 1);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);
}

#[test]
fn _3e_ld_a_u8(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_af = 0xF1F0;
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x3e, 0x88), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.a(), 0x88);
    assert_eq!(test_cpu.register_af, 0x88F0); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC002);
}

#[test]
fn _3f_ccf(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_pc = 0xC000;
    test_cpu.write_program(vec!(0x3F), 0xC000);
    let cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.carry_flag(), 1);
    assert_eq!(test_cpu.half_carry_flag(), 0);
    assert_eq!(test_cpu.subtract_flag(), 0);
    assert_eq!(cycles, 4);
    assert_eq!(test_cpu.register_pc, 0xC001);

    test_cpu.write_program(vec!(0x3F), 0xC001);
    let _cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.carry_flag(), 0);
    assert_eq!(test_cpu.half_carry_flag(), 0);
    assert_eq!(test_cpu.subtract_flag(), 0);
}