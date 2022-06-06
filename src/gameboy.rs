use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use crate::gbdisassembler::init_instruction_set;
use crate::memory::Memory;
use crate::bus::BUS;
use crate::cpu::CPU;

pub struct GameBoy {
    // bios
    pub bios: Rc<RefCell<Memory>>,
    // ROM - 0000 to 7FFF
    pub rom: Rc<RefCell<Memory>>,
    // VRAM - 8000 to 9FFF
    pub vram: Rc<RefCell<Memory>>,
    // WRAM - C000 to DFFF
    pub wram: Rc<RefCell<Memory>>,
    // Sprite attribute (OAM) FE00 - FE9F
    pub oam: Rc<RefCell<Memory>>,
    // I/O registers FF00 to FF7F
    pub io: Rc<RefCell<Memory>>,
    // High ram FF80 to FFFE
    pub hram: Rc<RefCell<Memory>>,
    pub cpu: CPU,
    pub bus: Rc<RefCell<BUS>>,
}
impl GameBoy{
    #[allow(dead_code)]
    pub fn mem_read(&self, adress:usize)->u8{
        return self.bus.borrow_mut().at(adress)
    }

    pub fn mem_set(&mut self, adress:usize, value: u8)->(){
        self.bus.borrow_mut().set(adress, value);
    }

    pub fn load_rom(&mut self, romfile: &str) -> () {
        let rom_content = fs::read(romfile);
        let mut pointer: usize = 0;
        for data in rom_content.unwrap() {
            self.rom.borrow_mut().set(pointer, data);
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
                self.rom.borrow().dump(self.cpu.register_pc, self.cpu.register_pc + 0x100);
            }
        }
    }
}


impl Default for GameBoy{
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

        Self {
            bios: Rc::clone(&bios_ptr),
            rom: Rc::clone(&rom_ptr),
            vram: Rc::clone(&vram_ptr),
            wram: Rc::clone(&wram_ptr),
            oam: Rc::clone(&oam_ptr),
            io: Rc::clone(&io_ptr),
            hram: Rc::clone(&hram_ptr),
            bus: Rc::clone(&bus_ptr),
            cpu: CPU {register_af:0x0000, register_bc:0x0000, register_de:0x0000,
                register_hl:0x0000, register_sp:0xFFFE, register_pc:0x0150, 
                mem_ptr:  Rc::clone(&bus_ptr),
                instruction_set: init_instruction_set("src/cpu/CPU_Instructions.json"),
                standbymode: false,
            }
        }
    }
}

#[cfg(test)]
mod test;