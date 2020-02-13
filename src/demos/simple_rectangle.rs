use crate::{demos::Demo, types::Vec3};

pub struct SimpleRectangle;

impl Demo for SimpleRectangle {
    fn name(&self) -> &'static str {
        "simple_rectangle"
    }

    fn render(&self, buf: &mut [u8], width: usize, height: usize, _samples: u8) {
        let mut offset = 0;
        for j in (0..height).rev() {
            for i in 0..width {
                let color = Vec3::new(i as f64 / width as f64, j as f64 / width as f64, 0.2);

                buf[offset] = (255.99 * color.r()) as u8;
                buf[offset + 1] = (255.99 * color.g()) as u8;
                buf[offset + 2] = (255.99 * color.b()) as u8;

                offset += 4;
            }
        }
    }
}
