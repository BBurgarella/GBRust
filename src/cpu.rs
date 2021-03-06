// Implementation of the GB CPU
#[macro_use] mod bit_macros;
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use crate::memory::Memory;
use crate::bus::BUS;
use crate::gbdisassembler::{CPUInstruction, init_instruction_set};

pub struct CPU{
    pub register_af: u16,
    pub register_bc: u16,
    pub register_de: u16,
    pub register_hl: u16,
    pub register_sp: u16,
    pub register_pc: u16,
    //https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
    pub mem_ptr: Rc<RefCell<BUS>>,
    pub instruction_set: Vec<CPUInstruction>,
    pub standbymode: bool,
}


// Implementation of basic functions related to the AF register
impl CPU{

    // as this function is only used to debug I allow dead code
    #[allow(dead_code)]
    pub fn write_program(&mut self, data: Vec<u8>, start_adress: u16){
        let mut writer: usize = start_adress as usize;
        for dat in data {
            self.mem_set(writer, dat);
            writer += 1;
        }

    }

    pub fn mem_read(&self, adress:usize)->u8{
        return self.mem_ptr.borrow().at(adress)
    }

    pub fn mem_set(&mut self, adress:usize, value: u8)->(){
        self.mem_ptr.borrow_mut().set(adress, value);
    }

    // I hope that this function does not take too much time to run
    pub fn ld_u8(&mut self, reg_ident: char)-> u8{
        match reg_ident {
            'a' => {
                self.set_a(self.mem_read((self.register_pc + 1) as usize));
            }
            'f' => {
                self.set_f(self.mem_read((self.register_pc + 1) as usize));
            }
            'b' => {
                self.set_b(self.mem_read((self.register_pc + 1) as usize));
            }
            'c' => {
                self.set_c(self.mem_read((self.register_pc + 1) as usize));
            }
            'd' => {
                self.set_d(self.mem_read((self.register_pc + 1) as usize));
            }
            'e' => {
                self.set_e(self.mem_read((self.register_pc + 1) as usize));
            }
            'h' => {
                self.set_h(self.mem_read((self.register_pc + 1) as usize));
            }
            'l' => {
                self.set_l(self.mem_read((self.register_pc + 1) as usize));
            }
            _ => {
                return 0
            }
        }
        self.register_pc += 2;
        let cycles = 8;
        return cycles
    }

    pub fn ld_u16(&mut self, reg_ident: &str) -> u8 {
        let upper = self.mem_read((self.register_pc + 2) as usize) as u16;
        let lower = self.mem_read((self.register_pc + 1) as usize) as u16;
        match reg_ident {
            "BC" => {
                self.register_bc = (upper << 8) + lower;
            }
            "DE" => {
                self.register_de = (upper << 8) + lower;
            }
            "HL" => {
                self.register_hl = (upper << 8) + lower;
            }
            "SP" => {
                self.register_sp = (upper << 8) + lower;
            }
            _ => {
                return 0
            }
        }
        
        self.register_pc += 3;
        let cycles = 12;     
        return cycles   
    }

    pub fn inc_8bit(&mut self, reg_ident: char) -> u8 {
        let val: usize;
        
        match reg_ident {
            'a' => {
                val = 1 + self.a() as usize;
                self.set_a((val & 0xFF) as u8);
            }
            'b' => {
                val = 1 + self.b() as usize;
                self.set_b((val & 0xFF) as u8);
            }
            'c' => {
                val = 1 + self.c() as usize;
                self.set_c((val & 0xFF) as u8);
            }
            'd' => {
                val = 1 + self.d() as usize;
                self.set_d((val & 0xFF) as u8);
            }
            'e' => {
                val = 1 + self.e() as usize;
                self.set_e((val & 0xFF) as u8);
            }
            'h' => {
                val = 1 + self.h() as usize;
                self.set_h((val & 0xFF) as u8);
            }
            'l' => {
                val = 1 + self.l() as usize;
                self.set_l((val & 0xFF) as u8);
            }
            _ => {
                return 0
            }
        } 

        // flag management
        self.set_subtract_flag(false);
        self.set_zero_flag((val & 0xFF) == 0);
        self.set_halfcarry_flag((val & 0xF00) == 0x100);

        // increment pc and return cycles
        self.register_pc += 1;
        let cycles = 4; 
        return cycles;
    }

    pub fn dec_8bit(&mut self, reg_ident: char) -> u8 {
        let val: isize;
        
        match reg_ident {
            'a' => {
                val = -1 + self.a() as isize;
                // need to match case to handle overflow (or is it called underflow here ?)
                match val>=0 {
                    true => {
                        self.set_a(val as u8);
                    }
                    false => {
                        self.set_a((0xFF + (val+1)) as u8);
                        self.set_halfcarry_flag(true);

                    }
            }
            }
            'b' => {
                val = -1 + self.b() as isize;
                // need to match case to handle overflow (or is it called underflow here ?)
                match val>=0 {
                    true => {
                        self.set_b(val as u8);
                    }
                    false => {
                        self.set_b((0xFF + (val+1)) as u8);
                        self.set_halfcarry_flag(true);

                }
            }
            }
            'c' => {
                val = -1 + self.c()as isize;
                // need to match case to handle overflow (or is it called underflow here ?)
                match val>=0 {
                    true => {
                        self.set_c(val as u8);
                    }
                    false => {
                        self.set_c((0xFF + (val+1)) as u8);
                        self.set_halfcarry_flag(true);
                }
            }
            }
            'd' => {
                val = -1 + self.d()as isize;
                // need to match case to handle overflow (or is it called underflow here ?)
                match val>=0 {
                    true => {
                        self.set_d(val as u8);
                    }
                    false => {
                        self.set_d((0xFF + (val+1)) as u8);
                        self.set_halfcarry_flag(true);
                }
            }
            }
            'e' => {
                val = -1 + self.e()as isize;
                // need to match case to handle overflow (or is it called underflow here ?)
                match val>=0 {
                    true => {
                        self.set_e(val as u8);
                    }
                    false => {
                        self.set_e((0xFF + (val+1)) as u8);
                        self.set_halfcarry_flag(true);
                }
            }
            }
            'h' => {
                val = -1 + self.h()as isize;
                // need to match case to handle overflow (or is it called underflow here ?)
                match val>=0 {
                    true => {
                        self.set_h(val as u8);
                    }
                    false => {
                        self.set_h((0xFF + (val+1)) as u8);
                        self.set_halfcarry_flag(true);
                }
            }
            }
            'l' => {
                val = -1 + self.l()as isize;
                // need to match case to handle overflow (or is it called underflow here ?)
                match val>=0 {
                    true => {
                        self.set_l(val as u8);
                    }
                    false => {
                        self.set_l((0xFF + (val+1)) as u8);
                        self.set_halfcarry_flag(true);
                }
            }
            }
            _ => {
                return 0
            }
        }

        // flag management
        self.set_subtract_flag(true);
        self.set_zero_flag(val == 0);

        // increment pc and return cycles
        self.register_pc += 1;
        let cycles = 4;   
        return cycles;     
    }

    pub fn ld_r_r(&mut self, reg_ident1: char, reg_ident2: char) -> u8 {
        let val: u8;
        let mut cycles: u8;
        match reg_ident2 {
            'a' => {
                val = self.a();
                cycles = 4;
            }
            'b' => {
                val = self.b();
                cycles = 4;
            }
            'c' => {
                val = self.c();
                cycles = 4;
            }
            'd' => {
                val = self.d();
                cycles = 4;
            }
            'e' => {
                val = self.e();
                cycles = 4;
            }
            'h' => {
                val = self.h();
                cycles = 4;
            }
            'l' => {
                val = self.l();
                cycles = 4;
            }
            // p for "pointer", (HL) is implied
            'p' => {
                val = self.mem_read(self.register_hl as usize);
                cycles = 8;
            }
            _ => {
                println!("Invalid register name: {}", reg_ident1);
                return 0
            }
        };
        match reg_ident1 {
            'a' => {
                self.set_a(val);
            }
            'b' => {
                self.set_b(val);
            }
            'c' => {
                self.set_c(val);
            }
            'd' => {
                self.set_d(val);
            }
            'e' => {
                self.set_e(val);
            }
            'h' => {
                self.set_h(val);
            }
            'l' => {
                self.set_l(val);
            }
            'p' => {
                self.mem_set(self.register_hl as usize, val);
                cycles = 8;
            }
            _ => {
                println!("Invalid register name: {}", reg_ident2);
                return 0
            }
        }
        // return the number of cycles
        self.register_pc += 1;
        return cycles;
    }
    
    pub fn add_a_r(&mut self, reg_ident1: char) -> u8 {
        let val: usize;
        let cycles: u8;
        match reg_ident1 {
            'a' => {
                val = self.a() as usize + self.a() as usize;
                self.set_halfcarry_flag((((self.a() & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 4;
            }
            'b' => {
                val = self.b() as usize + self.a() as usize;
                self.set_halfcarry_flag((((self.b() & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 4;
            }
            'c' => {
                val = self.c() as usize + self.a() as usize;
                self.set_halfcarry_flag((((self.c() & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 4;
            }
            'd' => {
                val = self.d() as usize + self.a() as usize;
                self.set_halfcarry_flag((((self.d() & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 4;
            }
            'e' => {
                val = self.e() as usize + self.a() as usize;
                self.set_halfcarry_flag((((self.e() & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 4;
            }
            'h' => {
                val = self.h() as usize + self.a() as usize;
                self.set_halfcarry_flag((((self.h() & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 4;
            }
            'l' => {
                val = self.l() as usize + self.a() as usize;
                self.set_halfcarry_flag((((self.l() & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 4;
            }
            // p for "pointer", (HL) is implied
            'p' => {
                let hl = self.mem_read(self.register_hl as usize) as usize ;
                val = hl + self.a() as usize;
                self.set_halfcarry_flag((((hl as u8 & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 8;
            }
            _ => {
                println!("Invalid register name: {}", reg_ident1);
                return 0
            }
        };
        
        // return the number of cycles
        self.set_subtract_flag(false);
        self.set_zero_flag((val == 0) | ((val & 0x0FF) == 0));
        self.set_carry_flag((val & 0xF00) != 0x00);
        self.set_a((val & 0xFF) as u8);
        self.register_pc += 1;
        return cycles;
    }

    pub fn sub_a_r(&mut self, reg_ident1: char) -> u8 {
        let val: u8;
        let cycles: u8;
        let to_sub: u8;
        match reg_ident1 {
            'a' => {
                to_sub = self.a();
                cycles = 4;
            }
            'b' => {
                to_sub = self.b();
                cycles = 4;
            }
            'c' => {
                to_sub = self.c();
                cycles = 4;
            }
            'd' => {
                to_sub = self.d();
                cycles = 4;
            }
            'e' => {
                to_sub = self.e();
                cycles = 4;
            }
            'h' => {
                to_sub = self.h();
                cycles = 4;
            }
            'l' => {
                to_sub = self.l();
                cycles = 4;
            }
            // p for "pointer", (HL) is implied
            'p' => {
                to_sub = self.mem_read(self.register_hl as usize);
                cycles = 8;
            }
            _ => {
                println!("Invalid register name: {}", reg_ident1);
                return 0
            }
        };

        if (self.a()) < (to_sub) {
            self.set_carry_flag(true);
            val = 0xFF - (to_sub - 1 - self.a());
        } else {
            val = self.a() - to_sub;
            self.set_carry_flag(false);
        }

        self.set_halfcarry_flag((self.a() & 0x0f) < (to_sub & 0x0f));

        // return the number of cycles
        self.set_subtract_flag(true);
        self.set_zero_flag(val == 0);
        self.set_a(val);
        self.register_pc += 1;
        return cycles;
    }

    pub fn adc_a_r(&mut self, reg_ident1: char) -> u8 {
        let val: usize;
        let cycles: u8;
        match reg_ident1 {
            'a' => {
                val = self.a() as usize + self.a() as usize + self.carry_flag() as usize;
                self.set_halfcarry_flag((((self.a() & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 4;
            }
            'b' => {
                val = self.b() as usize + self.a() as usize + self.carry_flag() as usize;
                self.set_halfcarry_flag((((self.b() & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 4;
            }
            'c' => {
                val = self.c() as usize + self.a() as usize + self.carry_flag() as usize;
                self.set_halfcarry_flag((((self.c() & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 4;
            }
            'd' => {
                val = self.d() as usize + self.a() as usize + self.carry_flag() as usize;
                self.set_halfcarry_flag((((self.d() & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 4;
            }
            'e' => {
                val = self.e() as usize + self.a() as usize + self.carry_flag() as usize;
                self.set_halfcarry_flag((((self.e() & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 4;
            }
            'h' => {
                val = self.h() as usize + self.a() as usize + self.carry_flag() as usize;
                self.set_halfcarry_flag((((self.h() & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 4;
            }
            'l' => {
                val = self.l() as usize + self.a() as usize + self.carry_flag() as usize;
                self.set_halfcarry_flag((((self.l() & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 4;
            }
            // p for "pointer", (HL) is implied
            'p' => {
                let hl = self.mem_read(self.register_hl as usize) as usize ;
                val = hl + self.a() as usize + self.carry_flag() as usize;
                self.set_halfcarry_flag((((hl as u8 & 0xf) + (self.a() & 0xf)) & 0x10) == 0x10);
                cycles = 8;
            }
            _ => {
                println!("Invalid register name: {}", reg_ident1);
                return 0
            }
        };
        
        // return the number of cycles
        self.set_subtract_flag(false);
        self.set_zero_flag((val == 0) | ((val & 0x0FF) == 0));
        self.set_carry_flag((val & 0xF00) != 0x00);
        self.set_a((val & 0xFF) as u8);
        self.register_pc += 1;
        return cycles;
    }

    pub fn sbc_a_r(&mut self, reg_ident1: char) -> u8 {
        let val: u8;
        let cycles: u8;
        let to_sub: u8;
        match reg_ident1 {
            'a' => {
                to_sub = self.a();
                cycles = 4;
            }
            'b' => {
                to_sub = self.b();
                cycles = 4;
            }
            'c' => {
                to_sub = self.c();
                cycles = 4;
            }
            'd' => {
                to_sub = self.d();
                cycles = 4;
            }
            'e' => {
                to_sub = self.e();
                cycles = 4;
            }
            'h' => {
                to_sub = self.h();
                cycles = 4;
            }
            'l' => {
                to_sub = self.l();
                cycles = 4;
            }
            // p for "pointer", (HL) is implied
            'p' => {
                to_sub = self.mem_read(self.register_hl as usize);
                cycles = 8;
            }
            _ => {
                println!("Invalid register name: {}", reg_ident1);
                return 0
            }
        };

        if (self.a()) < (to_sub + self.carry_flag()) {
            self.set_carry_flag(true);
            val = 0xFF - (to_sub - 1 - (self.a() - self.carry_flag()));
        } else {
            val = self.a() - (to_sub+self.carry_flag());
            self.set_carry_flag(false);
        }

        self.set_halfcarry_flag((self.a() & 0x0f) < ((to_sub+self.carry_flag()) & 0x0f));

        // return the number of cycles
        self.set_subtract_flag(true);
        self.set_zero_flag(val == 0);
        self.set_a(val);
        self.register_pc += 1;
        return cycles;
    }

    pub fn and_a_r(&mut self, reg_ident1: char) -> u8 {
        let val: u8;
        let cycles: u8;
        match reg_ident1 {
            'a' => {
                val = self.a();
                cycles = 4;
            }
            'b' => {
                val = self.b();
                cycles = 4;
            }
            'c' => {
                val = self.c();
                cycles = 4;
            }
            'd' => {
                val = self.d();
                cycles = 4;
            }
            'e' => {
                val = self.e();
                cycles = 4;
            }
            'h' => {
                val = self.h();
                cycles = 4;
            }
            'l' => {
                val = self.l();
                cycles = 4;
            }
            // p for "pointer", (HL) is implied
            'p' => {
                val = self.mem_read(self.register_hl as usize);
                cycles = 8;
            }
            _ => {
                println!("Invalid register name: {}", reg_ident1);
                return 0
            }
        };

        self.set_a(self.a() & val);
        self.set_f(0b10100000);
        self.set_zero_flag(val == 0);
        self.register_pc += 1;
        return cycles;
    }

    pub fn xor_a_r(&mut self, reg_ident1: char) -> u8 {
        let val: u8;
        let cycles: u8;
        match reg_ident1 {
            'a' => {
                val = self.a();
                cycles = 4;
            }
            'b' => {
                val = self.b();
                cycles = 4;
            }
            'c' => {
                val = self.c();
                cycles = 4;
            }
            'd' => {
                val = self.d();
                cycles = 4;
            }
            'e' => {
                val = self.e();
                cycles = 4;
            }
            'h' => {
                val = self.h();
                cycles = 4;
            }
            'l' => {
                val = self.l();
                cycles = 4;
            }
            // p for "pointer", (HL) is implied
            'p' => {
                val = self.mem_read(self.register_hl as usize);
                cycles = 8;
            }
            _ => {
                println!("Invalid register name: {}", reg_ident1);
                return 0
            }
        };

        self.set_a(self.a() ^ val);
        self.set_f(0b00000000);
        self.set_zero_flag(self.a() == 0);
        self.register_pc += 1;
        return cycles;
    }

    pub fn or_a_r(&mut self, reg_ident1: char) -> u8 {
        let val: u8;
        let cycles: u8;
        match reg_ident1 {
            'a' => {
                val = self.a();
                cycles = 4;
            }
            'b' => {
                val = self.b();
                cycles = 4;
            }
            'c' => {
                val = self.c();
                cycles = 4;
            }
            'd' => {
                val = self.d();
                cycles = 4;
            }
            'e' => {
                val = self.e();
                cycles = 4;
            }
            'h' => {
                val = self.h();
                cycles = 4;
            }
            'l' => {
                val = self.l();
                cycles = 4;
            }
            // p for "pointer", (HL) is implied
            'p' => {
                val = self.mem_read(self.register_hl as usize);
                cycles = 8;
            }
            _ => {
                println!("Invalid register name: {}", reg_ident1);
                return 0
            }
        };

        self.set_a(self.a() | val);
        self.set_f(0b00000000);
        self.set_zero_flag(self.a() == 0);
        self.register_pc += 1;
        return cycles;
    }

    pub fn cp_a_r(&mut self, reg_ident1: char) -> u8 {
        let val: u8;
        let cycles: u8;
        match reg_ident1 {
            'a' => {
                val = self.a();
                cycles = 4;
            }
            'b' => {
                val = self.b();
                cycles = 4;
            }
            'c' => {
                val = self.c();
                cycles = 4;
            }
            'd' => {
                val = self.d();
                cycles = 4;
            }
            'e' => {
                val = self.e();
                cycles = 4;
            }
            'h' => {
                val = self.h();
                cycles = 4;
            }
            'l' => {
                val = self.l();
                cycles = 4;
            }
            // p for "pointer", (HL) is implied
            'p' => {
                val = self.mem_read(self.register_hl as usize);
                cycles = 8;
            }
            _ => {
                println!("Invalid register name: {}", reg_ident1);
                return 0
            }
        };

        self.set_zero_flag(val == self.a());
        self.set_subtract_flag(true);
        self.set_halfcarry_flag((self.a() & 0x0f) < ((val & 0x0f)));
        self.set_carry_flag(self.a() < val);
        self.register_pc += 1;
        return cycles;
    }

    pub fn ret(&mut self, condition: &str) -> u8 {
        let cycles: u8;
        match condition {
            "NZ" => {
                if self.zero_flag() == 0 {
                    // Read from stack
                    let lower: u16 = self.mem_read(self.register_sp as usize) as u16;
                    self.register_sp += 1;
                    let higher: u16 = self.mem_read(self.register_sp as usize) as u16;
                    self.register_sp += 1;
                    
                    // change programm counter accordingly
                    let adress: u16 = lower + (higher << 8);
                    self.register_pc = adress;
                    cycles = 20;
                } else {
                    // simply go to the next instruction
                    cycles = 8;
                    self.register_pc += 1;
                }

            }
            "NC" => {
                if self.carry_flag() == 0 {
                    // Read from stack
                    let lower: u16 = self.mem_read(self.register_sp as usize) as u16;
                    self.register_sp += 1;
                    let higher: u16 = self.mem_read(self.register_sp as usize) as u16;
                    self.register_sp += 1;

                    // change programm counter accordingly
                    let adress: u16 = lower + (higher << 8);
                    self.register_pc = adress;
                    cycles = 20;
                } else {
                    // simply go to the next instruction
                    cycles = 8;
                    self.register_pc += 1;
                }

            }
            "Z" => {
                if self.zero_flag() == 1 {
                    // Read from stack
                    let lower: u16 = self.mem_read(self.register_sp as usize) as u16;
                    self.register_sp += 1;
                    let higher: u16 = self.mem_read(self.register_sp as usize) as u16;
                    self.register_sp += 1;
                    
                    // change programm counter accordingly
                    let adress: u16 = lower + (higher << 8);
                    self.register_pc = adress;
                    cycles = 20;
                } else {
                    // simply go to the next instruction
                    cycles = 8;
                    self.register_pc += 1;
                }

            }
            "C" => {
                if self.carry_flag() == 1 {
                    // Read from stack
                    let lower: u16 = self.mem_read(self.register_sp as usize) as u16;
                    self.register_sp += 1;
                    let higher: u16 = self.mem_read(self.register_sp as usize) as u16;
                    self.register_sp += 1;

                    // change programm counter accordingly
                    let adress: u16 = lower + (higher << 8);
                    self.register_pc = adress;
                    cycles = 20;
                } else {
                    // simply go to the next instruction
                    cycles = 8;
                    self.register_pc += 1;
                }

            }
            "None" => {
                // Read from stack
                let lower: u16 = self.mem_read(self.register_sp as usize) as u16;
                self.register_sp += 1;
                let higher: u16 = self.mem_read(self.register_sp as usize) as u16;
                self.register_sp += 1;

                // change programm counter accordingly
                let adress: u16 = lower + (higher << 8);
                self.register_pc = adress;
                cycles = 16;
            }
            _ => {
                cycles = 0;
                println!("Unknown condition used in return function")
            }
        }
        return cycles
    }

    //    ##############################################
    // =============== registers getters =================
    //    ##############################################
    pub fn a(&self) -> u8{
        return  upper!(self.register_af);
    }

    pub fn f(&self) -> u8{
        return  lower!(self.register_af);
    }

    pub fn b(&self) -> u8{
        return  upper!(self.register_bc);
    }

    pub fn c(&self) -> u8{
        return  lower!(self.register_bc);
    }

    pub fn d(&self) -> u8{
        return  upper!(self.register_de);
    }

    pub fn e(&self) -> u8{
        return  lower!(self.register_de);
    }

    pub fn h(&self) -> u8{
        return  upper!(self.register_hl);
    }

    pub fn l(&self) -> u8{
        return  lower!(self.register_hl);
    }

    pub fn zero_flag(&self) -> u8{
        return (self.f() & 0b10000000) >> 7;
    }

    pub fn subtract_flag(&self) -> u8{
        return (self.f() & 0b01000000) >> 6;
    }

    pub fn half_carry_flag(&self) -> u8{
        return (self.f() & 0b00100000) >> 5;
    }

    pub fn carry_flag(&self) -> u8{
        return (self.f() & 0b00010000) >> 4;
    }

    //    ##############################################
    // =============== registers setters =================
    //    ##############################################
    pub fn set_a(&mut self, value: u8){
        let upperpart: u16 = (value as u16)<<8;
        self.register_af = upperpart + (self.register_af & 0x00FF);
    }

    pub fn set_f(&mut self, value: u8){
        let upperpart: u16 = value as u16;
        self.register_af = upperpart + (self.register_af & 0xFF00);
    }

    pub fn set_b(&mut self, value: u8){
        let upperpart: u16 = (value as u16)<<8;
        self.register_bc = upperpart + (self.register_bc & 0x00FF);
    }

    pub fn set_c(&mut self, value: u8){
        let upperpart: u16 = value as u16;
        self.register_bc = upperpart + (self.register_bc & 0xFF00);
    }

    pub fn set_d(&mut self, value: u8){
        let upperpart: u16 = (value as u16)<<8;
        self.register_de = upperpart + (self.register_de & 0x00FF);
    }

    pub fn set_e(&mut self, value: u8){
        let upperpart: u16 = value as u16;
        self.register_de = upperpart + (self.register_de & 0xFF00);
    }

    pub fn set_h(&mut self, value: u8){
        let upperpart: u16 = (value as u16)<<8;
        self.register_hl = upperpart + (self.register_hl & 0x00FF);
    }

    pub fn set_l(&mut self, value: u8){
        let upperpart: u16 = value as u16;
        self.register_hl = upperpart + (self.register_hl & 0xFF00);
    }

    pub fn set_zero_flag(&mut self, value: bool){
        let flag: u16 = (value as u16) << 7;
        self.register_af = flag + (self.register_af & 0xff7f);
    }

    pub fn set_subtract_flag(&mut self, value: bool){
        let flag: u16 = (value as u16) << 6;
        self.register_af = flag + (self.register_af & 0xffbf);
    }

    pub fn set_carry_flag(&mut self, value: bool){
        let flag: u16 = (value as u16) << 4;
        self.register_af = flag + (self.register_af & 0xffef);
    }

    pub fn set_halfcarry_flag(&mut self, value: bool){
        let flag: u16 = (value as u16) << 5;
        self.register_af = flag + (self.register_af & 0xffdf);
    }

    // here comes the painfull implementation of all the cases
    #[allow(dead_code)]
    pub fn tic(&mut self) -> u8{
        let cycles: u8;
        let op_code = self.mem_read((self.register_pc) as usize);
        match op_code {
            // ---------------------------------------------------
            //                  0x00 to 0x0F
            // ---------------------------------------------------
            // nop
            0x00 => {
                self.register_pc += 1;
                cycles = 4;
            }
            // LD BC.u16
            0x01 => {
                cycles = self.ld_u16("BC");
            }
            // LD (BC), A
            0x02 => {
                self.mem_set(self.register_bc as usize, self.a());
                self.register_pc += 1;
                cycles = 8;
            }
            // INC BC
            0x03 => {
                self.register_bc += 1;
                self.register_pc += 1;
                cycles = 8;
            }
            // INC B
            0x04 => {
                cycles = self.inc_8bit('b');
            }
            // DEC B
            0x05 => {
                cycles = self.dec_8bit('b');
            }
            // LD B, u8
            0x06 => {
                cycles = self.ld_u8('b');
            }
            // RLCA
            0x07 => {
                let last_bit = self.a() >> 7 != 0;
                self.set_carry_flag(last_bit);
                self.set_subtract_flag(false);
                self.set_halfcarry_flag(false);
                self.set_zero_flag(false);
                self.set_a(((self.a() << 1) & 0b11111111) + self.carry_flag());
                self.register_pc += 1;
                cycles = 4;
            }
            // LD, u16, SP
            0x08 => {
                // this instruction is fairly complex, first
                // read the data given by the adress at (low: pc+2, high: pc+1)
                let mut lower = self.mem_read((self.register_pc + 1) as usize);
                let mut upper = self.mem_read((self.register_pc + 2) as usize);
                // then build the adress from the fetched data
                let adress = ((upper as u16) << 8) + lower as u16;
                // use this adress to read memory
                lower = self.mem_read((adress) as usize);
                upper = self.mem_read((adress + 1) as usize);
                // build a new adress from this memory
                let adress =  ((upper as u16) << 8) + lower as u16;
                // and put it in the stack pointer
                self.register_sp = adress as u16;
                // which explains the long cycle
                cycles = 20;
                self.register_pc += 3;
            }
            // ADD HL, BC
            0x09 => {
                let added_val = self.register_hl as usize + self.register_bc as usize;
                self.set_carry_flag(added_val & 0xF0000 != 0);
                // source: https://robdor.com/2016/08/10/gameboy-emulator-half-carry-flag/
                self.set_halfcarry_flag((((self.l() & 0xf) + (self.c() & 0xf)) & 0x10) == 0x10);
                self.register_hl = (added_val & 0xFFFF) as u16;
                self.set_subtract_flag(false);
                cycles = 8;
                self.register_pc += 1;

            }
            // LD A, (BC)
            0x0A => {
                self.set_a(self.mem_read(self.register_bc as usize));
                self.register_pc += 1;
                cycles = 8;
            }
            // DEC BC
            0x0B => {
                self.register_bc -= 1;
                self.register_pc += 1;
                cycles = 8;                
            }
            // INC C
            0x0C => {
                cycles = self.inc_8bit('c')
            }
            // DEC C
            0x0D => {
                cycles = self.dec_8bit('c');
            }
            // LD C, u8
            0x0E => {
                cycles = self.ld_u8('c');
            }
            // RRCA
            0x0F =>{
                let first_bit = self.a() & 0x01 != 0;
                self.set_carry_flag(first_bit);
                self.set_zero_flag(false);
                self.set_subtract_flag(false);
                self.set_halfcarry_flag(false);
                self.set_a(((self.a() >> 1) & 0b11111111) + (self.carry_flag()<< 7));
                self.register_pc += 1;
                cycles = 4;          
            }
            
            // ---------------------------------------------------
            //                  0x10 to 0x1F
            // ---------------------------------------------------
            // STOP
            0x10 => {
                self.standbymode = true;
                self.register_pc += 1;
                cycles = 4;
            }
            // LD DE, u16
            0x11 => {
                cycles = self.ld_u16("DE");
            }
            // LD (DE), A
            0x12 => {
                self.mem_set(self.register_de as usize, self.a());
                self.register_pc += 1;
                cycles = 8;
            }
            // INC DE
            0x13 => {
                self.register_de += 1;
                self.register_pc += 1;
                cycles = 8;                
            }
            // INC D
            0x14 => {
                cycles = self.inc_8bit('d');
            }
            // DEC D
            0x15 => {
                cycles = self.dec_8bit('d');
            }
            // LD D,u8
            0x16 => {
                cycles = self.ld_u8('d');
            }
            // RLA
            0x17 => {
                let last_bit = self.a() >> 7 != 0;
                let old_carry = self.carry_flag();
                self.set_carry_flag(last_bit);
                self.set_subtract_flag(false);
                self.set_halfcarry_flag(false);
                self.set_zero_flag(false);
                self.set_a(((self.a() << 1) & 0b11111111) + old_carry);
                self.register_pc += 1;
                cycles = 4;  
            }
            // JR i8
            0x18 => {
                let mut jump_distance: u16 = self.mem_read((self.register_pc + 1) as usize) as u16;
                // manual casting to i8
                if jump_distance > 128 {
                    jump_distance = 256 - jump_distance;
                    self.register_pc = self.register_pc - jump_distance;
                } else {
                    self.register_pc = self.register_pc + jump_distance;
                }
                cycles = 12;
            }
            // ADD HL, DE
            0x19 => {
                let added_val = self.register_hl as usize + self.register_de as usize;
                self.set_carry_flag(added_val & 0xF0000 != 0);
                // source: https://robdor.com/2016/08/10/gameboy-emulator-half-carry-flag/
                self.set_halfcarry_flag((((self.l() & 0xf) + (self.e() & 0xf)) & 0x10) == 0x10);
                self.register_hl = (added_val & 0xFFFF) as u16;
                self.set_subtract_flag(false);
                cycles = 8;
                self.register_pc += 1;
            }
            // LD, A, (DE)
            0x1A => {
                self.set_a(self.mem_read(self.register_de as usize));
                self.register_pc += 1;
                cycles = 8;
            }
            // DEC DE
            0x1B => {
                self.register_de -= 1;
                self.register_pc += 1;
                cycles = 8;      
            }
            // INC E
            0x1C => {
                self.inc_8bit('e');
                cycles = 4;
            }
            // DEC E
            0x1D => {
                self.dec_8bit('e');
                cycles = 4;
            }
            // LD, E, u8
            0x1e => {
                self.ld_u8('e');
                cycles = 8;
            }
            // RRA
            0x1F => {
                let first_bit = self.a() & 0b1 != 0;
                let old_carry = self.carry_flag();
                self.set_carry_flag(first_bit);
                self.set_halfcarry_flag(false);
                self.set_subtract_flag(false);
                self.set_zero_flag(self.a() == 0);
                self.set_a((self.a() >> 1) + (old_carry << 7));
                self.register_pc += 1;
                cycles = 4;       
            }
            // ---------------------------------------------------
            //                  0x20 to 0x2F
            // ---------------------------------------------------
            // JR, ZZ, i8
            0x20 => {
                if self.zero_flag() == 0 {
                    let mut jump_distance: u16 = self.mem_read((self.register_pc + 1) as usize) as u16;
                    // manual casting to i8
                    if jump_distance > 128 {
                        jump_distance = 256 - jump_distance;
                        self.register_pc = self.register_pc - jump_distance;
                    } else {
                        self.register_pc = self.register_pc + jump_distance;
                    }
                    cycles = 12;
                } else {
                    cycles = 8;
                    self.register_pc += 2;
                }
            }
            // LD, HL, u16
            0x21 => {
                cycles = self.ld_u16("HL");
            }
            // LD (HL+), A
            0x22 => {
                self.mem_set(self.register_hl as usize, self.a());
                self.register_pc += 1;
                self.register_hl += 1;
                cycles = 8;               
            }
            // INC HL
            0x23 => {
                self.register_pc += 1;
                self.register_hl += 1;
                cycles = 8;
            }
            // INC H
            0x24 => {
                self.inc_8bit('h');
                cycles = 4;
            }
            // DEC HL
            0x25 => {
                self.dec_8bit('h');
                cycles = 4;
            }
            // LD, H, u8
            0x26 => {
                self.ld_u8('h');
                cycles = 8;
            }
            // DAA
            0x27 => {
                let mut correction: u16 = 0;
                let value = self.a();
                // lower nibble
                if (self.half_carry_flag() != 0) || (value & 0xf) > 9 {
                    correction += 0x6;
                } 
                // higher nibble
                if (self.carry_flag() != 0) || ((value & 0xf0)>> 4) > 9 {
                    correction += 0x60;
                }
                let corrected_value = value as u16 + correction;
                self.set_a((corrected_value & 0xff) as u8);

                // flags
                self.set_subtract_flag(false);
                self.set_zero_flag((corrected_value & 0xFF) == 0);
                self.set_halfcarry_flag((corrected_value & 0xF00) == 0x100);
                cycles = 4;
            }
            // JR Z, i8
            0x28 => {
                if self.zero_flag() == 1 {
                    let mut jump_distance: u16 = self.mem_read((self.register_pc + 1) as usize) as u16;
                    // manual casting to i8
                    if jump_distance > 128 {
                        jump_distance = 256 - jump_distance;
                        self.register_pc = self.register_pc - jump_distance;
                    } else {
                        self.register_pc = self.register_pc + jump_distance;
                    }
                    cycles = 12;
                } else {
                    cycles = 8;
                    self.register_pc += 2;
                }
            }
            // ADD HL, HL
            0x29 => {
                let added_val = self.register_hl as usize + self.register_hl as usize;
                self.set_carry_flag(added_val & 0xF0000 != 0);
                // source: https://robdor.com/2016/08/10/gameboy-emulator-half-carry-flag/
                self.set_halfcarry_flag((((self.l() & 0xf) + (self.l() & 0xf)) & 0x10) == 0x10);
                self.register_hl = (added_val & 0xFFFF) as u16;
                self.set_subtract_flag(false);
                cycles = 8;
                self.register_pc += 1;
            }
            // LD A,(HL+)
            0x2A => {
                self.set_a(self.mem_read(self.register_hl as usize));
                self.register_pc += 1;
                self.register_hl += 1;
                cycles = 8;    
            }
            // DEC HL
            0x2B => {
                self.register_hl -= 1;
                self.register_pc += 1;
                cycles = 8;   
            }
            // INC L
            0x2C => {
                cycles = self.inc_8bit('l');
            }
            // DEC L
            0x2D => {
                cycles = self.dec_8bit('l');
            }
            // LD L, u8
            0x2E => {
                cycles = self.ld_u8('l');
            }
            // CPL
            0x2F => {
                self.set_a(self.a()^0xFF);
                self.set_subtract_flag(false);
                self.set_halfcarry_flag(false);
                self.register_pc += 1;
                cycles = 4;
            }
            // ---------------------------------------------------
            //                  0x30 to 0x3F
            // ---------------------------------------------------
            // JR, NC, i8
            0x30 => {
                if self.carry_flag() == 0 {
                    let mut jump_distance: u16 = self.mem_read((self.register_pc + 1) as usize) as u16;
                    // manual casting to i8
                    if jump_distance > 128 {
                        jump_distance = 256 - jump_distance;
                        self.register_pc = self.register_pc - jump_distance;
                    } else {
                        self.register_pc = self.register_pc + jump_distance;
                    }
                    cycles = 12;
                } else {
                    cycles = 8;
                    self.register_pc += 2;
                }
            }
            // LD, SP, u16
            0x31 => {
                cycles = self.ld_u16("SP");
            }
            // LDD HL, A
            0x32 => {
                self.mem_set(self.register_hl as usize, self.a());
                self.register_pc += 1;
                self.register_hl -= 1;
                cycles = 8;    
            }
            // INC SP
            0x33 => {
                self.register_sp += 1;
                self.register_pc += 1;
                cycles = 8;                
            }
            // INC (HL)
            0x34 => {
                let val: usize;
                let adress = self.register_hl as usize;
                val = 1 + self.mem_read(adress) as usize;
                self.mem_set(adress, (val & 0xFF) as u8);
        
                // flag management
                self.set_subtract_flag(false);
                self.set_zero_flag((val & 0xFF) == 0);
                self.set_halfcarry_flag((val & 0xF00) == 0x100);
        
                // increment pc and return cycles
                self.register_pc += 1;
                let cycles = 12; 
                return cycles;                
            }
            // DEC HL
            0x35 => {
                let val: isize;
                let adress = self.register_hl as usize;
                val = -1 + self.mem_read(adress) as isize;
                // need to match case to handle overflow (or is it called underflow here ?)
                match val>=0 {
                    true => {
                        self.mem_set(adress,val as u8);
                    }
                    false => {
                        self.mem_set(adress, (0xFF + (val+1)) as u8);
                        self.set_halfcarry_flag(true);
                    }
                }
                // increment pc and return cycles
                self.register_pc += 1;
                let cycles = 12; 
                return cycles;                     
            }
            // LD (HL), u8
            0x36 => {
                self.mem_set(self.register_hl as usize, self.mem_read((self.register_pc + 1) as usize));
                let cycles = 12;
                self.register_pc += 2;
                return cycles;
            }
            // SCF
            0x37 => {
                self.set_carry_flag(true);
                self.set_subtract_flag(false);
                self.set_halfcarry_flag(false);
                self.register_pc += 1;
                let cycles = 4; 
                return cycles;   
            }
            // JR C, i8
            0x38 => {
                if self.carry_flag() == 1 {
                    let mut jump_distance: u16 = self.mem_read((self.register_pc + 1) as usize) as u16;
                    // manual casting to i8
                    if jump_distance > 128 {
                        jump_distance = 256 - jump_distance;
                        self.register_pc = self.register_pc - jump_distance;
                    } else {
                        self.register_pc = self.register_pc + jump_distance;
                    }
                    cycles = 12;
                } else {
                    cycles = 8;
                    self.register_pc += 2;
                }
            }
            // ADD HL, SP
            0x39 => {
                let added_val = self.register_hl as usize + self.register_sp as usize;
                self.set_carry_flag(added_val & 0xF0000 != 0);
                // source: https://robdor.com/2016/08/10/gameboy-emulator-half-carry-flag/
                self.set_halfcarry_flag((((self.l() & 0xf) + (self.l() & 0xf)) & 0x10) == 0x10);
                self.set_subtract_flag(false);
                self.register_hl = (added_val & 0xFFFF) as u16;
                cycles = 8;
                self.register_pc += 1;                
            }
            // LD, A, HL-
            0x3A => {
                self.set_a(self.mem_read(self.register_hl as usize));
                self.register_pc += 1;
                self.register_hl -= 1;
                cycles = 8;
            }
            // DEC SP
            0x3B => {
                self.register_sp -= 1;
                self.register_pc += 1;
                cycles = 8;   
            }
            // INC A 
            0x3C => {
                cycles = self.inc_8bit('a');
            }
            // DEC A 
            0x3D => {
                cycles = self.dec_8bit('a');
            }
            // LD, A, u8
            0x3E => {
                cycles = self.ld_u8('a')
            }
            // CCF
            0x3F => {
                self.set_carry_flag(self.carry_flag() == 0);
                self.set_subtract_flag(false);
                self.set_halfcarry_flag(false);
                self.register_pc += 1;
                let cycles = 4; 
                return cycles;  
            }
            // ---------------------------------------------------
            //                  0x40 to 0x4F
            // ---------------------------------------------------
            // LD, B, B
            0x40 => {
                cycles = self.ld_r_r('b', 'b');
            }
            // LD, B, C
            0x41 => {
                cycles = self.ld_r_r('b', 'c');
            }
            // LD, B, D
            0x42 => {
                cycles = self.ld_r_r('b', 'd');
            }
            // LD, B, E
            0x43 => {
                cycles = self.ld_r_r('b', 'e');
            }
            // LD, B, H
            0x44 => {
                cycles = self.ld_r_r('b', 'h');
            }
            // LD, B, L
            0x45 => {
                cycles = self.ld_r_r('b', 'l');
            }
            // LD, B, (HL)
            0x46 => {
                cycles = self.ld_r_r('b', 'p')
            }
            // LD, B, A
            0x47 => {
                cycles = self.ld_r_r('b', 'a');
            }
           // LD, C, B
            0x48 => {
            cycles = self.ld_r_r('c', 'b');
            }
            // LD, C, C
            0x49 => {
                cycles = self.ld_r_r('c', 'c');
            }
            // LD, C, D
            0x4A => {
                cycles = self.ld_r_r('c', 'd');
            }
            // LD, C, E
            0x4B => {
                cycles = self.ld_r_r('c', 'e');
            }
            // LD, C, H
            0x4C => {
                cycles = self.ld_r_r('c', 'h');
            }
            // LD, C, L
            0x4D => {
                cycles = self.ld_r_r('c', 'l');
            }
            // LD, C, (HL)
            0x4E => {
                cycles = self.ld_r_r('c', 'p')
            }
            // LD, C, A
            0x4F => {
                cycles = self.ld_r_r('c', 'a');
            }
            // ---------------------------------------------------
            //                  0x50 to 0x5F
            // ---------------------------------------------------
            // LD, D, B
            0x50 => {
                cycles = self.ld_r_r('d', 'b');
            }
            // LD, D, C
            0x51 => {
                cycles = self.ld_r_r('d', 'c');
            }
            // LD, D, D
            0x52 => {
                cycles = self.ld_r_r('d', 'd');
            }
            // LD, D, E
            0x53 => {
                cycles = self.ld_r_r('d', 'e');
            }
            // LD, D, H
            0x54 => {
                cycles = self.ld_r_r('d', 'h');
            }
            // LD, D, L
            0x55 => {
                cycles = self.ld_r_r('d', 'l');
            }
            // LD, D, (HL)
            0x56 => {
                cycles = self.ld_r_r('d', 'p')
            }
            // LD, D, A
            0x57 => {
                cycles = self.ld_r_r('d', 'a');
            }
           // LD, E, B
            0x58 => {
            cycles = self.ld_r_r('e', 'b');
            }
            // LD, C, C
            0x59 => {
                cycles = self.ld_r_r('e', 'c');
            }
            // LD, E, D
            0x5A => {
                cycles = self.ld_r_r('e', 'd');
            }
            // LD, E, E
            0x5B => {
                cycles = self.ld_r_r('e', 'e');
            }
            // LD, E, H
            0x5C => {
                cycles = self.ld_r_r('e', 'h');
            }
            // LD, E, L
            0x5D => {
                cycles = self.ld_r_r('e', 'l');
            }
            // LD, E, (HL)
            0x5E => {
                cycles = self.ld_r_r('e', 'p')
            }
            // LD, E, A
            0x5F => {
                cycles = self.ld_r_r('e', 'a');
            }            
            
            // ---------------------------------------------------
            //                  0x60 to 0x6F
            // ---------------------------------------------------
            // LD, h, B
            0x60 => {
                cycles = self.ld_r_r('h', 'b');
            }
            // LD, h, C
            0x61 => {
                cycles = self.ld_r_r('h', 'c');
            }
            // LD, h, D
            0x62 => {
                cycles = self.ld_r_r('h', 'd');
            }
            // LD, h, E
            0x63 => {
                cycles = self.ld_r_r('h', 'e');
            }
            // LD, h, H
            0x64 => {
                cycles = self.ld_r_r('h', 'h');
            }
            // LD, h, L
            0x65 => {
                cycles = self.ld_r_r('h', 'l');
            }
            // LD, h, (HL)
            0x66 => {
                cycles = self.ld_r_r('h', 'p')
            }
            // LD, h, A
            0x67 => {
                cycles = self.ld_r_r('h', 'a');
            }
           // LD, l, B
            0x68 => {
            cycles = self.ld_r_r('l', 'b');
            }
            // LD, l, C
            0x69 => {
                cycles = self.ld_r_r('l', 'c');
            }
            // LD, l, D
            0x6A => {
                cycles = self.ld_r_r('l', 'd');
            }
            // LD, l, E
            0x6B => {
                cycles = self.ld_r_r('l', 'e');
            }
            // LD, l, H
            0x6C => {
                cycles = self.ld_r_r('l', 'h');
            }
            // LD, l, L
            0x6D => {
                cycles = self.ld_r_r('l', 'l');
            }
            // LD, l, (HL)
            0x6E => {
                cycles = self.ld_r_r('l', 'p')
            }
            // LD, l, A
            0x6F => {
                cycles = self.ld_r_r('l', 'a');
            }            
                         
            // ---------------------------------------------------
            //                  0x70 to 0x7F
            // ---------------------------------------------------
            // LD, (HL), B
            0x70 => {
                cycles = self.ld_r_r('p', 'b');
            }
            // LD, (HL), C
            0x71 => {
                cycles = self.ld_r_r('p', 'c');
            }
            // LD, (HL), D
            0x72 => {
                cycles = self.ld_r_r('p', 'd');
            }
            // LD, (HL), E
            0x73 => {
                cycles = self.ld_r_r('p', 'e');
            }
            // LD, (HL), H
            0x74 => {
                cycles = self.ld_r_r('p', 'h');
            }
            // LD, (HL), L
            0x75 => {
                cycles = self.ld_r_r('p', 'l');
            }
            // LD, (HL), (HL)
            0x76 => {
                cycles = 4;
                self.standbymode = true;
                self.register_pc += 1;
            }
            // LD, (HL), A
            0x77 => {
                cycles = self.ld_r_r('p', 'a');
            }
           // LD, A, B
            0x78 => {
            cycles = self.ld_r_r('a', 'b');
            }
            // LD, A, C
            0x79 => {
                cycles = self.ld_r_r('a', 'c');
            }
            // LD, A, D
            0x7A => {
                cycles = self.ld_r_r('a', 'd');
            }
            // LD, A, E
            0x7B => {
                cycles = self.ld_r_r('a', 'e');
            }
            // LD, A, H
            0x7C => {
                cycles = self.ld_r_r('a', 'h');
            }
            // LD, A, L
            0x7D => {
                cycles = self.ld_r_r('a', 'l');
            }
            // LD, A, (HL)
            0x7E => {
                cycles = self.ld_r_r('a', 'p')
            }
            // LD, A, A
            0x7F => {
                cycles = self.ld_r_r('a', 'a');
            }            
            
            // ---------------------------------------------------
            //                  0x80 to 0x8F
            // ---------------------------------------------------
            // ADD A,B
            0x80 => {
                cycles = self.add_a_r('b');
            }
            // ADD A,C
            0x81 => {
                cycles = self.add_a_r('c');
            }
            // ADD A,D
            0x82 => {
                cycles = self.add_a_r('d');
            }
            // ADD A,E
            0x83 => {
                cycles = self.add_a_r('e');
            }
            // ADD A,H
            0x84 => {
                cycles = self.add_a_r('h');
            }
            // ADD A,L
            0x85 => {
                cycles = self.add_a_r('l');
            }
            // ADD A,(HL)
            0x86 => {
                cycles = self.add_a_r('p');
            }
            // ADD A,A
            0x87 => {
                cycles = self.add_a_r('a');
            }
            // ADC A,B
            0x88 => {
                cycles = self.adc_a_r('b');
            }
            // ADC A,C
            0x89 => {
                cycles = self.adc_a_r('c');
            }
            // ADC A,D
            0x8A => {
                cycles = self.adc_a_r('d');
            }
            // ADC A,E
            0x8B => {
                cycles = self.adc_a_r('e');
            }
            // ADC A,H
            0x8C => {
                cycles = self.adc_a_r('h');
            }
            // ADC A,L
            0x8D => {
                cycles = self.adc_a_r('l');
            }
            // ADC A,(HL)
            0x8E => {
                cycles = self.adc_a_r('p');
            }
            // ADC A,A
            0x8F => {
                cycles = self.adc_a_r('a');
            }            
           
            // ---------------------------------------------------
            //                  0x90 to 0x9F
            // ---------------------------------------------------
            // SUB A,B
            0x90 => {
                cycles = self.sub_a_r('b');
            }
            // SUB A,C
            0x91 => {
                cycles = self.sub_a_r('c');
            }
            // SUB A,D
            0x92 => {
                cycles = self.sub_a_r('d');
            }
            // SUB A,E
            0x93 => {
                cycles = self.sub_a_r('e');
            }
            // SUB A,H
            0x94 => {
                cycles = self.sub_a_r('h');
            }
            // SUB A,L
            0x95 => {
                cycles = self.sub_a_r('l');
            }
            // SUB A,(HL)
            0x96 => {
                cycles = self.sub_a_r('p');
            }
            // SUB A,A
            0x97 => {
                cycles = self.sub_a_r('a');
            }
            // SBC A,B
            0x98 => {
                cycles = self.sbc_a_r('b');
            }
            // SBC A,C
            0x99 => {
                cycles = self.sbc_a_r('c');
            }
            // SBC A,D
            0x9A => {
                cycles = self.sbc_a_r('d');
            }
            // SBC A,E
            0x9B => {
                cycles = self.sbc_a_r('e');
            }
            // SBC A,H
            0x9C => {
                cycles = self.sbc_a_r('h');
            }
            // SBC A,L
            0x9D => {
                cycles = self.sbc_a_r('l');
            }
            // SBC A,(HL)
            0x9E => {
                cycles = self.sbc_a_r('p');
            }
            // SBC A,A
            0x9F => {
                cycles = self.sbc_a_r('a');
            }            

            // ---------------------------------------------------
            //                  0xA0 to 0xAF
            // ---------------------------------------------------
            // AND A,B
            0xA0 => {
                cycles = self.and_a_r('b');
            }
            // AND A,C
            0xA1 => {
                cycles = self.and_a_r('c');
            }
            // AND A,D
            0xA2 => {
                cycles = self.and_a_r('d');
            }
            // AND A,E
            0xA3 => {
                cycles = self.and_a_r('e');
            }
            // AND A,H
            0xA4 => {
                cycles = self.and_a_r('h');
            }
            // AND A,L
            0xA5 => {
                cycles = self.and_a_r('l');
            }
            // AND A,(HL)
            0xA6 => {
                cycles = self.and_a_r('p');
            }
            // AND A,A
            0xA7 => {
                cycles = self.and_a_r('a');
            }
            // XOR A,B
            0xA8 => {
                cycles = self.xor_a_r('b');
            }
            // XOR A,C
            0xA9 => {
                cycles = self.xor_a_r('c');
            }
            // XOR A,D
            0xAA => {
                cycles = self.xor_a_r('d');
            }
            // XOR A,E
            0xAB => {
                cycles = self.xor_a_r('e');
            }
            // XOR A,H
            0xAC => {
                cycles = self.xor_a_r('h');
            }
            // XOR A,L
            0xAD => {
                cycles = self.xor_a_r('l');
            }
            // XOR A,(HL)
            0xAE => {
                cycles = self.xor_a_r('p');
            }
            // XOR A,A
            0xAF => {
                cycles = self.xor_a_r('a');
            }
          
            // ---------------------------------------------------
            //                  0xB0 to 0xBF
            // ---------------------------------------------------
            // OR A,B
            0xB0 => {
                cycles = self.or_a_r('b');
            }
            // OR A,C
            0xB1 => {
                cycles = self.or_a_r('c');
            }
            // AND A,D
            0xB2 => {
                cycles = self.or_a_r('d');
            }
            // OR A,E
            0xB3 => {
                cycles = self.or_a_r('e');
            }
            // OR A,H
            0xB4 => {
                cycles = self.or_a_r('h');
            }
            // OR A,L
            0xB5 => {
                cycles = self.or_a_r('l');
            }
            // OR A,(HL)
            0xB6 => {
                cycles = self.or_a_r('p');
            }
            // OR A,A
            0xB7 => {
                cycles = self.or_a_r('a');
            }
            // CP A,B
            0xB8 => {
                cycles = self.cp_a_r('b');
            }
            // CP A,C
            0xB9 => {
                cycles = self.cp_a_r('c');
            }
            // CP A,D
            0xBA => {
                cycles = self.cp_a_r('d');
            }
            // CP A,E
            0xBB => {
                cycles = self.cp_a_r('e');
            }
            // CP A,H
            0xBC => {
                cycles = self.cp_a_r('h');
            }
            // CP A,L
            0xBD => {
                cycles = self.cp_a_r('l');
            }
            // CP A,(HL)
            0xBE => {
                cycles = self.xor_a_r('p');
            }
            // CP A,A
            0xBF => {
                cycles = self.xor_a_r('a');
            }
                      
            
            
            // ---------------------------------------------------
            //                  0xC0 to 0xCF
            // ---------------------------------------------------
            0xC0 => {
                cycles = self.ret("NZ");
            }
            
            
            // ---------------------------------------------------
            //                  0xD0 to 0xDF
            // ---------------------------------------------------
            
            
            
            // ---------------------------------------------------
            //                  0xE0 to 0xEF
            // ---------------------------------------------------
            
            
            
            // ---------------------------------------------------
            //                  0xF0 to 0xFF
            // ---------------------------------------------------
            _ => {
                //println!("Unknown instruction ! {:#04X}", op_code);
                cycles = 0;
            }
        }
        return cycles;
    }

}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let header = " Upper Register |  Value  | Lower Register | Value\n===================================================";
        let af = format!("        A       |  {:#04X}   |        F       | {:#04X}", self.a(), self.f());
        let bc = format!("        B       |  {:#04X}   |        C       | {:#04X}", self.b(), self.c());
        let de = format!("        D       |  {:#04X}   |        E       | {:#04X}", self.d(), self.e());
        let hl = format!("        H       |  {:#04X}   |        L       | {:#04X}", self.h(), self.l());
        let header2 = "===================================================\n     16 bit register      |          Value            \n\
                       ===================================================";
        let pc = format!("            PC            |          {:#06X}", self.register_pc);   
        let sp = format!("            SP            |          {:#06X}", self.register_sp);             
        //let flags: String = format!("Zero: {}, Subtract: {}", self.register_af, self.register_af);
        let header3 = "===================================================\n                        Flags  \n\
                       ===================================================";
        let flags = format!("   Z  |  {}  |  S  |  {}  |  H  |  {} |  C  |  {} |",self.zero_flag(), self.subtract_flag(), self.half_carry_flag(), self.carry_flag());
        write!(f, "{}", format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", header, af, bc, de, hl, header2, pc, sp, header3, flags))
    }
}

// default values for the CPU are with empty registers
// except for the stack pointer SP and the program counter PC
impl Default for CPU{
    fn default() -> Self{

        // bios
        let bios_ptr = Rc::new(RefCell::new(Memory {data: vec![0; 0xFF], offset: 0x0000}));
        // rom
        let rom_ptr  = Rc::new(RefCell::new(Memory {data: vec![0; 0x8000], offset: 0x0000}));
        // vram
        let vram_ptr = Rc::new(RefCell::new(Memory {data: vec![0; 0x2000], offset: 0x8000}));
        // vram
        let wram_ptr = Rc::new(RefCell::new(Memory {data: vec![0; 0x2000], offset: 0xC000}));
        // oam
        let oam_ptr = Rc::new(RefCell::new(Memory {data: vec![0; 0x1E00], offset: 0xFE00}));
        // I/O
        let io_ptr = Rc::new(RefCell::new(Memory {data: vec![0; 0x80], offset: 0xFF00}));
        // High ram
        let hram_ptr = Rc::new(RefCell::new(Memory {data: vec![0; 0x7f], offset: 0xFF80}));

        let bus_ptr =  Rc::new(RefCell::new(BUS {
            bios: Rc::clone(&bios_ptr),
            rom: Rc::clone(&rom_ptr),
            vram: Rc::clone(&vram_ptr),
            wram: Rc::clone(&wram_ptr),
            oam: Rc::clone(&oam_ptr),
            io: Rc::clone(&io_ptr),
            hram: Rc::clone(&hram_ptr),
            interrupt_register: 0}));

        Self {register_af:0x0000, register_bc:0x0000, register_de:0x0000,
              register_hl:0x0000, register_sp:0xFFFE, register_pc:0x0150, 
              mem_ptr: Rc::clone(&bus_ptr),
              instruction_set: init_instruction_set("src/cpu/CPU_Instructions.json"),
              standbymode: false,
            }  
    }
}

#[cfg(test)]
mod registers;
// instructions from 0x00 to 0x0F
mod instructions_0;
// instructions from 0x10 to 0x1F
mod instructions_1;
// instructions from 0x20 to 0x2F
mod instructions_2;
// instructions from 0x30 to 0x3F
mod instructions_3;
// instructions from 0x40 to 0x4F
mod instructions_4;
// instructions from 0x50 to 0x5F
mod instructions_5;
// instructions from 0x60 to 0x6F
mod instructions_6;
// instructions from 0x70 to 0x7F
mod instructions_7;
// instructions from 0x80 to 0x8F
mod instructions_8;
// instructions from 0x90 to 0x9F
mod instructions_9;
// instructions from 0xA0 to 0xAF
mod instructions_a;
// instructions from 0xB0 to 0xBF
mod instructions_b;
// instructions from 0xC0 to 0xCF
mod instructions_c;