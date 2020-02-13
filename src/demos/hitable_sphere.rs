use crate::{
    demos::Demo,
    types::{Hitable, HitableList, Ray, Sphere, Vec3},
};
pub struct HitableSphere;

impl Demo for HitableSphere {
    fn name(&self) -> &'static str {
        "Sphere using Hit table"
    }

    fn render(&self, buf: &mut [u8], width: usize, height: usize, _samples: u8) {
        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);

        let world = HitableList {
            list: vec![
                Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
                Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
            ],
        };

        let mut offset = 0;
        for j in (0..height).rev() {
            for i in 0..width {
                let u = i as f64 / width as f64;
                let v = j as f64 / height as f64;
                let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

                let color = calc_color(ray, &world);
                buf[offset] = (255.99 * color.r()) as u8;
                buf[offset + 1] = (255.99 * color.g()) as u8;
                buf[offset + 2] = (255.99 * color.b()) as u8;
                offset += 4;
            }
        }
    }
}

fn calc_color(ray: Ray, world: &HitableList) -> Vec3 {
    if let Some(hit_rec) = world.hit(&ray, 0.0, std::f64::MAX) {
        // It's easier to visualise normals as unit vectors
        // So, This trick of adding 1 to each dimension and then halving
        // the resulting value shifts the normals from -1<->1 range to
        // 0<->1 range
        Vec3::new(
            hit_rec.normal.x() + 1.0,
            hit_rec.normal.y() + 1.0,
            hit_rec.normal.z() + 1.0,
        ) * 0.5
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = unit_direction.y() * 0.5 + 1.0;
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}
