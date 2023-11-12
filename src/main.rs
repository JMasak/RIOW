mod vec3;
mod ray;
mod hitable;
mod sphere;
mod interval;
mod camera;

use camera::Camera;

use crate::sphere::Sphere;
use crate::vec3::Vec3;
use crate::hitable::HitableList;

// constants
const PI: f32 = std::f32::consts::PI;

//utility functions
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

fn raytrace() {
    // world
    let mut world = HitableList::new();
    world.add(Box::new(Sphere::new(Vec3::with_position(0.0, -100.5, -1.0), 100.0)));
    world.add(Box::new(Sphere::new(Vec3::with_position(0.0, 0.0, -1.0), 0.5)));
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 720;
    cam.render(&world);
}

fn main() {
    raytrace();
}
