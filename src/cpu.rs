// Implementation of the GB CPU
use std::fmt;

pub struct CPU{
    register_af: u16,
    register_bc: u16,
    register_de: u16,
    register_hl: u16,
    register_sp: u16,
    register_pc: u16,
}

macro_rules! upper {
    ($self:tt.$regi: ident) => {
        (($self.$regi & 0xFF00)>>8) as u8
    }
}

macro_rules! lower {
    ($self:tt.$regi: ident) => {
        ($self.$regi & 0x00FF) as u8
    }
}

// Implementation of basic functions related to the AF register
impl CPU{

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

}


impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let header = " Upper Register |  Value  | Lower Register | Value\n===================================================";
        let af = format!("        A       |  \x1b[94m{:#04X}\x1b[00m   |        F       | \x1b[94m{:#04X}\x1b[00m", self.a(), self.f());
        let bc = format!("        B       |  \x1b[94m{:#04X}\x1b[00m   |        C       | \x1b[94m{:#04X}\x1b[00m", self.b(), self.c());
        let de = format!("        D       |  \x1b[94m{:#04X}\x1b[00m   |        E       | \x1b[94m{:#04X}\x1b[00m", self.d(), self.e());
        let hl = format!("        H       |  \x1b[94m{:#04X}\x1b[00m   |        L       | \x1b[94m{:#04X}\x1b[00m", self.h(), self.l());
        let header2 = "===================================================\n     16 bit register      |          Value            \n\
                       ===================================================";
        let pc = format!("            PC            |          \x1b[94m{:#06X}\x1b[00m", self.register_pc);   
        let sp = format!("            SP            |          \x1b[94m{:#06X}\x1b[00m", self.register_sp);             
        //let flags: String = format!("Zero: {}, Subtract: {}", self.register_af, self.register_af);
        let header3 = "===================================================\n                        Flags  \n\
                       ===================================================";
      let flags = format!("   Z  |  \x1b[94m{}\x1b[00m  |  S  |  \x1b[94m{}\x1b[00m  |  H  |  \x1b[94m{}\x1b[00m |  C  |  \x1b[94m{}\x1b[00m |",self.zero_flag(), self.subtract_flag(), self.half_carry_flag(), self.carry_flag());
        write!(f, "{}", format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", header, af, bc, de, hl, header2, pc, sp, header3, flags))
    }
}

// default values for the CPU are with empty registers
// except for the stack pointer SP and the program counter PC
impl Default for CPU{
    fn default() -> Self{
        Self {register_af:0x0000, register_bc:0x0000, register_de:0x0000,
              register_hl:0x0000, register_sp:0xFFFE, register_pc:0x0150}
    }
}

