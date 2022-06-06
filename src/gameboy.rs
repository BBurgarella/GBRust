use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use crate::gbdisassembler::init_instruction_set;
use crate::memory::Memory;
use crate::cpu::CPU;

pub struct GameBoy {
    pub mem: Rc<RefCell<Memory>>,
    pub cpu: CPU,
}
impl GameBoy{
    #[allow(dead_code)]
    pub fn mem_read(&self, adress:usize)->u8{
        return self.mem.borrow_mut().at(adress)
    }

    pub fn mem_set(&mut self, adress:usize, value: u8)->(){
        self.mem.borrow_mut().set(adress, value);
    }

    pub fn load_rom(&mut self, romfile: &str) -> () {
        let rom_content = fs::read(romfile);
        let mut pointer: usize = 0;
        for data in rom_content.unwrap() {
            self.mem_set(pointer, data);
            pointer += 1;
            if pointer > 0x8000 {
                println!("Warning, the game you are loading features memory bank, which are not implemented yet");
                break;
            }
        }
    }

    #[allow(while_true)]
    pub fn boot(&mut self, romfile: &str) {
        self.load_rom(romfile);
        self.load_rom("src/gameboy/DMG_ROM.bin");
        self.cpu.register_pc = 0x0000;
        let mut quit = false;

        while !quit {
            let cycles = self.cpu.tic();
            if cycles == 0 {
                quit = true;
                println!("\nCPU status: \n{}", self.cpu);
                self.mem.borrow().dump(self.cpu.register_pc, self.cpu.register_pc + 100);
            }
        }
    }
}


impl Default for GameBoy{
    fn default() -> Self{
        let mem_ptr_init = Rc::new(RefCell::new(Memory::default()));
        Self {
            mem: Rc::clone(&mem_ptr_init),
            cpu: CPU {register_af:0x0000, register_bc:0x0000, register_de:0x0000,
                register_hl:0x0000, register_sp:0xFFFE, register_pc:0x0150, 
                mem_ptr:  Rc::clone(&mem_ptr_init),
                instruction_set: init_instruction_set("src/cpu/CPU_Instructions.json"),
                standbymode: false,
            }
        }
    }
}

#[cfg(test)]
mod test;