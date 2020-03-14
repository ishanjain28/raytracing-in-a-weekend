use {
    crate::{
        demos::{Chunk, Demo},
        types::{
            material::{Lambertian, Metal},
            Hitable, HitableList, Ray, Sphere,
        },
        Camera,
    },
    rand::Rng,
    ultraviolet::vec::Vec3,
};

pub struct Materials;

impl Demo for Materials {
    fn name(&self) -> &'static str {
        "materials"
    }

    fn world(&self) -> Option<HitableList> {
        Some(HitableList {
            list: vec![
                Box::new(Sphere::with_material(
                    Vec3::new(0.0, 0.0, -1.0),
                    0.5,
                    Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
                )),
                Box::new(Sphere::with_material(
                    Vec3::new(0.0, -100.5, -1.0),
                    100.0,
                    Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
                )),
                Box::new(Sphere::with_material(
                    Vec3::new(1.0, 0.0, -1.0),
                    0.5,
                    Box::new(Metal::with_fuzz(Vec3::new(0.8, 0.6, 0.2), 0.3)),
                )),
                Box::new(Sphere::with_material(
                    Vec3::new(-1.0, 0.0, -1.0),
                    0.5,
                    Box::new(Metal::with_fuzz(Vec3::new(0.8, 0.8, 0.8), 0.5)),
                )),
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

                    let ray = camera.get_ray(u, v);
                    color += calc_color(ray, world.unwrap(), 0);
                }

                color /= samples as f32;

                // gamma 2 corrected
                buffer[offset] = (255.99 * color.x.sqrt()) as u8;
                buffer[offset + 1] = (255.99 * color.y.sqrt()) as u8;
                buffer[offset + 2] = (255.99 * color.z.sqrt()) as u8;
                offset += 4;
            }
        }
    }
}

fn calc_color(ray: Ray, world: &HitableList, depth: u32) -> Vec3 {
    if let Some(hit_rec) = world.hit(&ray, 0.001, std::f32::MAX) {
        if depth >= 50 {
            Vec3::new(0.0, 0.0, 0.0)
        } else {
            let material = hit_rec.material.as_ref();
            if let (attenuation, Some(scattered_ray)) = material.unwrap().scatter(&ray, &hit_rec) {
                calc_color(scattered_ray, &world, depth + 1) * attenuation
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        }
    } else {
        let unit_direction = ray.direction().normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}
