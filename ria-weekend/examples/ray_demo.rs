extern crate ria_weekend;
extern crate sdl2;

use ria_weekend::{demo::Demo, ray, ray::Ray, vec3::Vec3};
use sdl2::{pixels, rect::Rect};

fn main() {
    let (width, height): (usize, usize) = (500, 500);
    let mut demo = Demo::new("ray_demo", width, height).expect("error occurred");

    let texture_creator = demo.canvas.texture_creator();

    // linear interpolation based on y coordinate
    // top to down
    let linear_interpolate_y = |ray: Ray| -> Vec3 {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        // (1.0 - t) * start blend_color + t * end color
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.0, 0.0, 0.0) * t
    };
    let mut texture = texture_creator
        .create_texture_streaming(pixels::PixelFormatEnum::RGB888, width as u32, height as u32)
        .map_err(|e| e.to_string())
        .expect("error in creating texture");
    let mut buf = Vec::new();
    ray::create_ray_demo(
        &mut buf,
        (width as u32, height as u32),
        linear_interpolate_y,
    );

    texture.update(Rect::new(0, 0, width as u32, height as u32), &buf, 20);

    demo.canvas.copy(&texture, None, None).unwrap();
    demo.canvas.present();
    demo.save_as_ppm(&buf);
    demo.start().unwrap();
}
