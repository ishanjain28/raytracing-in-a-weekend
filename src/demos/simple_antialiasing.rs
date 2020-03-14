use {
    crate::{
        demos::{Chunk, Demo},
        types::{Hitable, HitableList, Ray, Sphere},
        Camera,
    },
    rand::Rng,
    ultraviolet::vec::Vec3,
};
pub struct SimpleAntialiasing;

impl Demo for SimpleAntialiasing {
    fn name(&self) -> &'static str {
        "simple-antialiasing"
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
        camera: Option<&Camera>,
        world: Option<&HitableList>,
        samples: u8,
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
        let camera = camera.unwrap();

        let mut rng = rand::thread_rng();
        let mut offset = 0;

        for j in start_y..start_y + ny {
            for i in start_x..start_x + nx {
                let mut color = Vec3::new(0.0, 0.0, 0.0);
                for _s in 0..samples {
                    let u = (i as f32 + rng.gen::<f32>()) / x as f32;
                    let v = (j as f32 + rng.gen::<f32>()) / y as f32;

                    let r = camera.get_ray(u, v);
                    color += calc_color(r, world.unwrap());
                }
                color /= samples as f32;
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
