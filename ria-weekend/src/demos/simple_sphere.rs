use crate::{ray::Ray, vec3::Vec3};

pub struct SimpleSphere;

impl crate::Demo for SimpleSphere {
    fn name(&self) -> String {
        "simple_sphere".to_owned()
    }

    fn render(&self, buf: &mut Vec<u8>, w: usize, h: usize) {
        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
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
                let ir = (255.99 * color[0]) as u8;
                let ig = (255.99 * color[1]) as u8;
                let ib = (255.99 * color[2]) as u8;

                buf[offset] = ir;
                buf[offset + 1] = ig;
                buf[offset + 2] = ib;
                offset += 4;
            }
        }
    }
}

fn ray_hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
    // dot(A + t*B - C, A + t*B - C) =  R*R
    // when expanded we get
    // t * t * dot(B, B) + 2 * t * dot(B, A-C) + dot(A-C, A-C) - R*R = 0

    // A-C
    let ac = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * ac.dot(&ray.direction());
    let c = ac.dot(&ac) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn calc_color(ray: Ray) -> Vec3 {
    // linear interpolation based on y coordinate
    // top to down
    // center at z=-1. xy axis cuts sphere in half
    if ray_hit_sphere(Vec3::new(0.0, 0.0, 1.0), 0.5, &ray) {
        // For all rays that hit sphere, return red color
        // This will result in a sphere that is red in color
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction().unit_vector();
    // For rays that don't hit sphere, It'll paint the gradient as the background
    // Linear gradient depends on y
    let t = 0.5 * (unit_direction.y() + 1.0);

    // start color + end color
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
