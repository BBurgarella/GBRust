// Implementation of the GB CPU
#[macro_use] mod bit_macros;
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use crate::memory::Memory;
use crate::gbdisassembler::{CPUInstruction, init_instruction_set};

pub struct CPU{
    pub register_af: u16,
    pub register_bc: u16,
    pub register_de: u16,
    pub register_hl: u16,
    pub register_sp: u16,
    pub register_pc: u16,
    //https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
    pub mem_ptr: Rc<RefCell<Memory>>,
    pub instruction_set: Vec<CPUInstruction>,
}


// Implementation of basic functions related to the AF register
impl CPU{

    #[allow(dead_code)]
    pub fn mem_read(&self, adress:usize)->u8{
        return self.mem_ptr.borrow_mut().at(adress)
    }

    #[allow(dead_code)]
    pub fn mem_set(&mut self, adress:usize, value: u8)->(){
        self.mem_ptr.borrow_mut().set(adress, value);
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
    #[allow(dead_code)]
    pub fn set_a(&mut self, value: u8){
        let upperpart: u16 = (value as u16)<<8;
        self.register_af = upperpart + (self.register_af & 0x00FF);
    }

    #[allow(dead_code)]
    pub fn set_f(&mut self, value: u8){
        let upperpart: u16 = value as u16;
        self.register_af = upperpart + (self.register_af & 0xFF00);
    }

    #[allow(dead_code)]
    pub fn set_b(&mut self, value: u8){
        let upperpart: u16 = (value as u16)<<8;
        self.register_bc = upperpart + (self.register_bc & 0x00FF);
    }

    #[allow(dead_code)]
    pub fn set_c(&mut self, value: u8){
        let upperpart: u16 = value as u16;
        self.register_bc = upperpart + (self.register_bc & 0xFF00);
    }

    #[allow(dead_code)]
    pub fn set_d(&mut self, value: u8){
        let upperpart: u16 = (value as u16)<<8;
        self.register_de = upperpart + (self.register_de & 0x00FF);
    }

    #[allow(dead_code)]
    pub fn set_e(&mut self, value: u8){
        let upperpart: u16 = value as u16;
        self.register_de = upperpart + (self.register_de & 0xFF00);
    }

    #[allow(dead_code)]
    pub fn set_h(&mut self, value: u8){
        let upperpart: u16 = (value as u16)<<8;
        self.register_hl = upperpart + (self.register_hl & 0x00FF);
    }

    #[allow(dead_code)]
    pub fn set_l(&mut self, value: u8){
        let upperpart: u16 = value as u16;
        self.register_hl = upperpart + (self.register_hl & 0xFF00);
    }

    #[allow(dead_code)]
    pub fn set_zero_flag(&mut self, value: bool){
        let flag: u16 = (value as u16) << 7;
        self.register_af = flag + (self.register_af & 0xff7f);
    }

    #[allow(dead_code)]
    pub fn set_subtract_flag(&mut self, value: bool){
        let flag: u16 = (value as u16) << 6;
        self.register_af = flag + (self.register_af & 0xffbf);
    }

    // here comes the painfull implementation of all the cases
    #[allow(dead_code)]
    pub fn tic(&mut self, op_code: u8) -> bool{
        match op_code {
            // nop
            0x00 => {
                self.register_pc += 1;
            }
            // LD BC.u16
            0x01 => {
                let upper = self.mem_read((self.register_pc + 2) as usize) as u16;
                let lower = self.mem_read((self.register_pc + 1) as usize) as u16;
                self.register_bc = (upper << 8) + lower;
                self.register_pc += 3;
            }
            // LD (BC), A
            0x02 => {
                self.mem_set(self.register_bc as usize, self.a());
                self.register_pc += 1;
            }
            // INC BC
            0x03 => {
                self.register_bc += 1;
                self.register_pc += 1;
            }
            // INC B
            0x04 => {
                self.set_b(self.b() + 1);
                self.register_pc += 1;
            }
            // DEC B
            0x05 => {
                self.set_b(self.b() - 1);
                self.register_pc += 1;
            }
            _ => {println!("Unknown instruction ! {:#04X}", op_code); return false
            }
        }
        return true;
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
        Self {register_af:0x0000, register_bc:0x0000, register_de:0x0000,
              register_hl:0x0000, register_sp:0xFFFE, register_pc:0x0150, 
              mem_ptr: Rc::new(RefCell::new(Memory::default())),
              instruction_set: init_instruction_set("src/cpu/CPU_Instructions.json")
            }  
    }
}

#[cfg(test)]
mod test;
mod test_instructions;