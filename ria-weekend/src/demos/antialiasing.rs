use crate::camera::Camera;
use crate::hitable::{HitRecord, Hitable, HitableList};
use crate::shapes;
use crate::types::{Ray, Vec3};
use rand::Rng;

pub struct Antialiasing;

impl crate::Demo for Antialiasing {
    fn name(&self) -> String {
        "antialiasing".to_owned()
    }

    fn render(&self, buf: &mut Vec<u8>, w: usize, h: usize, ns: u8) {
        let list: Vec<Box<dyn Hitable>> = vec![
            Box::new(shapes::Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
            Box::new(shapes::Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
        ];
        let world = HitableList::new(list);
        let camera = Camera::default();
        let mut rng = rand::thread_rng();
    }
}

fn calculate_color(ray: Ray, world: &HitableList) -> Vec3 {
    let mut hit_rec = HitRecord {
        t: 0.0,
        normal: Vec3::new(0.0, 0.0, 0.0),
        point: Vec3::new(0.0, 0.0, 0.0),
    };
    if world.hit(&ray, 0.001, std::f32::MAX, &mut hit_rec) {
        Vec3::new(
            hit_rec.normal.x() + 1.0,
            hit_rec.normal.y() + 1.0,
            hit_rec.normal.z() + 1.0,
        ) * 0.5
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}
