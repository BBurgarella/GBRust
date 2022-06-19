#[allow(unused_imports)]
use super::*;


// ---------------------------------------------------
//                  0xB0 to 0xBF
// ---------------------------------------------------
#[test]
fn _c0_ret_nz() {
    // create the CPU instance
    let mut test_cpu: CPU = CPU::default();
    // set the registers
    test_cpu.register_pc = 0xC000;
    test_cpu.register_sp = 0xFFF0;
    // set the zero flag, no return on the first call
    test_cpu.set_zero_flag(true);
    test_cpu.write_program(vec!(0xC0, 0xC0), 0xC000);

    // Write the final SP value in the upper RAM
    test_cpu.write_program(vec!(0xFE, 0xFF), 0xFFF0);

    // CPU tic
    let mut cycles = test_cpu.tic(); 
    assert_eq!(cycles, 8);
    assert_eq!(test_cpu.register_pc, 0xC001); 
    test_cpu.set_zero_flag(false);  
    
    // CPU second tic
    cycles = test_cpu.tic(); 
    assert_eq!(test_cpu.register_pc, 0xFFFE); 
    assert_eq!(cycles, 20); 
}
