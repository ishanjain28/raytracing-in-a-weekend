use crate::{demo::Demo, ray, ray::Ray, vec3::Vec3};

const RADIUS: f32 = 0.8;
pub struct SurfaceNormalSphere;

impl crate::Demo for SurfaceNormalSphere {
    fn name(&self) -> String {
        "surface_normal_sphere".to_owned()
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
                let u = i as f32 / w as f32;
                let v = j as f32 / h as f32;

                let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
                let color = calculate_color(ray);
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

fn calculate_color(ray: Ray) -> Vec3 {
    // center at z=-1. xy axis cuts sphere in half
    // blending parameter
    let t = ray_hit_sphere(Vec3::new(0.0, 0.0, -1.0), RADIUS, &ray);
    if t > 0.0 {
        // For all rays that hit sphere, return red color
        // This will result in a sphere that is red in color
        let n = (ray.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }
    let unit_direction = ray.direction().unit_vector();
    // For rays that don't hit sphere, It'll paint the gradient as the background
    // Linear gradient depends on y
    let t = 0.5 * (unit_direction.y() + 1.0);

    // start color + end color
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn ray_hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    // dot(A + t*B - C, A + t*B - C) =  R*R
    // when expanded we get
    // t * t * dot(B, B) + 2 * t * dot(B, A-C) + dot(A-C, A-C) - R*R = 0

    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant >= 0.0 {
        // return quadratic root
        (-b - discriminant.sqrt()) / (2.0 * a)
    } else {
        -1.0
    }
}
