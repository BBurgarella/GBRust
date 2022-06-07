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
                val = 1 + self.b() as usize;
                self.set_b((val & 0xFF) as u8);
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
        match reg_ident2 {
            'a' => {
                val = self.a();
            }
            'b' => {
                val = self.b();
            }
            'c' => {
                val = self.c();
            }
            'd' => {
                val = self.d();
            }
            'e' => {
                val = self.e();
            }
            'h' => {
                val = self.h();
            }
            'l' => {
                val = self.l();
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
            _ => {
                println!("Invalid register name: {}", reg_ident2);
                return 0
            }
        }
        // return the number of cycles
        let cycles = 4;
        self.register_pc += 1;
        return cycles;
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
            
            // ---------------------------------------------------
            //                  0x20 to 0x2F
            // ---------------------------------------------------
            
            
            // ---------------------------------------------------
            //                  0x30 to 0x3F
            // ---------------------------------------------------
           
            
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
            _ => {
                println!("Unknown instruction ! {:#04X}", op_code);
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
mod test;
mod test_instruction_0;
mod test_instruction_1;
mod test_instruction_4;