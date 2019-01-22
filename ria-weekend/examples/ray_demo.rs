extern crate ria_weekend;

use ria_weekend::{demo::Demo, ray, ray::Ray, vec3::Vec3};

fn main() {
    let demo = Demo::new("ray_demo");
    let dimensions = demo.dimensions();

    let mut buf = String::new();
    // linear interpolation based on y coordinate
    // top to down
    let linear_interpolate_y = |ray: Ray| -> Vec3 {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        // (1.0 - t) * start blend_color + t * end color
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.0, 0.0, 0.0) * t
    };

    ray::create_ray_demo(&mut buf, dimensions, linear_interpolate_y);

    demo.save_as_ppm(buf);
}
