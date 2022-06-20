use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use crate::gbdisassembler::init_instruction_set;
use crate::gui::GUI;
use crate::memory::Memory;
use crate::bus::BUS;
use crate::cpu::CPU;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode; 
use std::time::{Duration, Instant};
use std::io::{self, Write};

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
    pub screen: GUI,
}
impl GameBoy{
    #[allow(dead_code)]
    pub fn mem_read(&self, adress:usize)->u8{
        return self.bus.borrow_mut().at(adress)
    }

    #[allow(dead_code)]
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

        self.screen.main_canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.screen.main_canvas.clear();
        self.screen.main_canvas.present();

        let mut i: i32 = 0;
        let mut j: i32 = 0;

        let mut pixel_array: [Point; 160 * 144] = [Point::new(0, 0); 160 * 144];

        for pix in 0..((self.screen.resolution_x*self.screen.resolution_y)-1) {
            pixel_array[pix as usize].x = i;
            pixel_array[pix as usize].y = j;
            
            i = (i + 1) % self.screen.resolution_x as i32;
            if i == 0 {
                j = (j + 1) % self.screen.resolution_y as i32;
            }
        }

        'running: loop {

            let t0 = Instant::now();
            self.screen.main_canvas.clear();
            for event in self.screen.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            // The rest of the game loop goes here...
            self.screen.main_canvas.set_draw_color(Color::RGB(0, 64, (255) as u8)); 
            self.screen.main_canvas.draw_points(&pixel_array[..]).unwrap();
 
            //for pix_id in 1..pixel_array.len() {
            //    self.screen.main_canvas.draw_point(pixel_array[pix_id]).unwrap();
            //}

            
            let cycles = self.cpu.tic();

            if cycles == 0 {
                //println!("\nCPU status: \n{}", self.cpu);
                //self.rom.borrow().dump(self.cpu.register_pc, self.cpu.register_pc + 0x100);
                //break 'running
            }

            self.screen.main_canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 /60));
            print!("FPS: {}\r", 1e6/t0.elapsed().as_micros() as f32);
            io::stdout().flush().unwrap();
        }
        self.rom.borrow().dump(self.cpu.register_pc, self.cpu.register_pc + 0x100);
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
        // bus
        let bus_ptr =  Rc::new(RefCell::new(BUS {
            bios: Rc::clone(&bios_ptr),
            rom: Rc::clone(&rom_ptr),
            vram: Rc::clone(&vram_ptr),
            wram: Rc::clone(&wram_ptr),
            oam: Rc::clone(&oam_ptr),
            io: Rc::clone(&io_ptr),
            hram: Rc::clone(&hram_ptr),
            interrupt_register: 0}));

        // screen
        let screen = GUI::default();

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
            },
            screen: screen,
        }
    }
}

#[cfg(test)]
mod test;