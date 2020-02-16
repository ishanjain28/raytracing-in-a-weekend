use crate::demos::{Chunk, Demo};

pub struct SimpleRectangle;

impl Demo for SimpleRectangle {
    fn name(&self) -> &'static str {
        "simple_rectangle"
    }

    fn render_chunk(&self, chunk: &mut Chunk, samples: u8) {
        let x = chunk.x;
        let y = chunk.y;
        let nx = chunk.nx;
        let ny = chunk.ny;
        let start_x = chunk.start_x;
        let start_y = chunk.start_y;
        let buffer = &mut chunk.buffer;

        let mut offset = 0;

        for j in start_y..start_y + ny {
            for i in start_x..start_x + nx {
                let color = [i as f64 / x as f64, j as f64 / y as f64, 0.2];
                buffer[offset] = (255.99 * color[0]) as u8;
                buffer[offset + 1] = (255.99 * color[1]) as u8;
                buffer[offset + 2] = (255.99 * color[2]) as u8;
                offset += 4;
            }
        }
    }
}
