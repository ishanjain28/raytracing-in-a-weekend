pub struct LinearGradientRectangle;

use crate::{
    types::{HitableList, Ray, Vec3},
    Camera,
    {demos::Chunk, Demo},
};

impl Demo for LinearGradientRectangle {
    fn name(&self) -> &'static str {
        "linear-gradient-rectangle"
    }

    fn render_chunk(
        &self,
        chunk: &mut Chunk,
        _camera: Option<&Camera>,
        _world: Option<&HitableList>,
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

        // -2.0 and 4.0 in lower_left_corner and horizontal respectively
        // because our canvas is in 2:1 ratio
        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);

        let mut offset = 0;
        for j in start_y..start_y + ny {
            for i in start_x..start_x + nx {
                let u = i as f64 / x as f64;
                let v = j as f64 / y as f64;
                let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

                let c = color(ray);
                buffer[offset] = (255.99 * c.r()) as u8;
                buffer[offset + 1] = (255.99 * c.g()) as u8;
                buffer[offset + 2] = (255.99 * c.b()) as u8;
                offset += 4;
            }
        }
    }
}

fn color(ray: Ray) -> Vec3 {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * unit_direction.y() + 1.0;
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
