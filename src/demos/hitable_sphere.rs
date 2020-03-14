use {
    crate::{
        demos::{Chunk, Demo},
        types::{Hitable, HitableList, Ray, Sphere},
        Camera,
    },
    ultraviolet::vec::Vec3,
};
pub struct HitableSphere;

impl Demo for HitableSphere {
    fn name(&self) -> &'static str {
        "sphere-using-hit-table"
    }

    fn world(&self) -> Option<HitableList> {
        Some(HitableList {
            list: vec![
                Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
                Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
            ],
        })
    }

    fn render_chunk(
        &self,
        chunk: &mut Chunk,
        _camera: Option<&Camera>,
        world: Option<&HitableList>,
        _samples: u8,
    ) {
        let &mut Chunk {
            x,
            y,
            nx,
            ny,
            start_x,
            start_y,
            ref mut buffer,
        } = chunk;
        let world = world.unwrap();

        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);

        let mut offset = 0;
        for j in start_y..start_y + ny {
            for i in start_x..start_x + nx {
                let u = i as f32 / x as f32;
                let v = j as f32 / y as f32;
                let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
                let color = calc_color(ray, &world);

                buffer[offset] = (255.99 * color.x) as u8;
                buffer[offset + 1] = (255.99 * color.y) as u8;
                buffer[offset + 2] = (255.99 * color.z) as u8;
                offset += 4;
            }
        }
    }
}

fn calc_color(ray: Ray, world: &HitableList) -> Vec3 {
    if let Some(hit_rec) = world.hit(&ray, 0.0, std::f32::MAX) {
        // It's easier to visualise normals as unit vectors
        // So, This trick of adding 1 to each dimension and then halving
        // the resulting value shifts the normals from -1<->1 range to
        // 0<->1 range
        Vec3::new(
            hit_rec.normal.x + 1.0,
            hit_rec.normal.y + 1.0,
            hit_rec.normal.z + 1.0,
        ) * 0.5
    } else {
        let unit_direction = ray.direction().normalized();
        let t = unit_direction.y * 0.5 + 1.0;
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}
