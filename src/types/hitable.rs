use crate::types::{Material, Ray, Vec3};

pub struct HitRecord<'a> {
    ///  Rays are represented by A + t * B
    ///  where A is the source point and B destination point
    ///  by adjusting t we can move forward/back on the ray
    ///
    ///  t is the point at which a ray intersected another object.
    ///  As in, If we put this value of t in A + t * B equation, We'll get the exact
    ///  point at which a ray intersects some other object
    pub t: f64,
    /// Ray object otherwise is represented by the Source/Destination points
    /// p is what we get when we perform the operation, A + t * B
    /// i.e. A vector from Ray source to the point t
    pub p: Vec3,

    /// unit outward facing normal
    pub normal: Vec3,

    /// material if any of the surface
    pub material: Option<&'a Box<dyn Material>>,
}

pub trait Hitable: Send + Sync {
    fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        None
    }
}
