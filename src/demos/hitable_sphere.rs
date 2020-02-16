use crate::{
    demos::{Chunk, Demo},
    types::{Hitable, HitableList, Ray, Sphere, Vec3},
};
pub struct HitableSphere;

impl Demo for HitableSphere {
    fn name(&self) -> &'static str {
        "Sphere using Hit table"
    }

    fn render_chunk(&self, buf: &mut [u8], meta: Chunk, _samples: u8) {
        let Chunk {
            x,
            y,
            nx,
            ny,
            start_x,
            start_y,
        } = meta;

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

        for j in start_y..start_y + ny {
            for i in start_x..start_x + nx {
                let u = i as f64 / x as f64;
                let v = j as f64 / y as f64;
                let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
                let color = calc_color(ray, &world);

                let offset = ((y - j - 1) * x + i) * 4;
                buf[offset] = (255.99 * color.r()) as u8;
                buf[offset + 1] = (255.99 * color.g()) as u8;
                buf[offset + 2] = (255.99 * color.b()) as u8;
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
