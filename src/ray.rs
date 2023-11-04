use crate::vec3::Vec3;


pub struct Ray
{
    origin: Vec3,
    direction: Vec3
}

impl Ray
{
    pub fn new() -> Self
    {
        Ray
        {
            origin: Vec3::new(),
            direction: Vec3::new()
        }
    }

    pub fn with_point_direction(orig: Vec3, dir: Vec3) -> Self
    {
        Ray
        {
            origin: orig,
            direction: dir
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