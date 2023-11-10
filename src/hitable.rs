use crate::{Vec3, ray::Ray};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal).is_sign_negative();
        self.normal = if self.front_face {
            outward_normal.clone()
        }
        else {
            -outward_normal.clone()
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut HitRecord) -> bool;
}

pub struct HitableList {
    objects: Vec<Box<dyn Hitable>>
}

impl HitableList {
    pub fn new() -> Self
    {
        Self { objects: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hitable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let mut rec = HitRecord{
            p: Vec3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut _closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, ray_tmax, &mut rec) {
                hit_anything = true;
                _closest_so_far = rec.t;
            }
        }
        match hit_anything {
            true => Some(rec),
            false => None
        }
    }
}