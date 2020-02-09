use crate::types::{Ray, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32, hit_rec: &mut HitRecord) -> bool;
}

pub struct HitableList {
    inner: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(items: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList { inner: items }
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_rec: &mut HitRecord) -> bool {
        let mut temp_hit_rec: HitRecord = HitRecord {
            t: 0.0,
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
        };
        let mut hit_anything = false;
        let mut closest_to_far = t_max;

        for obj in &self.inner {
            if obj.hit(&ray, t_min, closest_to_far, &mut temp_hit_rec) {
                hit_anything = true;
                closest_to_far = hit_rec.t;
                hit_rec.point = temp_hit_rec.point;
                hit_rec.t = temp_hit_rec.t;
                hit_rec.normal = temp_hit_rec.normal;
            }
        }
        hit_anything
    }
}
