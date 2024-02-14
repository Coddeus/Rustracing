mod camera;
mod materials;
mod objects;
mod ray;
mod utils;
mod vec3;


use std::rc::Rc;

use materials::{Lambertian, Metal};
use vec3::{Color3, Point3};
use objects::{Objects, Sphere};
use camera::Camera;


fn main() {
    let mat_ground = Rc::new(Lambertian::new(Color3::new(0.8, 0.8, 0.8)));
    let mat_center = Rc::new(Metal::new(Color3::new(0.7, 0.3, 0.3), 0.0));
    let mat_left = Rc::new(Metal::new(Color3::new(0.8, 0.8, 0.8), 0.3));
    let mat_right = Rc::new(Metal::new(Color3::new(0.8, 0.6, 0.2), 1.0));

    let mut objects: Objects = Objects::new();
    let all_obj = vec![
        Sphere::new(Point3::new(0., -100.5, -1.), 100., mat_ground.clone()),
        Sphere::new(Point3::new(0., 0., -1.), 0.5, mat_center.clone()),
        Sphere::new(Point3::new(-1., 0., -1.), 0.5, mat_left.clone()),
        Sphere::new(Point3::new(1., 0., -1.), 0.5, mat_right.clone()),
        // Add objects here
    ];
    for obj in all_obj.iter() {
        objects.add(obj);
    }


    let mut cam = Camera::initialize(16./9., 1440, 10, 10);
    cam.render(objects);
}
