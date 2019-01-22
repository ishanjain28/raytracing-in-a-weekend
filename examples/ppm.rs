extern crate ria_weekend;

use ria_weekend::ppm;
use std::error::Error;
use std::fs::File;
use std::{io, io::Write};

fn main() {
    let mut height = 1080;
    let mut width = 1920;

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
    let file_name = format!("ppm_sample{}x{}.ppm", width, height);
    let ppm_sample = ppm::create_sample(height, width);

    let mut file = match File::create(&file_name) {
        Ok(file) => file,
        Err(e) => panic!("couldn't create {}: {}", file_name, e.description()),
    };

    match file.write_all(ppm_sample.as_bytes()) {
        Ok(_) => println!("Succesfully wrote to {}", file_name),
        Err(e) => panic!("couldn't write to {}: {}", file_name, e.description()),
    }
}
