mod demo;
mod linear_interpolation_y;
mod ppm_example;
mod ray;
mod simple_sphere;
mod vec3;

use demo::Demo;
use linear_interpolation_y::LinearInterpolationY;
use ppm_example::PpmExample;
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::PixelFormatEnum,
    rect::Rect,
    render::{Canvas, Texture, TextureValueError},
    video::Window,
    EventPump, Sdl,
};
use simple_sphere::SimpleSphere;
use vec3::Vec3;

fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init()?;
    let video_subsys = sdl_ctx.video()?;

    let (mut width, mut height): (usize, usize) = (500, 500);

    let mut window = video_subsys
        .window("Ray tracing in a weekend", width as u32, height as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_ctx.event_pump()?;

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .build()
        .map_err(|e| e.to_string())?;

    // Buffer to store a RGBA framebuffer
    let mut buffer = vec![0; height * width * 4];

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_static(PixelFormatEnum::RGB888, width as u32, height as u32)
        .map_err(|e| e.to_string())?;

    let mut active_demo: Box<Demo> = Box::new(LinearInterpolationY);

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Ok(()),
                Event::KeyUp {
                    keycode: Some(Keycode::Num1),
                    ..
                } => {
                    active_demo = Box::new(PpmExample);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num2),
                    ..
                } => {
                    active_demo = Box::new(LinearInterpolationY);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num3),
                    ..
                } => {
                    active_demo = Box::new(SimpleSphere);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    active_demo.save_as_ppm(&buffer, width, height);
                }
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    width = w as usize;
                    height = h as usize;
                    buffer.resize(width * height * 4, 0);
                    texture = texture_creator
                        .create_texture_static(PixelFormatEnum::RGB888, width as u32, height as u32)
                        .expect("error in resizing texture");
                }
                _ => {}
            };
        }

        active_demo.render(&mut buffer, width, height);
        texture.update(None, &buffer, width * 4);
        canvas.copy(&texture, None, None);
        canvas.present();
    }
}
