use std::f64::INFINITY;
use std::fs::File;
use std::io::{self, Write};

use fastrand::f64;

use crate::objects::{HitRecord, Objects};
use crate::ray::Ray;
use crate::utils::Interval;
use crate::vec3::{Color3, Point3, Vec3};



pub struct Camera {
    samples_per_pixel: f64,
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00: Point3,
    delta_u: Vec3,
    delta_v: Vec3,
    output: File,
}

impl Camera {
    pub fn render(&mut self, mut objects: Objects) {
        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        let _ = self.output.write(header.as_bytes()).unwrap();
    
        for row in 0..self.image_height {
            print!("Row {}: ", row);
            io::stdout().flush().unwrap();
    
            for col in 0..self.image_width {
                let mut frag_sum_color: Color3 = Color3::new(0., 0., 0.);
                (0..self.samples_per_pixel as u32).into_iter().for_each(|_i| {
                    let r = self.get_ray(col, row);
                    frag_sum_color += self.ray_color(&r, &mut objects);
                });
    
                self.write_color(frag_sum_color);
            }
            
            println!("Done.");
        }
    }

    pub fn initialize(aspect: f64, samples_per_pixel: u32, image_width: u32) -> Self {
        let samples_per_pixel: f64 = samples_per_pixel as f64;

        let image_height = {
            let height: f64 = image_width as f64 / aspect;
            if height < 1. { 1 }
            else { height as u32 }
        };

        let center = Point3::new(0., 0., 0.);


        // Viewport

        let focal_length: f64 = 1.;

        let viewport_height: f64 = 2.;
        let viewport_width: f64 = viewport_height * image_width as f64 / image_height as f64;

        let viewport_u: Vec3 = Vec3::new(viewport_width, 0., 0.);
        let viewport_v: Vec3 = Vec3::new(0., -viewport_height, 0.);

        let delta_u = viewport_u / image_width as f64;
        let delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center - Vec3::new(0., 0., focal_length) - viewport_u/2. - viewport_v/2.;
        let pixel00 = viewport_upper_left + 0.5 * (delta_u + delta_v);

        let output = File::create("./out.ppm").unwrap();

        Self {
            samples_per_pixel,
            image_width,
            image_height,
            center,
            pixel00,
            delta_u,
            delta_v,
            output,
        }
    }

    fn get_ray(&self, col: u32, row: u32) -> Ray {
        let frag_pos = self.pixel00 + (col as f64 + f64() - 0.5) * self.delta_u + (row as f64 + f64() - 0.5) * self.delta_v;
        let ray_dir = frag_pos - self.center;

        Ray::new(self.center, ray_dir)

    }
    
    fn ray_color(&mut self, r: &Ray, objects: &Objects) -> Color3 {
        let mut rec: HitRecord = HitRecord::new();
        if objects.hit(r, Interval::new(0., INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color3::new(1., 1., 1.))
        }

        let unit: Vec3 = r.dir().unit();
        let alpha: f64 = 0.5 * (unit.y() + 1.);
        (1.-alpha) * Color3::new(1., 1., 1.) + alpha * Color3::new(0., 0., 1.)
    }

    fn write_color(&mut self, sum_color: Color3) {
        let scale: f64 = 1. / self.samples_per_pixel;

        let mut pixel_color: Color3 = sum_color * scale;
        pixel_color.clamp(0., 1.);

        let _ = self.output.write(pixel_color.ppm().as_bytes()).unwrap();
    }
}