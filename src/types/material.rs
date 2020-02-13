use {
    crate::types::{HitRecord, Ray, Vec3},
    rand::Rng,
};
pub trait Material {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> (Vec3, Option<Ray>);
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_rec: &HitRecord) -> (Vec3, Option<Ray>) {
        let mut rng = rand::thread_rng();
        let target = hit_rec.p + hit_rec.normal + random_point_in_unit_space(&mut rng);
        let scattered_ray = Ray::new(hit_rec.p, target - hit_rec.p);

        (self.albedo, Some(scattered_ray))
    }
}

impl Lambertian {
    pub fn new(a: Vec3) -> Self {
        Self { albedo: a }
    }
}

fn random_point_in_unit_space(rng: &mut rand::rngs::ThreadRng) -> Vec3 {
    let mut point = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0
        - Vec3::new(1.0, 1.0, 1.0);
    while point.sq_len() >= 1.0 {
        point = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0
            - Vec3::new(1.0, 1.0, 1.0);
    }
    point
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> (Vec3, Option<Ray>) {
        let reflected_ray = reflect(ray_in.direction().unit_vector(), hit_rec.normal);
        let scattered_ray = Ray::new(hit_rec.p, reflected_ray);

        if scattered_ray.direction().dot(&hit_rec.normal) > 0.0 {
            (self.albedo, Some(scattered_ray))
        } else {
            (self.albedo, None)
        }
    }
}

fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - normal * incident.dot(&normal) * 2.0
}
