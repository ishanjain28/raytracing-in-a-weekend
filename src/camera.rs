use {crate::types::Ray, rand::Rng, ultraviolet::vec::Vec3};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    lens_radius: f32,

    // position vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    // vertical_fov is the viewable angle from top->bottom
    // look_from is basically camera position
    // look_at is the point where camera is looking
    // v_up is camera's up vector. i.e. it points upwards from the camera
    // orthogonal to look_from - look_at vector
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        vertical_fov: f32,
        aspect: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Self {
        // convert degree to radian
        let angle = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = (angle / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = look_from;
        let w = (look_from - look_at).normalized();
        let u = v_up.cross(w).normalized();
        let v = w.cross(u);

        let lower_left_corner = origin
            - u * focus_distance * half_width
            - v * focus_distance * half_height
            - w * focus_distance;
        let horizontal = u * half_width * focus_distance * 2.0;
        let vertical = v * half_height * focus_distance * 2.0;
        let lens_radius = aperture / 2.0;

        Self {
            lens_radius,
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let mut rng = rand::thread_rng();
        let rd = random_in_unit_disk(&mut rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        )
    }
}

fn random_in_unit_disk(rng: &mut rand::rngs::ThreadRng) -> Vec3 {
    let mut p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);

    while p.dot(p) >= 1.0 {
        p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) * 2.0 - Vec3::new(1.0, 0.0, 0.0);
    }
    p
}
