use std::fs::File;
use std::io::prelude::*;
use rust_tracer::Vec3;

fn main() {
    println!("Hello, world!");
    write_pixels();
}

fn write_pixels() {

    let img_height: f32 = 256.0;
    let img_width: f32 = 256.0;

    let mut image_string = String::new();

    image_string.push_str("P3\n256 256\n255\n");

    for i in (0..=((img_height.round() as i64)-1)).rev() {
        print!("\rScanlines remaining: {}", i);
        for j in 0..=((img_width.round() as i64)-1) {
            let red: f32 = i as f32 / (img_width - 1.0) * 255.0;
            let green: f32 = j as f32 / (img_height - 1.0) * 255.0;
            let blue: f32 = 0.5 * 255.0;

            let r = red.round() as i64;
            let g = green.round() as i64;
            let b = blue.round() as i64;

            let t = format!("{} {} {}\n", r, g, b);

            image_string.push_str(t.as_str());
        }
    }
    print!("\n");
    let path = "../image.ppm";

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create image file {}", why),
        Ok(file) => file,
    };

    match file.write_all(image_string.as_bytes()) {
        Err(why) => panic!("couldn't write to image {}", why),
        Ok(_) => println!("Successfully wrote image file"),
    }
}
