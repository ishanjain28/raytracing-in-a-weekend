use {
    crate::{
        demos::Demo,
        types::{material, Hitable, HitableList, Ray, Sphere, Vec3},
        Camera,
    },
    rand::Rng,
};

pub struct Materials;

impl Demo for Materials {
    fn name(&self) -> &'static str {
        "Metal Material"
    }

    fn render(&self, buf: &mut [u8], width: usize, height: usize, samples: u8) {
        let world = HitableList {
            list: vec![
                Box::new(Sphere::with_material(
                    Vec3::new(0.0, 0.0, -1.0),
                    0.5,
                    Box::new(material::Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
                )),
                Box::new(Sphere::with_material(
                    Vec3::new(0.0, -100.5, -1.0),
                    100.0,
                    Box::new(material::Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
                )),
                Box::new(Sphere::with_material(
                    Vec3::new(1.0, 0.0, -1.0),
                    0.5,
                    Box::new(material::Metal::new(Vec3::new(0.8, 0.6, 0.2))),
                )),
                Box::new(Sphere::with_material(
                    Vec3::new(-1.0, 0.0, -1.0),
                    0.5,
                    Box::new(material::Metal::new(Vec3::new(0.8, 0.8, 0.8))),
                )),
            ],
        };

        let camera: Camera = Default::default();
        let mut rng = rand::thread_rng();
        let mut offset = 0;

        for j in (0..height).rev() {
            for i in 0..width {
                let mut color = Vec3::new(0.0, 0.0, 0.0);
                for _s in 0..samples {
                    let u = (i as f64 + rng.gen::<f64>()) / width as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / height as f64;

                    let ray = camera.get_ray(u, v);
                    color += calc_color(ray, &world, 0);
                }

                color /= samples as f64;
                // gamma 2 corrected
                buf[offset] = (255.99 * color.r().sqrt()) as u8;
                buf[offset + 1] = (255.99 * color.g().sqrt()) as u8;
                buf[offset + 2] = (255.99 * color.b().sqrt()) as u8;
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
