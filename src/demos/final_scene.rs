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

pub struct FinalScene;

impl Demo for FinalScene {
    fn name(&self) -> &'static str {
        "final-scene"
    }

    fn world(&self) -> Option<HitableList> {
        let mut world = HitableList {
            list: Vec::with_capacity(500),
        };

        world.push(Box::new(Sphere::with_material(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
        )));

        let mut rng = rand::thread_rng();
        let radius = 0.2;
        let l = Vec3::new(4.0, 0.2, 0.0);

        for a in -11..11 {
            let a = a as f64;
            for b in -11..11 {
                let b = b as f64;
                let choose_material_probability = rng.gen::<f64>();
                let center = Vec3::new(a + 0.9 * rng.gen::<f64>(), 0.2, b + 0.9 * rng.gen::<f64>());

                if (center - l).length() > 0.9 {
                    if choose_material_probability < 0.8 {
                        // diffuse material
                        world.push(Box::new(Sphere::with_material(
                            center,
                            radius,
                            Box::new(Lambertian::new(Vec3::new(
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                            ))),
                        )));
                    } else if choose_material_probability < 0.95 {
                        // metal material
                        world.push(Box::new(Sphere::with_material(
                            center,
                            radius,
                            Box::new(Metal::with_fuzz(
                                Vec3::new(
                                    (1.0 + rng.gen::<f64>()) * 0.5,
                                    (1.0 + rng.gen::<f64>()) * 0.5,
                                    (1.0 + rng.gen::<f64>()) * 0.5,
                                ),
                                0.5 * rng.gen::<f64>(),
                            )),
                        )));
                    } else {
                        // glass material
                        world.push(Box::new(Sphere::with_material(
                            center,
                            radius,
                            Box::new(Dielectric::new(1.5)),
                        )));
                    }
                }
            }
        }

        world.push(Box::new(Sphere::with_material(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Box::new(Dielectric::new(1.5)),
        )));
        world.push(Box::new(Sphere::with_material(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
        )));
        world.push(Box::new(Sphere::with_material(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Box::new(Metal::with_fuzz(Vec3::new(0.7, 0.6, 0.5), 0.0)),
        )));

        println!("world size = {}", world.list.len());
        Some(world)
    }

    fn camera(&self, aspect_ratio: f64) -> Option<Camera> {
        let lookfrom = Vec3::new(13.0, 2.0, 3.0);
        let lookat = Vec3::new(0.0, 0.0, 0.0);
        let camera = Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
            0.1,
            10.0,
        );
        Some(camera)
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
