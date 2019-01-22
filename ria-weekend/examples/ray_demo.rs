extern crate ria_weekend;

use ria_weekend::ray;
use std::error::Error;
use std::fs::File;
use std::{io, io::Write};

fn main() {
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
    let file_name = format!("ray_demo{}x{}.ppm", width, height);
    let mut buf = format!("P3\n{} {}\n255\n", width, height);
    let mut file = match File::create(&file_name) {
        Ok(file) => file,
        Err(e) => panic!("couldn't create {}: {}", file_name, e.description()),
    };
    ray::create_ray_demo(&mut buf, width, height);

    match file.write_all(buf.as_bytes()) {
        Ok(_) => println!("Succesfully wrote to {}", file_name),
        Err(e) => panic!("couldn't write to {}: {}", file_name, e.description()),
    }
}
