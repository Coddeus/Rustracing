mod camera;
mod objects;
mod ray;
mod utils;
mod vec3;


use vec3::Point3;
use objects::{Objects, Sphere};
use camera::Camera;


fn main() {
    let mut objects: Objects = Objects::new();
    let all_obj = vec![
        Sphere::new(Point3::new(0., 0., -1.), 0.5),
        Sphere::new(Point3::new(0., -100.5, -1.), 100.),
        // Add objects here
    ];
    for obj in all_obj.iter() {
        objects.add(obj);
    }


    let mut cam = Camera::initialize(16./9., 1440, 10, 10);
    cam.render(objects);
}
