extern crate sdl2;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::PixelFormatEnum,
    rect::Rect,
    render::{Canvas, Texture, TextureValueError},
    video::Window,
    EventPump, Sdl,
};
use std::error::Error;
use std::fs::File;
use std::{io, io::Write};

pub struct Demo<'a> {
    pub width: usize,
    pub height: usize,
    pub project_name: &'a str,
    pub canvas: Canvas<Window>,
    pub sdl_ctx: Sdl,
}

impl<'a> Demo<'a> {
    pub fn new(project_name: &str, width: usize, height: usize) -> Result<Demo, String> {
        let sdl_ctx = sdl2::init()?;
        let video_subsys = sdl_ctx.video()?;

        let window = video_subsys
            .window(project_name, width as u32, height as u32)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let mut canvas = window
            .into_canvas()
            .target_texture()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Demo {
            width,
            height,
            canvas,
            project_name,
            sdl_ctx,
        })
    }

    pub fn save_as_ppm(&self, buf: &[u8]) {
        let mut header = format!("P3\n{} {}\n255\n", self.width, self.height);

        let mut file = match File::create(&format!(
            "{}-{}x{}.ppm",
            self.project_name, self.width, self.height
        )) {
            Ok(file) => file,
            Err(e) => panic!("couldn't create {}: {}", self.project_name, e.description()),
        };
        file.write(header.as_bytes())
            .expect("error in writing file header");
        match file.write_all(&buf) {
            Ok(_) => println!("Succesfully wrote to {}", self.project_name),
            Err(e) => panic!(
                "couldn't write to {}: {}",
                self.project_name,
                e.description()
            ),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl_ctx.event_pump()?;

        loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => return Ok(()),
                    _ => {}
                };
            }
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
