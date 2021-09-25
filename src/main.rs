use std::fs::File;
use std::io::prelude::*;
use rust_tracer::*;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::video::Window;
use std::task::Context;
use sdl2::rect::Point;
use std::mem::size_of;
use std::thread::sleep;
use std::{thread, time};

// use sdl2::gfx::primitives::DrawRenderer;

fn main() -> Result<(), String>{
    println!("Hello, world!");

    let sdl_context = sdl2::init()?;
    // let video_subsystem = sdl_context.video()?;
    let w = 800;
    let res = (w, w / (16 / 9));

    let mut offset = 1.0;
    let arr = paint_pixels(res.1, offset);

    let window = init_window(&sdl_context, res.0, 16.0 / 9.0)?;

    let mut canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let timer = sdl_context.timer()?;

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(None, res.0, res.0 * 9 / 16)
        .map_err(|e| e.to_string())?;

    texture.update(None, arr.as_slice(), (res.1 * 4 * (size_of::<u8>() as u32)) as usize);
    canvas.copy(&texture, None, None);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {
                    // eprintln!("Event pump is pumping: {:?}", event);
                    canvas.present();
                }
            }

        }

        // offset += (timer.ticks() as f64).sin() * 0.5;
    }

    // write_pixels();

    Ok(())
}

fn init_window(context: &sdl2::Sdl, width: u32, aspect_ratio: f64) -> Result<Window, String> {
    let video_subsystem = context.video()?;

    let window = video_subsystem
        .window("SDL Window", width, (width as f64/ aspect_ratio) as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string());

    window.map_err(|e| e.to_string())
}

/// Used to paint the scene to an array of pixel components.
fn paint_pixels(width: u32, offset: f64) -> Vec<u8> {
    // Use canvas.drawPoint
    // let (width, height) = canvas.drawable_size();

    let aspect_ratio = 16.0 / 9.0;
    let img_width: f64 = width as f64;
    let img_height: f64 = img_width / aspect_ratio;
    let samples_per_pixel = 50;

    let max_depth = 50;

    // World
    let mut world: HittableList = Default::default();
    world.add(Rc::new(Sphere {
        centre: Vec3::from((0.0, -105.5, -1.0)),
        radius: 100.0,
    }));
    world.add(Rc::new(Sphere {
        centre: Vec3::from((0.0, 0.0, -1.0)),
        radius: 0.5,
    }));
    // world.add(Rc::new(Sphere {
    //     centre: Vec3::from((1.0, 1.0 * offset, -1.5)),
    //     radius: 0.25,
    // }));

    // Camera
    let camera = Camera::new(width, 16.0 / 9.0, 2, 1.0, unit_vector(0.0));

    let mut color_vector: Vec<u8> = Vec::new();

    for i in (0..=((camera.height as i64)-1)).rev() {
        // print!("\rScan-lines remaining: {}\n", i);
        for j in 0..=((camera.width as i64)-1) {

            // let mut pixel = ray_to_pixel(&ray, &world);
            let mut r_temp = 0.0;
            let mut g_temp = 0.0;
            let mut b_temp = 0.0;

            for k in 0..samples_per_pixel {

                let u = (j as f64 + fastrand::f64()) / (img_width - 1.0); // Scan across left to right of the viewport
                let v = (i as f64 + fastrand::f64()) / (img_height - 1.0); // Scan from bottom to top of the viewport
                let ray = camera.get_ray(u, v);

                let progress = (j / camera.width as i64) / camera.height as i64 * 100;

                let col = ray_to_pixel(&ray, &world, max_depth);
                r_temp += col.r as f64;
                g_temp += col.g as f64;
                b_temp += col.b as f64;

                print!("\rScan-lines remaining: {} Sample: {}", i, k);
            }

            let pixel = sample_colour((r_temp, g_temp, b_temp), samples_per_pixel);

            color_vector.push(255);
            color_vector.push(pixel.r);
            color_vector.push(pixel.g);
            color_vector.push(pixel.b);
            // println!("({} {} {})   ", pixel.r, pixel.g, pixel.b);
        }
    }

    color_vector
}

/// Used to write a PPM image.
fn write_pixels() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let img_width: f64 = 1200.0;
    let img_height: f64 = img_width / aspect_ratio;

    // World
    let mut world: HittableList = Default::default();
    world.add(Rc::new(Sphere {
        centre: Vec3 { x: 0.0, y: -105.0, z: -1.0 },
        radius: 100.0,
    }));
    world.add(Rc::new(Sphere {
            centre: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
            radius: 0.5,
        }));
    world.add(Rc::new(Sphere {
        centre: Vec3 { x: 1.0, y: 1.0, z: -1.5 },
        radius: 0.25,
    }));

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
        print!("\rScan-lines remaining: {}", i);
        for j in 0..=((img_width.round() as i64)-1) {

            let u = j as f64 / (img_width - 1.0); // Scan across left to right of the viewport
            let v = i as f64 / (img_height - 1.0); // Scan from bottom to top of the viewport

            let ray = Ray {
                origin: origin.clone(),
                direction: lower_left_corner.clone() + horizontal.mul(u) + vertical.mul(v) - origin.clone()
            };

            let colour = ray_to_colour(&ray, &world);

            image_string.push_str(write_colour(ray_to_colour(&ray, &world)).as_str());
            // image_string.push_str(write_colour(colour).as_str())
        }
    }
    print!("\n");
    let path = "new-img.ppm";

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create image file {}", why),
        Ok(file) => file,
    };

    write_file(image_string.as_str(), &mut file);
}

/// Used to convert a Vector3 into a string.
fn write_colour(col: Vec3) -> String {
    format!("{} {} {}\n", (col.x * 255.0) as i32, (col.y * 255.0) as i32, (col.z * 255.0) as i32)
}

/// Creates a PPM file given an input string.
fn write_file(str: &str, file: &mut File) {
    match file.write_all(str.as_bytes()) {
        Err(e) => panic!("Couldn't write to image: {}", e),
        Ok(_) => println!("Image successfully written!"),
    }
}

/// Converts a ray to a Vec3 representing a single colour, given a world object too.
fn ray_to_colour(ray: &Ray, world: &dyn Hittable) -> Vec3 {
    let mut rec: HitRecord = Default::default();

    if world.hit(ray, 0.0, f64::INFINITY, &mut rec) {
        return (rec.normal + unit_vector(1.0)).mul(0.5);
    }

    let unit_direction = ray.direction.unit_vector(); // Get the unit vector of the ray

    let t = (unit_direction.y + 1.0) * 0.5;

    return unit_vector(1.0).mul(1.0-t) + Vec3{x: 0.5, y: 0.7, z: 1.0,}.mul(t)
}

/// Traces a ray in a scene to an SDL pixel.
fn ray_to_pixel(ray: &Ray, world: &dyn Hittable, depth: u32) -> sdl2::pixels::Color {
    let mut rec: HitRecord = Default::default();

    if depth <= 0 {
        return Color::RGBA(0, 0, 0, 255);
    }

    print!("\rCurrently on depth pass {}", depth);
    if world.hit(ray, 0.0, f64::INFINITY, &mut rec) {
        // return (rec.normal + unit_vector(1.0)).mul(0.5);
        let target = rec.point.clone() + rec.normal.clone() + Vec3::random_in_unit_sphere();
        let temp_ray = Ray { origin: rec.point.clone(), direction: target - rec.point };
        let cache_col = ray_to_pixel(&temp_ray, world, depth - 1);
        return Color::RGBA(
            (cache_col.r as f64 * 0.5) as u8,
            (cache_col.g as f64 * 0.5) as u8,
            (cache_col.b as f64 * 0.5) as u8,
            cache_col.a,
        );
        return vec_to_col(&(rec.normal + unit_vector(1.0)).mul(0.5));
    }

    let unit_direction = ray.direction.unit_vector(); // Get the unit vector of the ray

    let t = (unit_direction.y + 1.0) * 0.5;
    let vec = unit_vector(1.0).mul(1.0-t) + Vec3{x: 0.5, y: 0.7, z: 1.0,}.mul(t);
    vec_to_col(&vec)
}

fn vec_to_col(v: &Vec3) -> sdl2::pixels::Color {
    Color::RGBA((v.x * 255.0) as u8, (v.y * 255.0) as u8, (v.z * 255.0) as u8, 255)
}