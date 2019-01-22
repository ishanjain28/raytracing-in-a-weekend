use crate::vec3::Vec3;

pub fn create_sample(h: u32, w: u32) -> String {
    let mut buf = format!("P3\n{} {}\n255\n", w, h);
    for j in (0..h).rev() {
        for i in 0..w {
            let color = Vec3::new((i as f32) / (w as f32), (j as f32) / (h as f32), 0.5_f32);

            let ir = (255.99 * color[0]) as u8;
            let ig = (255.99 * color[1]) as u8;
            let ib = (255.99 * color[2]) as u8;
            buf.push_str(&format!("{} {} {}\n", ir, ig, ib));
        }
    }
    buf
}
