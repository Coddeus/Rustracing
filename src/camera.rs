use std::f64::INFINITY;
use std::fs::File;
use std::io::{self, Write};

use fastrand::f64;

use crate::objects::{HitRecord, Objects};
use crate::ray::Ray;
use crate::utils::{deg_to_rad, Interval};
use crate::vec3::{Color3, Point3, Vec3};



pub struct Camera {
    pub aspect: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub vfov: f64,
    pub output: File,
    
    image_height: u32,
    center: Point3,
    pixel00: Point3,
    delta_u: Vec3,
    delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect: 16.0/9.0,
            image_width: 720,
            samples_per_pixel: 10,
            max_depth: 10,
            lookfrom: Point3::new(0.0, 0.0, -1.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            vfov: 90.0,
            output: File::create("./out.ppm").unwrap(),

            // Dummy initialization
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00: Point3::new(0.0, 0.0, 0.0),
            delta_u: Vec3::new(0.0, 0.0, 0.0),
            delta_v: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Camera {
    pub fn render(&mut self, mut objects: Objects) {
        self.initialize();

        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        let _ = self.output.write(header.as_bytes()).unwrap();
    
        for row in 0..self.image_height {
            print!("Row {}: ", row);
            io::stdout().flush().unwrap();
    
            for col in 0..self.image_width {
                let mut frag_sum_color: Color3 = Color3::new(0., 0., 0.);
                (0..self.samples_per_pixel as u32).into_iter().for_each(|_i| {
                    let r = self.get_ray(col, row);
                    frag_sum_color += self.ray_color(&r, self.max_depth, &mut objects);
                });
    
                self.write_color(frag_sum_color);
            }
            
            println!("Done.");
        }
    }

    pub fn initialize(&mut self) {
        self.image_height = {
            let height: f64 = self.image_width as f64 / self.aspect;
            if height < 1. { 1 }
            else { height as u32 }
        };

        self.center = self.lookfrom;

        // Viewport

        let focal_length: f64 = (self.lookfrom - self.lookat).len();
        let theta: f64 = deg_to_rad(self.vfov);
        let h: f64 = (theta/2.0).tan();
        let viewport_height: f64 = 2. * h * focal_length;
        let viewport_width: f64 = viewport_height * self.image_width as f64 / self.image_height as f64;

        self.w = (self.lookfrom - self.lookat).unit();
        self.u = (self.vup & self.w).unit();
        self.v = self.w & self.u;

        let viewport_u: Vec3 = viewport_width * self.u;
        let viewport_v: Vec3 = viewport_height * (-self.v);

        self.delta_u = viewport_u / self.image_width as f64;
        self.delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - focal_length * self.w - viewport_u/2. - viewport_v/2.;
        self.pixel00 = viewport_upper_left + 0.5 * (self.delta_u + self.delta_v);
    }

    fn get_ray(&self, col: u32, row: u32) -> Ray {
        let frag_pos = self.pixel00 + (col as f64 + f64() - 0.5) * self.delta_u + (row as f64 + f64() - 0.5) * self.delta_v;
        let ray_dir = frag_pos - self.center;

        Ray::new(self.center, ray_dir)

    }
    
    fn ray_color(&mut self, r: &Ray, depth: u32, objects: &Objects) -> Color3 {
        if depth < 1 { return Color3::new(0., 0., 0.) }

        let mut rec: HitRecord = HitRecord::new();
        if objects.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let mut r_scattered: Ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.));
            let mut attenuation: Color3 = Color3::new(0., 0., 0.);
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut r_scattered) {
                return attenuation.mult(self.ray_color(&r_scattered, depth-1, objects))
            }
            return Color3::new(0., 0., 0.)
        }

        let unit: Vec3 = r.dir().unit();
        let alpha: f64 = 0.5 * (unit.y() + 1.);
        (1.-alpha) * Color3::new(1., 1., 1.) + alpha * Color3::new(1.0, 0.5, 0.8)
    }

    fn write_color(&mut self, sum_color: Color3) {
        let scale: f64 = 1. / self.samples_per_pixel as f64;

        let mut pixel_color: Color3 = sum_color * scale;
        pixel_color.to_gamma_2();
        pixel_color.clamp(0., 1.);

        let _ = self.output.write(pixel_color.ppm().as_bytes()).unwrap();
    }
}