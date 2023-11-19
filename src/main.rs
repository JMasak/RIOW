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
use rand::prelude::*;

// constants
const PI: f32 = std::f32::consts::PI;

//utility functions
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random() -> f32 {
    // Returns a random real in [0,1).
    rand::random()
}

pub fn random_with_min_max(min: f32, max: f32) -> f32{
    min + (max-min) * random()
}

fn raytrace() {
    // world
    let mut world = HitableList::new();
    world.add(Box::new(Sphere::new(Vec3::with_position(0.0, -100.5, -1.0), 100.0)));
    world.add(Box::new(Sphere::new(Vec3::with_position(0.0, 0.0, -1.0), 0.5)));
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 720;
    cam.samples_per_pixel = 20;
    cam.max_depth = 50;
    cam.render(&world);
}

fn main() {
    raytrace();
}
