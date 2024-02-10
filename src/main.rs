use std::fs::File;
use std::io::{self, Write};

mod vec3;
use vec3::*;
mod ray;
use ray::*;

fn ray_color(r: &Ray) -> Color3 {
    let unit: Vec3 = r.dir().unit();
    let alpha: f64 = 0.5 * (unit.y() + 1.);
    (1.-alpha) * Color3::new(1., 1., 1.) + alpha * Color3::new(0., 0., 1.)
}

fn main() {
    let mut output = File::create("./out.ppm").unwrap();

    // Image

    let aspect: f64 = 16./9.;

    let image_width: u32 = 400;
    let image_height: u32 = {
        let height: f64 = image_width as f64 / aspect;
        if height < 1. { 1 }
        else { height as u32 }
    };

    // Viewport
    let focal_length: f64 = 1.;

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * image_width as f64 / image_height as f64;

    let camera_pos = Point3::new(0., 0., 0.);

    let viewport_u: Vec3 = Vec3::new(viewport_width, 0., 0.);
    let viewport_v: Vec3 = Vec3::new(0., -viewport_height, 0.);

    let delta_u: Vec3 = viewport_u / image_width as f64;
    let delta_v: Vec3 = viewport_v / image_height as f64;

    let viewport_upper_left = camera_pos - Vec3::new(0., 0., focal_length) - viewport_u/2. - viewport_v/2.;
    let pixel00: Point3 = viewport_upper_left + 0.5 * (delta_u + delta_v);

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    let _ = output.write(header.as_bytes()).unwrap();

    for row in 0..image_height {
        print!("Row {}: ", row);
        io::stdout().flush().unwrap();

        for col in 0..image_width {
            let frag_pos = pixel00 + col as f64 * delta_u + row as f64 * delta_v;
            let ray_dir = frag_pos - camera_pos;

            let r: Ray = Ray::new(camera_pos, ray_dir);

            let frag_color: Color3 = ray_color(&r);
            let _ = output.write(frag_color.ppm().as_bytes()).unwrap();
        }
        
        println!("Done.");
    }
}
