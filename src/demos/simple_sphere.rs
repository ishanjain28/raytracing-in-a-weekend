use crate::{
    demos::{Chunk, Demo},
    types::{Ray, Vec3},
};

const RADIUS: f64 = 0.5;

pub struct SimpleSphere;

impl Demo for SimpleSphere {
    fn name(&self) -> &'static str {
        "simple_sphere"
    }

    fn render_chunk(&self, buf: &mut [u8], meta: Chunk, _ns: u8) {
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
        let Chunk {
            x,
            y,
            nx,
            ny,
            start_x,
            start_y,
        } = meta;
        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        // Observer's position
        let origin = Vec3::new(0.0, 0.0, 0.0);

        for j in start_y..start_y + ny {
            for i in start_x..start_x + nx {
                // relative offsets
                // current position to total width/length
                let u = i as f64 / x as f64;
                let v = j as f64 / y as f64;

                let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
                let color = calc_color(ray);

                let offset = ((y - j - 1) * x + i) * 4;
                buf[offset] = (255.99 * color.r()) as u8;
                buf[offset + 1] = (255.99 * color.g()) as u8;
                buf[offset + 2] = (255.99 * color.b()) as u8;
            }
        }
    }
}

fn ray_hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    // For a point to lie on a circle,
    // (x-cx)^2 + (y-cy)^2 + (z-cz)^2 = R * R
    // should hold true. This equation can be rewritten as,
    // dot(p(t)-C, p(t)-C)
    // where p(t) => A + B * t. p(t) represents a point on ray
    // Putting p(t) back in equation
    // dot((A + t*B - C), (A + t*B - C)) = R * R
    // This can be written as,
    // dot((t*B + (A-C)), (t*B + (A-C)) => (t*B + (A-C))^2
    // the expansion of this dot product will result in the same equation
    // i.e. t * t * dot(B,B) + 2 * t * dot(B, A-C) + dot(A-C, A-C) - R * R

    // Vector from circle center to point
    let ac = ray.origin() - center;

    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * ray.direction().dot(&ac);
    let c = ac.dot(&ac) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn calc_color(ray: Ray) -> Vec3 {
    // linear interpolation based on y coordinate
    // top to down
    // z == -1 because the observer is at 0.0,0.0,0.0 and the circle is being
    // drawn in the z = -1 plane. So, The intersection will happen in this plane
    // since circle is 2D
    if ray_hit_sphere(Vec3::new(0.0, 0.0, -1.0), RADIUS, &ray) {
        // For all rays that hit sphere, return red color
        // This will result in a sphere that is red in color
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction().unit_vector();
    // For rays that don't hit sphere, It'll paint the gradient as the background
    // Linear gradient depends on y
    let t = 0.5 * unit_direction.y() + 1.0;

    // start color + end color
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
