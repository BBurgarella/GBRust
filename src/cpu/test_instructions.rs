use super::*;

#[test]
fn _00_nop0(){
    let mut test_cpu: CPU = CPU::default();
    let ret = test_cpu.tic(0x00);  
    assert!(ret)
}

#[test]
fn _01_ld_bc_u16(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_pc = 0x0000;
    test_cpu.mem_set(0x0000, 0x01);
    test_cpu.mem_set(0x0001, 0xBB);
    test_cpu.mem_set(0x0002, 0xCC);
    let ret = test_cpu.tic(0x01); 
    assert_eq!(test_cpu.register_bc, 0xBBCC); 
    assert_eq!(test_cpu.b(), 0xBB); 
    assert_eq!(test_cpu.c(), 0xCC); 
    assert!(ret)
}

#[test]
fn _02_ld_pbc_a(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.set_a(0xAA);
    test_cpu.register_bc = 0xBBCC;
    let ret = test_cpu.tic(0x02); 
    assert_eq!(test_cpu.mem_read(0xBBCC), 0xAA); 
    assert!(ret)
}

#[test]
fn _03_inc_bc(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.register_bc = 0xFFF0;
    let ret = test_cpu.tic(0x03); 
    assert_eq!(test_cpu.register_bc, 0xFFF1); 
    assert!(ret)
}