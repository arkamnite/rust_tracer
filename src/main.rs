use std::fs::File;
use std::io::prelude::*;
use rust_tracer::*;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::render::WindowCanvas;

fn main() -> Result<(), String>{
    println!("Hello, world!");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("SDL Window", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;


    let mut event_pump = sdl_context.event_pump()?;

    let timer = sdl_context.timer()?;

    'running: loop {

        let ticks = timer.ticks() as f64;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                    | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(((ticks.sin() + 1.0) * 127.5) as u8, 0, 0));
        canvas.clear();
        canvas.present();
        std::thread::sleep(Duration::from_millis(50));
    }

    write_pixels();

    Ok(())
}

fn blit_texture(canvas: &WindowCanvas) -> Result<(), String> {


    Ok(())
}

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
        print!("\rScanlines remaining: {}", i);
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
    let path = "newimg.ppm";

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create image file {}", why),
        Ok(file) => file,
    };

    write_file(image_string.as_str(), &mut file);
}

fn write_colour(col: Vec3) -> String {
    format!("{} {} {}\n", (col.x * 255.0) as i32, (col.y * 255.0) as i32, (col.z * 255.0) as i32)
}

fn write_file(str: &str, file: &mut File) {
    match file.write_all(str.as_bytes()) {
        Err(e) => panic!("Couldn't write to image: {}", e),
        Ok(_) => println!("Image successfully written!"),
    }
}

fn ray_to_colour(ray: &Ray, world: &dyn Hittable) -> Vec3 {
    let mut rec: HitRecord = Default::default();

    if world.hit(ray, 0.0, f64::INFINITY, &mut rec) {
        return (rec.normal + unit_vector(1.0)).mul(0.5);
    }

    let unit_direction = ray.direction.unit_vector(); // Get the unit vector of the ray

    let t = (unit_direction.y + 1.0) * 0.5;

    return unit_vector(1.0).mul(1.0-t) + Vec3{x: 0.5, y: 0.7, z: 1.0,}.mul(t)
}