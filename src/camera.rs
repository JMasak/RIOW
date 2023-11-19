use std::{time::Instant, fs::File, io::Write};
use crate::{hitable::{HitableList}, ray::Ray, vec3::Vec3, interval::Interval, random};


pub struct Camera {
    pub aspect_ratio: f32,          // Ratio of image width over height
    pub image_width: usize,         // Rendered image width in pixel count
    pub samples_per_pixel: usize,   // Count of random samples for each pixel
    pub max_depth: isize,           // Maximum number of ray bounces into scene
    image_height: usize,            // Rendered image height
    center: Vec3,                   // Camera center
    pixel00_loc: Vec3,              // Location of pixel 0, 0
    pixel_delta_u: Vec3,            // Offset to pixel to the right
    pixel_delta_v: Vec3             // Offset to pixel below
}

impl Camera {
    pub fn new() -> Self {
        Camera { 
            aspect_ratio: 1.0, 
            image_width: 100, 
            image_height: 100, 
            samples_per_pixel: 10,
            max_depth: 10,
            center: Vec3::with_position(0.0, 0.0, 0.0), 
            pixel00_loc: Vec3::with_position(-0.09, 0.09, 1.0), 
            pixel_delta_u: Vec3::with_position(0.02, 0.0, 0.0) ,
            pixel_delta_v: Vec3::with_position(0.0, -0.02, 0.0)
        }
    }
    pub fn render(&mut self, world: &HitableList) {
        self.initialize();
        let mut f = File::create("test.ppm").unwrap();
        write!(f, "P3\n{} {}\n255\n", self.image_width, self.image_height).unwrap();
        let start_time = Instant::now();

        for j in 0..self.image_height
        {
            print!("\rScanlines remaining: {}     ", (self.image_height - j));
            for i in 0..self.image_width
            {
                let mut c = Vec3::new();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    c = c + self.ray_color(&r, self.max_depth, &world);
                }
                write!(f, "{}\n", c.write_color(self.samples_per_pixel)).unwrap();
            }
        }
        let elapsed_time = start_time.elapsed();
        print!("\rDone! Raytracing took: {:?}\n", elapsed_time);
    }

    fn initialize(&mut self) {
        self.center = Vec3::with_position(0.0, 0.0, 0.0);
        self.image_height = match (self.image_width as f32 / self.aspect_ratio) as usize {
            0 => 1,
            x => x
        };

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * self.aspect_ratio;

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::with_position(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::with_position(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = &viewport_u / self.image_width as f32;
        self.pixel_delta_v = &viewport_v / self.image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = &self.center - Vec3::with_position(0.0, 0.0, focal_length) - &viewport_u/2.0 - &viewport_v/2.0;
        self.pixel00_loc = &viewport_upper_left + &(&self.pixel_delta_u + &self.pixel_delta_v) * 0.5;
    }

    fn ray_color(&self, r: &Ray, depth: isize, world: &HitableList) -> Vec3 {
        if depth <= 0 {
            return Vec3::with_color(0.0, 0.0, 0.0)
        }
        match world.hit(r, Interval::with_min_max(0.0, std::f32::INFINITY)) {
            Some(rec)=> {
                let direction = Vec3::random_on_hemisphere(&rec.normal);
                0.5 * self.ray_color(&Ray::new(&rec.p, &direction), depth-1, world)
            },
            None => {
                let unit_direction = Vec3::unit_vector(&r.direction());
                let a = 0.5*(unit_direction.y() + 1.0);
                Vec3::with_color(1.0, 1.0, 1.0) * (1.0 - a) + a * Vec3::with_color(0.5, 0.7, 1.0)
            }
        } 
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        // Get a randomly sampled camera ray for the pixel at location i,j.
        let pixel_center = &self.pixel00_loc + (i as f32 * &self.pixel_delta_u) + (j as f32 * &self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_direction = pixel_sample - &self.center;
        Ray::new(&self.center, &ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + random();
        let py = -0.5 + random();
        (px * &self.pixel_delta_u) + (py * &self.pixel_delta_v)
    }
}