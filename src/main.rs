use std::fs::File;
use std::io::prelude::*;
use rust_tracer::*;
use num::Float;

fn main() {
    println!("Hello, world!");
    write_pixels();
}

fn write_pixels() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let img_width: f64 = 600.0;
    let img_height: f64 = img_width / aspect_ratio;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0; // <- MAGIC NUMBER ALERT.

    let origin: Vec3 = Default::default(); // A zero vector.
    let horizontal = Vec3 { x: viewport_width, y: 0.0, z: 0.0, };
    let vertical = Vec3 { x: 0.0, y: viewport_height, z: 0.0, };
    let lower_left_corner = origin.clone() - horizontal.div(2.0) - vertical.div(2.0) - Vec3 {x: 0.0, y: 0.0, z: focal_length, };

    // Render

    let mut image_string = String::new();

    image_string.push_str(format!("P3\n{} {}\n255\n", img_width, img_height).as_str());

    for i in (0..=((img_height.round() as i64)-1)).rev() {
        print!("\rScanlines remaining: {}", i);
        for j in 0..=((img_width.round() as i64)-1) {

            let u = j as f64 / (img_width - 1.0); // Scan across left to right of the viewport
            let v = i as f64 / (img_height - 1.0); // Scan from bottom to top of the viewport

            let ray = Ray {
                origin: origin.clone(),
                direction: lower_left_corner.clone() + horizontal.mul(u) + vertical.mul(v) - origin.clone()
            };

            let colour = Vec3 {
                x: i as f64 / (img_height - 1.0),
                y: j as f64 / (img_width - 1.0),
                z: 0.5,
            };

            image_string.push_str(write_colour(ray_to_colour(&ray)).as_str());
            // image_string.push_str(write_colour(colour).as_str())
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
    format!("{} {} {}\n", (col.x * 255.0) as f32, (col.y * 255.0) as f32, (col.z * 255.0) as f32)
}

fn write_file(str: &str, file: &mut File) {
    match file.write_all(str.as_bytes()) {
        Err(e) => panic!("Couldn't write to image: {}", e),
        Ok(_) => println!("Image successfully written!"),
    }
}

fn ray_to_colour(ray: &Ray) -> Vec3 {

    let t =  hit_sphere(&Vec3 {x: 0.0, y: 0.0, z: -1.0, }, 0.5, &ray); //  we created the centre

    // Only colour normals that are in front of the camera
    if t > 0.0 {
        let N = find_unit_vector(&(ray.at(t) - Vec3 { x: 0.0, y: 0.0, z: -1.0 }));
        return Vec3{x: N.x + 1.0, y: N.y + 1.0, z: N.z + 1.0}.mul(0.5)
    }

    let unit_direction = ray.direction.unit_vector(); // Get the unit vector of the ray
    // <- MAGIC NUMBER ALERT
    let mag = 0.5 * (unit_direction.y + 1.0); // The 1.0 is the focal length here. As we go upwards, the colour decreases.
    let colour_vec = Vec3 {x: 1.0, y: 1.0, z: 1.0};
    let grad_vec = Vec3 { x: 0.5, y: 0.7, z: 1.0 };
    colour_vec.mul((1.0 - mag)) + grad_vec.mul(mag) // Compute the magic gradient colour.
}

fn lerp_float(begin: f64, end: f64, t: f64) -> f64 {
    ((1.0 - t) * begin) + (t * end)
}

// Calculates the discriminant
fn hit_sphere(centre: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin.clone() - centre.clone();
    let a = ray.direction.length_sq();
    let half_b = oc.dot(&ray.direction); // we removed the '2' as we can consider the case b = 2h
    let c = oc.length_sq() - (radius * radius);
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}