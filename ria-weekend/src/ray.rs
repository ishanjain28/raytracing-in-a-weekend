use crate::vec3::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a, b }
    }
    pub fn origin(&self) -> Vec3 {
        return self.a;
    }
    pub fn direction(&self) -> Vec3 {
        return self.b;
    }
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        return self.a + self.b * t;
    }
}

// linear interpolation based on y coordinate
// top to down
fn linear_interpolate_y(ray: Ray) -> Vec3 {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // (1.0 - t) * start blend_color + t * end color
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.0, 0.0, 0.0) * t
}

pub fn create_ray_demo(buf: &mut String, w: u32, h: u32) {
    // uses standard cg RHS notation
    // y up, z pointing outwards and x to right
    let lower_left_corner = Vec3::new(-1.0, -1.0, -1.0);
    let horizontal = Vec3::new(2.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    // observer
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..h).rev() {
        for i in 0..w {
            // relative offsets
            // current position to total width/length
            let u = i as f32 / w as f32;
            let v = j as f32 / h as f32;

            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

            let color = linear_interpolate_y(ray);
            let ir = (255.99 * color[0]) as u32;
            let ig = (255.99 * color[1]) as u32;
            let ib = (255.99 * color[2]) as u32;
            let output = format!("{} {} {}\n", ir, ig, ib);
            buf.push_str(&output);
        }
    }
}
