// Main file of GBRust
mod cpu;


fn main() {
    let mut test_cpu: cpu::CPU = cpu::CPU::default();
    test_cpu.set_a(0xAA);
    test_cpu.set_f(0b10110000);
    test_cpu.set_b(0xBB);
    test_cpu.set_c(0xCC);
    test_cpu.set_d(0xDD);
    test_cpu.set_e(0xEE);
    test_cpu.set_h(0xF3);
    test_cpu.set_l(0x3F);
    println!("{}", test_cpu);
}
