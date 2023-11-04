use std::ops;
use std::fmt;

#[derive(Clone)]
pub struct Vec3
{
    e: [f32; 3]
}

impl Vec3
{
    pub fn new() -> Self
    {
        Self{ e: [0.0, 0.0, 0.0] }
    }

    pub fn with_color(red: f32, green: f32, blue: f32) -> Self
    {
        Self{ e: [red, green, blue] }
    }

    pub fn with_position(x: f32, y: f32, z: f32) -> Self
    {
        Self{ e: [x, y, z] }
    }

    pub fn x(&self) -> f32
    {
        self.e[0]
    }

    pub fn y(&self) -> f32
    {
        self.e[1]
    }

    pub fn z(&self) -> f32
    {
        self.e[2]
    }

    pub fn length_squared(&self) -> f32
    {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2] 
    }

    pub fn length(&self) -> f32
    {
        self.length_squared().sqrt()
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f32
    {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3
    {
        Vec3{ e: [
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0]
        ] }
    }

    pub fn unit_vector(v: &Vec3) -> Vec3
    {
        v / v.length()
    }
}

impl ops::Add<Vec3> for &Vec3 
{
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 
    {
        Vec3{ e: [
            self.e[0] + _rhs.e[0],
            self.e[1] + _rhs.e[1],
            self.e[2] + _rhs.e[2],
        ]}
    }
}

impl ops::Sub<Vec3> for Vec3 
{
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 
    {
        Vec3{ e: [
            self.e[0] - _rhs.e[0],
            self.e[1] - _rhs.e[1],
            self.e[2] - _rhs.e[2],
        ]}
    }
}

impl ops::Mul<f32> for &Vec3 
{
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output 
    {
        Vec3{ e: [
            self.e[0] * rhs,
            self.e[1] * rhs,
            self.e[2] * rhs
        ]}
    }
}

impl ops::Div<f32> for &Vec3 
{
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output 
    {
        Vec3{ e: [
            self.e[0] / rhs,
            self.e[1] / rhs,
            self.e[2] / rhs
        ]}
    }
}

impl ops::Neg for Vec3 
{
    type Output = Vec3;

    fn neg(self) -> Vec3 
    {
        Vec3{ e: [
            -self.e[0],
            -self.e[1],
            -self.e[2],
        ]}
    }
}

impl ops::Index<usize> for Vec3
{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl fmt::Display for Vec3
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", (self.e[0]*255.999) as u8, (self.e[1]*255.999) as u8, (self.e[2]*255.999) as u8)
    }
}