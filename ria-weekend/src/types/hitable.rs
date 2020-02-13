use crate::types::{Ray, Vec3};

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, _ray: &Ray, _t_min: f32, _t_max: f32) -> Option<HitRecord> {
        None
    }
}
