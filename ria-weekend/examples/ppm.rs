extern crate ria_weekend;

use ria_weekend::{ppm, demo::Demo};

fn main() {
    let demo = Demo::new("ppm_sample");
    let dimensions = demo.dimensions();
    let mut buf = String::new();

    ppm::create_sample(&mut buf, dimensions);

    demo.save_as_ppm(buf);
}
