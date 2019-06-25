use crate::{
    hitable::{HitRecord, Hitable},
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        // TODO: I don't yet understand how 4 was canceled from this equation here
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let mut root = (-b - discriminant.sqrt()) / a;
            if root < t_max && root > t_min {
                hit_rec.t = root;
                hit_rec.point = ray.point_at_parameter(hit_rec.t);
                hit_rec.normal = (hit_rec.point - self.center) / self.radius;
                return true;
            }
            root = (-b + discriminant.sqrt()) / a;
            if root < t_max && root > t_min {
                hit_rec.t = root;
                hit_rec.point = ray.point_at_parameter(hit_rec.t);
                hit_rec.normal = (hit_rec.point - self.center) / self.radius;
                return true;
            }
        }
        return false;
    }
}
