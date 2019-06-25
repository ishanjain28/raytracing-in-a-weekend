use crate::{
    hitable::{HitRecord, Hitable, HitableList},
    ray::Ray,
    shapes,
    vec3::Vec3,
};

const RADIUS: f32 = 0.5;
pub struct HitableSurfaceNormalSphere;

impl crate::Demo for HitableSurfaceNormalSphere {
    fn name(&self) -> String {
        "hit-table_surface_normal_sphere".to_owned()
    }

    fn render(&self, buf: &mut Vec<u8>, w: usize, h: usize) {
        // in my case, The resolution is 1200x800
        // These numbers are calculated by first calculating the aspect ratio
        // and then just figuring out lower left corner, Width(2 x aspect ratio width)
        // Height(2 x aspect ratio height)
        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);

        let list: Vec<Box<dyn Hitable>> = vec![
            Box::new(shapes::Sphere::new(Vec3::new(0.0, 0.0, -1.0), RADIUS)),
            Box::new(shapes::Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
        ];
        let world = HitableList::new(list);

        let mut offset = 0;
        for j in (0..h).rev() {
            for i in 0..w {
                let u = i as f32 / w as f32;
                let v = j as f32 / h as f32;

                let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

                let color = calculate_color(ray, &world);
                let ir = (255.99 * color.r()) as u8;
                let ig = (255.99 * color.g()) as u8;
                let ib = (255.99 * color.b()) as u8;

                buf[offset] = ir;
                buf[offset + 1] = ig;
                buf[offset + 2] = ib;
                offset += 4;
            }
        }
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
