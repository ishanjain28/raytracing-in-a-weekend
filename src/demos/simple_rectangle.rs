use crate::{
    demos::{Chunk, Demo},
    types::HitableList,
    Camera,
};

pub struct SimpleRectangle;

impl Demo for SimpleRectangle {
    fn name(&self) -> &'static str {
        "simple_rectangle"
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
        let mut offset = 0;

        for j in start_y..start_y + ny {
            for i in start_x..start_x + nx {
                let color = [i as f32 / x as f32, j as f32 / y as f32, 0.2];
                buffer[offset] = (255.99 * color[0]) as u8;
                buffer[offset + 1] = (255.99 * color[1]) as u8;
                buffer[offset + 2] = (255.99 * color[2]) as u8;
                offset += 4;
            }
        }
    }
}
