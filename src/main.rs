mod vec3;
mod ray;

use std::{fs::File, io::Write};
use std::time::Instant;

use crate::vec3::Vec3;
use crate::ray::Ray;

//define Picture
const ASPECT_RATION: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 800;
const IMAGE_HEIGHT: usize = match (IMAGE_WIDTH as f32 / ASPECT_RATION) as usize {
    0 => 1,
    x => x
};

//define Camera
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32);
const FOCAL_LENGTH: f32 = 1.0;

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32
{
    let oc = r.origin() - center;

    let a = r.direction().length_squared();
    let half_b = Vec3::dot(&oc, &r.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;

    if discriminant < 0.0
    {
        return -1.0
    }
    else 
    {
        return (-half_b - discriminant.sqrt() ) / a;
    }
}

fn ray_color(r: &Ray) -> Vec3
{
    let t = hit_sphere(&Vec3::with_position(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 
    {
        let n: Vec3 = Vec3::unit_vector(&(r.at(t) - Vec3::with_position(0.0,0.0,-1.0)));
        return 0.5*Vec3::with_color(n.x()+1.0, n.y()+1.0, n.z()+1.0);
    }
    let unit_direction = Vec3::unit_vector(&r.direction());
    let a = 0.5*(unit_direction.y() + 1.0);
    Vec3::with_color(1.0, 1.0, 1.0) * (1.0 - a) + a * Vec3::with_color(0.5, 0.7, 1.0)
}

fn raytrace() {
    let camera_center = Vec3::with_position(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::with_position(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = Vec3::with_position(0.0, -VIEWPORT_HEIGHT, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = &viewport_u / IMAGE_WIDTH as f32;
    let pixel_delta_v = &viewport_v / IMAGE_HEIGHT as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = &camera_center - Vec3::with_position(0.0, 0.0, FOCAL_LENGTH) - &viewport_u/2.0 - &viewport_v/2.0;
    let pixel00_loc = &viewport_upper_left + &(&pixel_delta_u + &pixel_delta_v) * 0.5;
    let mut f = File::create("test.ppm").unwrap();
    write!(f, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();
    let start_time = Instant::now();

    for j in 0..IMAGE_HEIGHT
    {
        print!("\rScanlines remaining: {}     ", (IMAGE_HEIGHT - j));
        for i in 0..IMAGE_WIDTH
        {
            let pixel_center = &pixel00_loc + (i as f32 * &pixel_delta_u) + (j as f32 * &pixel_delta_v);
            let ray_direction = pixel_center - &camera_center;
            let r = Ray::new(&camera_center, &ray_direction);

            let c = ray_color(&r);
            write!(f, "{}\n", c).unwrap();
        }
    }
    let elapsed_time = start_time.elapsed();
    print!("\rDone! Raytracing took: {:?}\n", elapsed_time);
}

fn main() {
    raytrace();
}
