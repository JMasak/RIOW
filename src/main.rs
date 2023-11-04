mod vec3;
mod ray;

use std::{fs::File, io::Write};

use crate::vec3::Vec3;
use crate::ray::Ray;

//define Picture
const ASPECT_RATION: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = match (IMAGE_WIDTH as f32 / ASPECT_RATION) as usize {
    0 => 1,
    x => x
};

//define Camera
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32);
const FOCAL_LENGTH: f32 = 1.0;
const CAMERA_CENTER: Vec3 = Vec3::with_position(0.0, 0.0, 0.0);

// Calculate the vectors across the horizontal and down the vertical viewport edges.
const VIEWPORT_U: Vec3 = Vec3::with_position(VIEWPORT_WIDTH, 0.0, 0.0);
const VIEWPORT_V: Vec3 = Vec3::with_position(0.0, -VIEWPORT_HEIGHT, 0.0);

// Calculate the horizontal and vertical delta vectors from pixel to pixel.
const PIXEL_DELTA_U: Vec3 = &VIEWPORT_U / IMAGE_WIDTH as f32;
const PIXEL_DELTA_V: Vec3 = &VIEWPORT_V / IMAGE_HEIGHT as f32;

// Calculate the location of the upper left pixel.
const VIEWPORT_UPPER_LEFT: Vec3 = CAMERA_CENTER - Vec3::with_position(0.0, 0.0, FOCAL_LENGTH) - &VIEWPORT_U/2.0 - &VIEWPORT_V/2.0;


fn ray_color(_r: &Ray) -> Vec3
{
    Vec3::new()
}

fn main() {
    let pixel00_loc = &VIEWPORT_UPPER_LEFT + &(&PIXEL_DELTA_U + PIXEL_DELTA_V) * 0.5;
    let mut f = File::create("test.ppm").unwrap();
    write!(f, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();

    for j in 0..IMAGE_HEIGHT
    {
        print!("\rScanlines remaining: {}     ", (IMAGE_HEIGHT - j));
        for i in 0..IMAGE_WIDTH
        {
            let red: f32 = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let green: f32 = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let blue: f32 = 0.0;

            let c = Vec3::with_color(red, green, blue);
            write!(f, "{}\n", c).unwrap();
        }
    }
    print!("\rDone!                    \n");
}
