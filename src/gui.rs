extern crate sdl2; 

use sdl2::render::Canvas;
use sdl2::{VideoSubsystem, Sdl, EventPump};
use sdl2::video::Window;

pub struct GUI {
    pub resolution_x: u32,
    pub resolution_y: u32,
    pub darkest: Vec<u8>,
    pub dark: Vec<u8>,
    pub light: Vec<u8>,
    pub lightest: Vec<u8>,
    pub nocolor: Vec<u8>,
    pub video_subsystem: VideoSubsystem,
    pub main_canvas: Canvas<Window>,
    pub event_pump: EventPump,
}

impl Default for GUI{
    fn default() -> Self{

        let resolution_x: u32 = 160;
        let resolution_y: u32 = 144;
        let darkest: Vec<u8> =  vec![15,56,15];
        let dark: Vec<u8> = vec![48,98,48];
        let light: Vec<u8> = vec![139,172,15];
        let lightest: Vec<u8> = vec![155,188,15];
        let nocolor: Vec<u8> = vec![255,255,255];
        let sdl_context: Sdl = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let main_window = video_subsystem.window("GBRust", resolution_x, resolution_y)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let main_canvas = main_window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Self {resolution_x: resolution_x, resolution_y: resolution_y,
              darkest: darkest, dark: dark, light:light, lightest: lightest,
              nocolor: nocolor, video_subsystem: video_subsystem,
              main_canvas: main_canvas, event_pump: event_pump
            }  
    }
}
