use crate::types::{Ray, Vec3};

pub struct Camera {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    // vertical_fov is the viewable angle from top->bottom
    // look_from is basically camera position
    // look_at is the point where camera is looking
    // v_up is camera's up vector. i.e. it points upwards from the camera
    // orthogonal to look_from - look_at vector
    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, vertical_fov: f64, aspect: f64) -> Self {
        // convert degree to radian
        let angle = vertical_fov * std::f64::consts::PI / 180.0;
        let half_height = (angle / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = look_from;
        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let lower_left_corner = origin - u * half_width - v * half_height - w;
        let horizontal = u * half_width * 2.0;
        let vertical = v * half_height * 2.0;

        Self {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}

impl std::default::Default for Camera {
    fn default() -> Self {
        Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            // 2:1 aspect ratio width:height
            2.0,
        )
    }
}
