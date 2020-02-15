use crate::{
    demos::{Chunk, Demo},
    types::Vec3,
    HORIZONTAL_PARTITION, VERTICAL_PARTITION,
};

pub struct SimpleRectangle;

impl Demo for SimpleRectangle {
    fn name(&self) -> &'static str {
        "simple_rectangle"
    }

    fn render(&self, buf: &mut [u8], width: usize, height: usize, _samples: u8) {
        let nx = width / VERTICAL_PARTITION;
        let ny = height / HORIZONTAL_PARTITION;

        for j in 0..VERTICAL_PARTITION {
            for i in 0..HORIZONTAL_PARTITION {
                let start_y = j * ny;
                let start_x = i * nx;
                let chunk = Chunk {
                    x: width,
                    y: height,
                    nx,
                    ny,
                    start_x,
                    start_y,
                };

                self.render_chunk(buf, chunk);
            }
        }
    }

    fn render_chunk(&self, buf: &mut [u8], meta: Chunk) {
        let Chunk {
            x,
            y,
            nx,
            ny,
            start_x,
            start_y,
        } = meta;

        for j in start_y..start_y + ny {
            for i in start_x..start_x + nx {
                let color = [i as f64 / x as f64, j as f64 / y as f64, 0.2];
                let offset = (j * x + i) * 4;

                buf[offset] = (255.99 * color[0]) as u8;
                buf[offset + 1] = (255.99 * color[1]) as u8;
                buf[offset + 2] = (255.99 * color[2]) as u8;
            }
        }
    }
}
