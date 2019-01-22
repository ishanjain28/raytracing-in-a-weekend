use std::error::Error;
use std::fs::File;
use std::{io, io::Write};

#[derive(Debug)]
pub struct Demo {
    width: u32,
    height: u32,
    file_name: String,
}

impl Demo {
    pub fn new(file_name: &str) -> Demo {
        let mut height = 100;
        let mut width = 200;

        println!("Enter width and height seperated by space");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error in reading input");

        let input = input
            .trim()
            .split(" ")
            .map(|v| v.parse::<u32>().expect("error in parsing input"))
            .collect::<Vec<u32>>();

        if input.len() >= 2 {
            height = input[1];
            width = input[0];
        }
        let file_name = format!("{}-{}x{}.ppm", file_name, width, height);

        Demo {
            width,
            height,
            file_name,
        }
    }

    pub fn save_as_ppm(&self, buf: String) {
        let mut header = format!("P3\n{} {}\n255\n", self.width, self.height);

        let mut file = match File::create(&self.file_name) {
            Ok(file) => file,
            Err(e) => panic!("couldn't create {}: {}", self.file_name, e.description()),
        };
        file.write(header.as_bytes())
            .expect("error in writing file header");
        match file.write_all(buf.as_bytes()) {
            Ok(_) => println!("Succesfully wrote to {}", self.file_name),
            Err(e) => panic!("couldn't write to {}: {}", self.file_name, e.description()),
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
