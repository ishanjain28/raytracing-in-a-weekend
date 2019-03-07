use crate::vec3::Vec3;
pub struct PpmExample;

impl crate::Demo for PpmExample {
    fn name(&self) -> String {
        "ppm_example".to_owned()
    }

    fn render(&self, buf: &mut Vec<u8>, w: usize, h: usize) {
        let mut offset = 0;
        for j in (0..h) {
            for i in 0..w {
                let color = Vec3::new((i as f32) / (w as f32), (j as f32) / (h as f32), 0.2);

                let ir = (255.99 * color[0]) as u8;
                let ig = (255.99 * color[1]) as u8;
                let ib = (255.99 * color[2]) as u8;

                buf[offset] = ir;
                buf[offset + 1] = ig;
                buf[offset + 2] = ib;
                buf[offset + 3] = 255;

                offset += 4;
            }
        }
    }
}
