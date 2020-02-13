use crate::types::{Material, Ray, Vec3};

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<&'a Box<dyn Material>>,
}

pub trait Hitable {
    fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        None
    }
}
