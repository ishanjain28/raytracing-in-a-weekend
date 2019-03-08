use std::error::Error;
use std::fs::File;
use std::io::Write;

pub trait Demo {
    fn render(&self, buf: &mut Vec<u8>, width: usize, height: usize);
    fn name(&self) -> String;

    fn save_as_ppm(&self, buf: &[u8], width: usize, height: usize) {
        let header = format!("P3\n{} {}\n255\n", width, height);

        let mut file = match File::create(&format!("{}-{}x{}.ppm", self.name(), width, height)) {
            Ok(file) => file,
            Err(e) => panic!("couldn't create {}: {}", self.name(), e.description()),
        };
        file.write(header.as_bytes())
            .expect("error in writing file header");

        for i in buf.chunks(4) {
            match file.write(format!("{} {} {}\n", i[0], i[1], i[2]).as_bytes()) {
                Ok(_) => (),
                Err(e) => panic!("couldn't write to {}: {}", self.name(), e.description()),
            }
        }
    }
}
