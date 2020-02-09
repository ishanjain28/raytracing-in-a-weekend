use crate::hitable::{HitRecord, Hitable, HitableList};
use crate::shapes;
use crate::types::{Ray, Vec3};
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub struct Renderer {
    width: usize,
    height: usize,
}

impl Renderer {
    fn new(width: usize, height: usize) -> Renderer {
        Renderer { width, height }
    }

    fn update_dimensions(&mut self, width: usize, height: usize) {}

    fn render<F>(&mut self, buf: &mut Vec<u8>, world: &HitableList, calc_color: F)
    where
        F: Fn(Ray, &HitableList) -> Vec3,
    {
        let w = self.width;
        let h = self.height;
        // in my case, The resolution is 1200x800
        // These numbers are calculated by first calculating the aspect ratio
        // and then just figuring out lower left corner, Width(2 x aspect ratio width)
        // Height(2 x aspect ratio height)
        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);

        let mut offset = 0;
        for j in (0..h).rev() {
            for i in 0..w {
                let u = i as f32 / w as f32;
                let v = j as f32 / h as f32;

                let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

                let color = calc_color(ray, &world);
                let ir = (255.99 * color.r()) as u8;
                let ig = (255.99 * color.g()) as u8;
                let ib = (255.99 * color.b()) as u8;

                buf[offset] = ir;
                buf[offset + 1] = ig;
                buf[offset + 2] = ib;
                offset += 4;
            }
        }
    }

    fn save_as_ppm(&self, buf: &[u8], name: &str) {
        let (width, height) = (self.width, self.height);
        let header = format!("P3\n{} {}\n255\n", width, height);

        let mut file = match File::create(&format!("{}-{}x{}.ppm", name, width, height)) {
            Ok(file) => file,
            Err(e) => panic!("couldn't create {}: {}", name, e),
        };
        file.write(header.as_bytes())
            .expect("error in writing file header");

        for i in buf.chunks(4) {
            match file.write(format!("{} {} {}\n", i[0], i[1], i[2]).as_bytes()) {
                Ok(_) => (),
                Err(e) => panic!("couldn't write to {}: {}", name, e),
            }
        }
    }
}
