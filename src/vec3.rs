use std::ops;
use std::fmt;

use crate::interval::Interval;

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

    pub fn random() -> Self {
        Self{ e: [rand::random(),rand::random(),rand::random()]}
    }

    pub fn random_with_limits(min: f32, max: f32) -> Self {
        let delta = max-min;
        Self{ e: [min + delta * rand::random::<f32>(),min + delta * rand::random::<f32>(),min + delta * rand::random::<f32>()]}
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_with_limits(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Vec3::unit_vector(&Vec3::random_in_unit_sphere())
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Vec3::random_unit_vector();
        if Vec3::dot(&on_unit_sphere, normal) > 0.0 {
            return on_unit_sphere
        }
        else {
            return -on_unit_sphere
        }
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

    pub fn write_color(&self, samples_per_pixel: usize) -> String{
        let intensity = Interval::with_min_max(0.0, 0.999);
        let scale = 1.0 / samples_per_pixel as f32;
        format!("{} {} {}", 
            (256.0 * intensity.clamp(Vec3::linear_to_gamma(self.e[0] * scale))) as u8, 
            (256.0 * intensity.clamp(Vec3::linear_to_gamma(self.e[1] * scale))) as u8, 
            (256.0 * intensity.clamp(Vec3::linear_to_gamma(self.e[2] * scale))) as u8)
    }

    pub fn linear_to_gamma(linear_component: f32) -> f32 {
        linear_component.sqrt()
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

impl ops::Add<Vec3> for Vec3 
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

impl ops::Add<&Vec3> for Vec3 
{
    type Output = Vec3;

    fn add(self, _rhs: &Vec3) -> Vec3 
    {
        Vec3{ e: [
            self.e[0] + _rhs.e[0],
            self.e[1] + _rhs.e[1],
            self.e[2] + _rhs.e[2],
        ]}
    }
}

impl ops::Add<&Vec3> for &Vec3 
{
    type Output = Vec3;

    fn add(self, _rhs: &Vec3) -> Vec3 
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

impl ops::Sub<Vec3> for &Vec3 
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

impl ops::Sub<&Vec3> for Vec3 
{
    type Output = Vec3;

    fn sub(self, _rhs: &Vec3) -> Vec3 
    {
        Vec3{ e: [
            self.e[0] - _rhs.e[0],
            self.e[1] - _rhs.e[1],
            self.e[2] - _rhs.e[2],
        ]}
    }
}

impl ops::Sub<&Vec3> for &Vec3 
{
    type Output = Vec3;

    fn sub(self, _rhs: &Vec3) -> Vec3 
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

impl ops::Mul<f32> for Vec3 
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

impl ops::Mul<&Vec3> for f32 
{
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output 
    {
        Vec3{ e: [
            rhs.e[0] * self,
            rhs.e[1] * self,
            rhs.e[2] * self
        ]}
    }
}

impl ops::Mul<Vec3> for f32 
{
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output 
    {
        Vec3{ e: [
            rhs.e[0] * self,
            rhs.e[1] * self,
            rhs.e[2] * self
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

impl ops::Div<f32> for Vec3 
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

impl ops::Div<&Vec3> for f32
{
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Self::Output
    {
        Vec3{ e: [
            rhs.e[0] / self,
            rhs.e[1] / self,
            rhs.e[2] / self
        ]}
    }
}

impl ops::Div<Vec3> for f32
{
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output
    {
        Vec3{ e: [
            rhs.e[0] / self,
            rhs.e[1] / self,
            rhs.e[2] / self
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