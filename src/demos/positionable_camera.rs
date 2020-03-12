use {
    crate::{
        demos::{Chunk, Demo},
        types::{
            material::{Dielectric, Lambertian, Metal},
            Hitable, HitableList, Ray, Sphere, Vec3,
        },
        Camera,
    },
    rand::Rng,
};

pub struct PositionableCamera;

impl Demo for PositionableCamera {
    fn name(&self) -> &'static str {
        "positionable-camera"
    }

    fn world(&self) -> Option<HitableList> {
        let radius = (std::f64::consts::PI / 4.0).cos();
        Some(HitableList {
            list: vec![
                Box::new(Sphere::with_material(
                    Vec3::new(-radius, 0.0, -1.0),
                    radius,
                    Box::new(Lambertian::new(Vec3::new(0.0, 0.0, 1.0))),
                )),
                Box::new(Sphere::with_material(
                    Vec3::new(radius, 0.0, -1.0),
                    radius,
                    Box::new(Metal::with_fuzz(Vec3::new(0.5, 0.5, 0.0), 0.2)),
                )),
                Box::new(Sphere::with_material(
                    Vec3::new(-3.0 * radius, 0.0, -1.0),
                    radius,
                    Box::new(Dielectric::new(1.5)),
                )),
                Box::new(Sphere::with_material(
                    Vec3::new(0.0, -100.5, -1.0),
                    100.0,
                    Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
                )),
            ],
        })
    }

    fn camera(&self, aspect_ratio: f64) -> Option<Camera> {
        let lookfrom = Vec3::new(-2.0, 2.0, 1.0);
        let lookat = Vec3::new(0.0, 0.0, -1.0);
        Some(Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0.0, 1.0, 0.0),
            68.0,
            aspect_ratio,
            0.0,
            1.0,
        ))
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
        let world = world.unwrap();
        let mut rng = rand::thread_rng();
        let mut offset = 0;

        for j in start_y..start_y + ny {
            for i in start_x..start_x + nx {
                let mut color = Vec3::new(0.0, 0.0, 0.0);
                for _s in 0..samples {
                    let u = (i as f64 + rng.gen::<f64>()) / x as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / y as f64;

                    let ray = camera.get_ray(u, v);
                    color += calc_color(ray, &world, 0);
                }
                color /= samples as f64;
                // gamma 2 corrected
                buffer[offset] = (255.99 * color.r().sqrt()) as u8;
                buffer[offset + 1] = (255.99 * color.g().sqrt()) as u8;
                buffer[offset + 2] = (255.99 * color.b().sqrt()) as u8;
                offset += 4;
            }
        }
    }
}

fn calc_color(ray: Ray, world: &HitableList, depth: u32) -> Vec3 {
    if let Some(hit_rec) = world.hit(&ray, 0.001, std::f64::MAX) {
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
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}
