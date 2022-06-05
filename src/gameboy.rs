use std::cell::RefCell;
use std::rc::Rc;
use crate::memory::Memory;
use crate::cpu::CPU;

pub struct GameBoy {
    pub mem: Rc<RefCell<Memory>>,
    pub cpu: CPU,
}
impl GameBoy{
    pub fn mem_read(&self, adress:usize)->u8{
        return self.mem.borrow_mut().at(adress)
    }
    
    pub fn mem_set(&mut self, adress:usize, value: u8)->(){
        self.mem.borrow_mut().set(adress, value);
    }
}


impl Default for GameBoy{
    fn default() -> Self{
        let mem_ptr_init = Rc::new(RefCell::new(Memory::default()));
        Self {
            mem: Rc::clone(&mem_ptr_init),
            cpu: CPU {register_af:0x0000, register_bc:0x0000, register_de:0x0000,
                register_hl:0x0000, register_sp:0xFFFE, register_pc:0x0150, mem_ptr:  Rc::clone(&mem_ptr_init)}
        }
    }
}
