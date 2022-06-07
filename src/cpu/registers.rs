
use super::*;
#[test]
fn a_setter_getter(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.set_a(0xAA);
    assert_eq!(test_cpu.a(), 0xAA);
    assert_eq!(test_cpu.f(), 0x00);
    assert_eq!(test_cpu.b(), 0x00);
    assert_eq!(test_cpu.c(), 0x00);
    assert_eq!(test_cpu.d(), 0x00);
    assert_eq!(test_cpu.e(), 0x00);
    assert_eq!(test_cpu.h(), 0x00);
    assert_eq!(test_cpu.l(), 0x00);        
}

#[test]
fn f_setter_getter(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.set_f(0xFF);
    assert_eq!(test_cpu.a(), 0x00);
    assert_eq!(test_cpu.f(), 0xFF);
    assert_eq!(test_cpu.b(), 0x00);
    assert_eq!(test_cpu.c(), 0x00);
    assert_eq!(test_cpu.d(), 0x00);
    assert_eq!(test_cpu.e(), 0x00);
    assert_eq!(test_cpu.h(), 0x00);
    assert_eq!(test_cpu.l(), 0x00);        
}


#[test]
fn b_setter_getter(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.set_b(0xBB);
    assert_eq!(test_cpu.a(), 0x00);
    assert_eq!(test_cpu.f(), 0x00);
    assert_eq!(test_cpu.b(), 0xBB);
    assert_eq!(test_cpu.c(), 0x00);
    assert_eq!(test_cpu.d(), 0x00);
    assert_eq!(test_cpu.e(), 0x00);
    assert_eq!(test_cpu.h(), 0x00);
    assert_eq!(test_cpu.l(), 0x00);        
}

#[test]
fn c_setter_getter(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.set_c(0xCC);
    assert_eq!(test_cpu.a(), 0x00);
    assert_eq!(test_cpu.f(), 0x00);
    assert_eq!(test_cpu.b(), 0x00);
    assert_eq!(test_cpu.c(), 0xCC);
    assert_eq!(test_cpu.d(), 0x00);
    assert_eq!(test_cpu.e(), 0x00);
    assert_eq!(test_cpu.h(), 0x00);
    assert_eq!(test_cpu.l(), 0x00);        
}

#[test]
fn d_setter_getter(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.set_d(0xDD);
    assert_eq!(test_cpu.a(), 0x00);
    assert_eq!(test_cpu.f(), 0x00);
    assert_eq!(test_cpu.b(), 0x00);
    assert_eq!(test_cpu.c(), 0x00);
    assert_eq!(test_cpu.d(), 0xDD);
    assert_eq!(test_cpu.e(), 0x00);
    assert_eq!(test_cpu.h(), 0x00);
    assert_eq!(test_cpu.l(), 0x00);        
}

#[test]
fn e_setter_getter(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.set_e(0xEE);
    assert_eq!(test_cpu.a(), 0x00);
    assert_eq!(test_cpu.f(), 0x00);
    assert_eq!(test_cpu.b(), 0x00);
    assert_eq!(test_cpu.c(), 0x00);
    assert_eq!(test_cpu.d(), 0x00);
    assert_eq!(test_cpu.e(), 0xEE);
    assert_eq!(test_cpu.h(), 0x00);
    assert_eq!(test_cpu.l(), 0x00);        
}

#[test]
fn h_setter_getter(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.set_h(0xF3);
    assert_eq!(test_cpu.a(), 0x00);
    assert_eq!(test_cpu.f(), 0x00);
    assert_eq!(test_cpu.b(), 0x00);
    assert_eq!(test_cpu.c(), 0x00);
    assert_eq!(test_cpu.d(), 0x00);
    assert_eq!(test_cpu.e(), 0x00);
    assert_eq!(test_cpu.h(), 0xF3);
    assert_eq!(test_cpu.l(), 0x00);        
}

#[test]
fn l_setter_getter(){
    let mut test_cpu: CPU = CPU::default();
    test_cpu.set_l(0x3F);
    assert_eq!(test_cpu.a(), 0x00);
    assert_eq!(test_cpu.f(), 0x00);
    assert_eq!(test_cpu.b(), 0x00);
    assert_eq!(test_cpu.c(), 0x00);
    assert_eq!(test_cpu.d(), 0x00);
    assert_eq!(test_cpu.e(), 0x00);
    assert_eq!(test_cpu.h(), 0x00);
    assert_eq!(test_cpu.l(), 0x3F);        
}

