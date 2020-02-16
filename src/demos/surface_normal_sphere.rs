use crate::{
    demos::{Chunk, Demo},
    types::{Ray, Vec3},
};

const RADIUS: f64 = 0.5;
pub struct SurfaceNormalSphere;

impl Demo for SurfaceNormalSphere {
    fn name(&self) -> &'static str {
        "surface_normal_sphere"
    }

    fn render_chunk(&self, chunk: &mut Chunk, _samples: u8) {
        let x = chunk.x;
        let y = chunk.y;
        let nx = chunk.nx;
        let ny = chunk.ny;
        let start_x = chunk.start_x;
        let start_y = chunk.start_y;
        let buffer = &mut chunk.buffer;

        // Usually, lower_left_corner should've been -1.0,-1.0,-1.0 and
        // horizontal should've been 2.0,0.0,0.0
        // but we are working with a canvas that is 2:1 in size.
        // So, If we had used aforementioned values then, We would've gotten
        // a ellipse instead of a circle
        // Since, we are using the same number of coordinates/values to
        // represent twice as many points in x axis, The generated image is also
        // stretched horizontally.
        // To prevent this from happening, Since our dimensions are in 2:1 ratio,
        // We adjust the lower_left_corner and horizontal values to scale
        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        // Observer position
        let origin = Vec3::new(0.0, 0.0, 0.0);

        let mut offset = 0;

        for j in start_y..start_y + ny {
            for i in start_x..start_x + nx {
                let u = i as f64 / x as f64;
                let v = j as f64 / y as f64;

                let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
                let color = calculate_color(ray);
                let ir = (255.99 * color.r()) as u8;
                let ig = (255.99 * color.g()) as u8;
                let ib = (255.99 * color.b()) as u8;

                buffer[offset] = ir;
                buffer[offset + 1] = ig;
                buffer[offset + 2] = ib;
                offset += 4;
            }
        }
    }
}

fn calculate_color(ray: Ray) -> Vec3 {
    let t = ray_hit_sphere(Vec3::new(0.0, 0.0, -1.0), RADIUS, &ray);
    if t > 0.0 {
        let n = (ray.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }
    let unit_direction = ray.direction().unit_vector();
    // For rays that don't hit sphere, It'll paint the gradient as the background
    // Linear gradient depends on y
    let t = 0.5 * unit_direction.y() + 1.0;

    // start color + end color
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn ray_hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> f64 {
    let pc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * pc.dot(&ray.direction());
    let c = pc.dot(&pc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant >= 0.0 {
        (-b - discriminant.sqrt()) / (2.0 * a)
    } else {
        -1.0
    }
}
