use crate::types::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a, b }
    }
    #[inline]
    pub const fn origin(&self) -> Vec3 {
        self.a
    }
    #[inline]
    pub const fn direction(&self) -> Vec3 {
        self.b
    }
    #[inline]
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.a + self.b * t
    }
}
