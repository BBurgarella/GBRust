use std::cell::RefCell;
use std::rc::Rc;
use crate::memory::Memory;

pub struct BUS {
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
    pub interrupt_register: u8,

}

impl BUS {
    pub fn at(&self, adress: usize) -> u8 {
        match adress {
            1..=0xFF => {
                if self.io.borrow().at(0xFF50) == 1 {
                    return self.bios.borrow().at(adress)
                } else {
                    return self.rom.borrow().at(adress)
                }
            }
            0x100..=0x7FFF => {
                return self.rom.borrow().at(adress)
            }
            0x8000..=0x9FFF => {
                return self.vram.borrow().at(adress)
            }
            0xC000..=0xDFFF => {
                return self.wram.borrow().at(adress)
            }
            // Mirror ram
            0xE000..=0xFDFF => {
                return self.wram.borrow().at(adress - 0x2000)
            }
            0xFE00..=0xFE9F => {
                return self.oam.borrow().at(adress)
            }
            0xFF00..=0xFF7F => {
                return self.io.borrow().at(adress)
            }
            0xFF80..=0xFFFE => {
                return self.hram.borrow().at(adress)
            }
            0xFFFF => {
                return self.interrupt_register
            }
            _ => {
                return 0
            }

        }

    }

    pub fn set(& mut self, adress: usize, data: u8) -> () {
        match adress {
            0x8000..=0x9FFF => {
                self.vram.borrow_mut().set(adress, data)
            }
            0xC000..=0xDFFF => {
                self.wram.borrow_mut().set(adress, data)
            }
            0xFE00..=0xFE9F => {
                self.oam.borrow_mut().set(adress, data)
            }
            0xFF00..=0xFF7F => {
                self.io.borrow_mut().set(adress, data)
            }
            0xFF80..=0xFFFE => {
                self.hram.borrow_mut().set(adress, data)
            }
            0xFFFF => {
                self.interrupt_register = data
            }
            _ => {
                // pass
            }

        }

    }
}