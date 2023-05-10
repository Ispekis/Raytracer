//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Cone
//

use crate::Math::{Vector3D::Vector3D, Point3D::Point3D};
use crate::Interfaces::Primitives::Primitives;
use crate::RayTracer::Ray::Ray;
use crate::Math::formulas;

#[derive(Copy, Clone)]
pub struct Cone {
    pub cosa:f64,
    pub height:f64,
    pub tip:Vector3D,
    pub axis:char,
    pub direction:Vector3D,
    pub color:Vector3D
}

impl Cone {
    pub fn new_config(axis:char, color:Vector3D, height:u32, degrees:f64, tip:Vector3D) -> Self {
        let mut pos = Point3D::default();
        let mut direction = Vector3D::default();
        if axis == 'X' {
            pos.x = position;
            direction.x = 1.0;
        }
        if axis == 'Y' {
            pos.y = position;
            direction.y = 1.0;
        }
        if axis == 'Z' {
            pos.z = position;
            direction.y = 1.0;
        }
        Cone {((degrees / 2.0).to_radians()).cos(), axis, direction, height, color, tip};
    }
}

impl Primitives for Cone {
    fn hits(&self, ray:Ray) -> bool{
        let co = ray.origin - self.tip;
        let a = ray.direction.scal(&self.direction) - self.cosa.powf(2.0);
        let b = 2.0 * (ray.direction.scal(&self.direction) * co.scal(self.direction) - ray.direction.scal(&co) * self.cosa.powf(2.0));
        let c = co.scal(&self.direction).powf(2.0) - co.scal(&co) * self.cosa.powf(2.0);
        let d = b * b - 4.0 * a * c;
        if (d < 0.0)
            return false;
        return true;
    }
    fn translate(&mut self, Translate:Vector3D) {
        self.center.x += &Translate.x;
        self.center.y += &Translate.y;
        self.center.z += &Translate.z;
    }
    fn rotateX(&mut self, angle:f64) {}
    fn rotateY(&mut self, angle:f64) {}
    fn rotateZ(&mut self, angle:f64) {}
}

impl Default for Cone {
    fn default() -> Self {
        Cone {
            center: Point3D::default(),
            radius: 0.0,
            color: Vector3D::default()
        }
    }
}