use crate::types::{Ray, Vec3};

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, vertical: Vec3, horizontal: Vec3, lower_left_corner: Vec3) -> Camera {
        Camera {
            origin,
            vertical,
            horizontal,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v,
        )
    }
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        }
    }
}
