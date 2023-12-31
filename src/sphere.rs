use crate::{vec3::Vec3, hitable::Hitable, interval::Interval};


pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    pub fn new(c: Vec3, r: f32) -> Self {
        Sphere { center: c, radius: r }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, ray_t: &Interval, rec: &mut crate::hitable::HitRecord) -> bool {
        let oc = r.origin() - &self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;    
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (&rec.p - &self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        return true;
    }
}