use {
    crate::types::{HitRecord, Ray, Vec3},
    rand::Rng,
};

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> (Vec3, Option<Ray>);
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(a: Vec3) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_rec: &HitRecord) -> (Vec3, Option<Ray>) {
        let mut rng = rand::thread_rng();
        let target = hit_rec.p + hit_rec.normal + random_point_in_unit_sphere(&mut rng);
        let scattered_ray = Ray::new(hit_rec.p, target - hit_rec.p);

        (self.albedo, Some(scattered_ray))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo, fuzz: 0.0 }
    }
    pub fn with_fuzz(albedo: Vec3, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> (Vec3, Option<Ray>) {
        let mut rng = rand::thread_rng();

        let reflected_ray = reflect(ray_in.direction().unit_vector(), hit_rec.normal);
        let scattered_ray = Ray::new(
            hit_rec.p,
            reflected_ray + random_point_in_unit_sphere(&mut rng) * self.fuzz,
        );

        if scattered_ray.direction().dot(&hit_rec.normal) > 0.0 {
            (self.albedo, Some(scattered_ray))
        } else {
            (self.albedo, None)
        }
    }
}

pub struct Dielectric {
    reflection_index: f64,
}

impl Dielectric {
    pub fn new(reflection_index: f64) -> Self {
        Self { reflection_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> (Vec3, Option<Ray>) {
        let reflected_ray = reflect(ray_in.direction(), hit_rec.normal);
        // Glass absorbs nothing! So, Attenuation is always going to be 1.0 for this
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut rng = rand::thread_rng();

        let (outward_normal, ni_over_nt, cosine) = if ray_in.direction().dot(&hit_rec.normal) > 0.0
        {
            (
                -hit_rec.normal,
                self.reflection_index,
                (ray_in.direction().dot(&hit_rec.normal) * self.reflection_index)
                    / ray_in.direction().length(),
            )
        } else {
            (
                hit_rec.normal,
                1.0 / self.reflection_index,
                (-ray_in.direction().dot(&hit_rec.normal)) / ray_in.direction().length(),
            )
        };

        if let Some(refracted_ray) = refract(ray_in.direction(), outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.reflection_index);

            if rng.gen::<f64>() < reflect_prob {
                (attenuation, Some(Ray::new(hit_rec.p, reflected_ray)))
            } else {
                (attenuation, Some(Ray::new(hit_rec.p, refracted_ray)))
            }
        } else {
            (attenuation, Some(Ray::new(hit_rec.p, reflected_ray)))
        }
    }
}

// Christophe Schlick's Polynomial approximation to figure out reflectivity as the angle changes
// See Fresnel Equations, https://en.wikipedia.org/wiki/Fresnel_equations
fn schlick(cosine: f64, reflection_index: f64) -> f64 {
    let mut r0 = (1.0 - reflection_index) / (1.0 + reflection_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - normal * incident.dot(&normal) * 2.0
}

// Snell's Law
fn refract(incident: Vec3, normal: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = incident.unit_vector();
    let dt = uv.dot(&normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - normal * dt) * ni_over_nt - normal * discriminant.sqrt())
    } else {
        None
    }
}

fn random_point_in_unit_sphere(rng: &mut rand::rngs::ThreadRng) -> Vec3 {
    let mut point = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0
        - Vec3::new(1.0, 1.0, 1.0);
    while point.sq_len() >= 1.0 {
        point = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0
            - Vec3::new(1.0, 1.0, 1.0);
    }
    point
}
