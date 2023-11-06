use crate::{Vec3, ray::Ray};

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