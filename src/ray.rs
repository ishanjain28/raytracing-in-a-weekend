use crate::vec3::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub const fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a, b }
    }

    pub const fn origin(&self) -> &Vec3 {
        return &self.a;
    }

    pub const fn direction(&self) -> &Vec3 {
        return &self.b;
    }

    pub const fn point_at_diameter(&self, t: f32) -> Vec3 {
        return self.a  + t * self.b;
    }
}
