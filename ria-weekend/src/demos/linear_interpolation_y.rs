use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct LinearInterpolationY;

impl crate::Demo for LinearInterpolationY {
    fn name(&self) -> String {
        "linear_interpolation_y".to_owned()
    }

    fn render(&self, buf: &mut Vec<u8>, w: usize, h: usize) {
        // in my case, The resolution is 1200x800
        // These numbers are calculated by first calculating the aspect ratio
        // and then just figuring out lower left corner, Width(2 x aspect ratio width)
        // Height(2 x aspect ratio height)
        let lower_left_corner = Vec3::new(-3.0, -2.0, -1.0);
        let horizontal = Vec3::new(6.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 4.0, 0.0);
        // Observer position
        let origin = Vec3::new(0.0, 0.0, 0.0);

        let mut offset = 0;
        for j in 0..h {
            for i in 0..w {
                // relative offsets
                // current position to total width/length
                let u = i as f32 / w as f32;
                let v = j as f32 / h as f32;

                let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
                let color = calc_color(ray);
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
}

#[inline]
fn calc_color(ray: Ray) -> Vec3 {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // (1.0 - t) * start blend_color + t * end color
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
