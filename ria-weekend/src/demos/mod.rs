mod hitable_sphere;
mod linear_gradient_rectangle;
mod simple_antialiasing;
mod simple_rectangle;
mod simple_sphere;
mod surface_normal_sphere;

pub use hitable_sphere::HitableSphere;
pub use linear_gradient_rectangle::LinearGradientRectangle;
pub use simple_antialiasing::SimpleAntialiasing;
pub use simple_rectangle::SimpleRectangle;
pub use simple_sphere::SimpleSphere;
pub use surface_normal_sphere::SurfaceNormalSphere;

use std::{fs::File, io::Write};

pub trait Demo {
    fn render(&self, buf: &mut [u8], width: usize, height: usize, samples: u8);
    fn name(&self) -> &'static str;

    fn save_as_ppm(&self, buf: &[u8], width: usize, height: usize) {
        let header = format!("P3\n{} {}\n255\n", width, height);

        let mut file = match File::create(&format!("{}-{}x{}.ppm", self.name(), width, height)) {
            Ok(file) => file,
            Err(e) => panic!("couldn't create {}: {}", self.name(), e),
        };
        file.write_all(header.as_bytes())
            .expect("error in writing file header");

        for i in buf.chunks(4) {
            match file.write_all(format!("{} {} {}\n", i[0], i[1], i[2]).as_bytes()) {
                Ok(_) => (),
                Err(e) => panic!("couldn't write to {}: {}", self.name(), e),
            }
        }
    }
}
