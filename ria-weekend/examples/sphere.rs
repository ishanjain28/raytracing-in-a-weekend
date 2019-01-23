extern crate ria_weekend;

use ria_weekend::{demo::Demo, ray, ray::Ray, vec3::Vec3};

fn main() {
    let demo = Demo::new("single_sphere_demo");
    let dimensions = demo.dimensions();

    let mut buf = String::new();
    // linear interpolation based on y coordinate
    // top to down
    let color = |ray: Ray| -> Vec3 {
        // center at z=-1. xy axis cuts sphere in half
        if ray_hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &ray) {
            // For all rays that hit sphere, return red color
            // This will result in a sphere that is red in color
            return Vec3::new(1.0, 0.0, 0.0);
        }
        let unit_direction = ray.direction().unit_vector();
        // For rays that don't hit sphere, It'll paint the gradient as the background
        // Linear gradient depends on y
        let t = 0.5 * (unit_direction.y() + 1.0);

        // start color + end color
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.0, 0.0, 0.0) * t
    };

    ray::create_ray_demo(&mut buf, dimensions, color);

    demo.save_as_ppm(buf);
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
