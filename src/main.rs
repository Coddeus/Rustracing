mod camera;
mod materials;
mod objects;
mod ray;
mod utils;
mod vec3;


use std::rc::Rc;

use fastrand::f64;
use materials::{Dielectric, Lambertian, Metal};
use vec3::{Color3, Point3, Vec3};
use objects::{Objects, Sphere};
use camera::Camera;


fn main() {
    let mut objects: Objects = Objects::new();

    let mat_ground = Rc::new(Lambertian::new(Color3::new(0.5, 0.5, 0.5)));
    let mut all_spheres: Vec<Sphere> = vec![
        Sphere::new(Point3::new(0., -1000., 0.), 1000., mat_ground.clone())
    ];

    let avoid_center: Point3 = Point3::new(4.0, 0.2, 0.0);
    
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = f64();
            let center = Point3::new(a as f64 + 0.9*f64(), 0.2, b as f64 + 0.9*f64());

            if (center - avoid_center).len() > 0.9 {

                if choose_mat < 0.6 { // Matte
                    let albedo: Color3 = Color3::new(f64(), f64(), f64());
                    let sphere_mat = Rc::new(Lambertian::new(albedo));
                    all_spheres.push(Sphere::new(center, 0.2, sphere_mat));

                } else if choose_mat < 0.9 { // Metal
                    let albedo: Color3 = Color3::rand(0.5, 1.0);
                    let fuzz: f64 = f64()/2.0;
                    let sphere_mat = Rc::new(Metal::new(albedo, fuzz));
                    all_spheres.push(Sphere::new(center, 0.2, sphere_mat));

                } else { // Glass
                    let sphere_mat = Rc::new(Dielectric::new(1.3 + 0.4 * f64()));
                    all_spheres.push(Sphere::new(center, 0.2, sphere_mat));

                }
            }
        }
    }
    
    let mat1 = Rc::new(Dielectric::new(1.5));
    all_spheres.push(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1));
    let mat2 = Rc::new(Lambertian::new(Color3::new(0.4, 0.2, 0.1)));
    all_spheres.push(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2));
    let mat3 = Rc::new(Metal::new(Color3::new(0.7, 0.6, 0.5), 0.0));
    all_spheres.push(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3));

    for obj in all_spheres.iter() {
        objects.add(obj);
    }


    let mut cam: Camera = Camera::default();
    cam.aspect = 16.0 / 9.0;
    cam.image_width = 720;
    cam.samples_per_pixel = 1;
    cam.max_depth = 3;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(objects);
}
