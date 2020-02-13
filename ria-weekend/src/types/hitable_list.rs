use crate::types::{HitRecord, Hitable, Ray};

pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_rec: Option<HitRecord> = None;
        for obj in &self.list {
            if let Some(l_hit_rec) = obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = l_hit_rec.t;
                hit_rec = Some(l_hit_rec);
            }
        }
        hit_rec
    }
}
