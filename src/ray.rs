
use crate::vec3::Vec3;


pub struct Ray
{
    origin: Vec3,
    direction: Vec3
}

impl Default for Ray {
    fn default() -> Self {
        Ray
        {
            origin: Vec3::new(),
            direction: Vec3::new()
        }
    }
}

impl Ray
{
    pub fn new(orig: &Vec3, dir: &Vec3) -> Self
    {
        Ray
        {
            origin: orig.clone(),
            direction: dir.clone()
        }
    }

    pub fn origin(&self) -> Vec3
    {
        self.origin.clone()
    }

    pub fn direction(&self) -> Vec3
    {
        self.direction.clone()
    }

    pub fn at(&self, t: f32) -> Vec3
    {
        &self.origin + &self.direction * t
    }

}