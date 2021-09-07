use std::fs::File;
use std::io::prelude::*;
use rust_tracer::Vec3;

fn main() {
    println!("Hello, world!");
    write_pixels();
}

fn write_pixels() {

    let img_height: f64 = 256.0;
    let img_width: f64 = 256.0;

    let mut image_string = String::new();

    image_string.push_str(format!("P3\n{} {}\n255\n", img_height, img_width).as_str());

    for i in (0..=((img_height.round() as i64)-1)).rev() {
        print!("\rScanlines remaining: {}", i);
        for j in 0..=((img_width.round() as i64)-1) {

            let colour = Vec3 {
                x: i as f64 / (img_width - 1.0),
                y: j as f64 / (img_height - 1.0),
                z: 0.5,
            };

            image_string.push_str(write_colour(colour).as_str())
        }
    }
    print!("\n");
    let path = "image.ppm";

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create image file {}", why),
        Ok(file) => file,
    };

    write_file(image_string.as_str(), &mut file);
}

fn write_colour(col: Vec3) -> String {
    format!("{} {} {}\n", col.x * 255.0, col.y * 255.0, col.z * 255.0)
}

fn write_file(str: &str, file: &mut File) {
    match file.write_all(str.as_bytes()) {
        Err(e) => panic!("Couldn't write to image: {}", e),
        Ok(_) => println!("Image successfully written!"),
    }
}