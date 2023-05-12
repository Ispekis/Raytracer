//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// planes
//

use std::f64::EPSILON;

use crate::Math::{Vector3D::Vector3D, Point3D::Point3D};
use crate::Interfaces::Primitives::Primitives;
use crate::RayTracer::Ray::Ray;
use crate::Math::formulas;

#[derive(Copy, Clone)]
pub struct Plane {
    pub axis:char,
    pub center:Point3D,
    pub direction:Vector3D,
    pub color:Vector3D
}

impl Plane {
    pub fn new_config(axis:char, position:f64, color:Vector3D) -> Self {
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
        Plane {axis, center:pos, direction, color}
    }
}

impl Primitives for Plane {
    fn hits(&self, ray:Ray) -> Option<Point3D>{
        // if (ray.origin.y.abs() <= std::f64::EPSILON) {
        //     return None;
        // }
        let dot = ray.direction.scal(&self.direction);

        if dot > 1e-6 {
            let t = ((self.center - ray.origin).scal(&self.direction)) / dot;
            if t >= 0.0 {
                let inter_point = ray.origin + (ray.direction * t);
                return Some(inter_point);
            }
        }
        return None;
        // if (ray.direction.y.abs() <= EPSILON) {
        //     return None;
        // }
        // let t = -ray.origin.y / ray.direction.y;
        // let inter_point = ray.origin + (ray.direction * t);
        // Some(inter_point)
    }
    fn translate(&mut self, translate:Vector3D) {
        self.center.x += translate.x;
        self.center.y += translate.y;
        self.center.z += translate.z;
    }
    fn rotateX(&mut self, angle:f64) {}
    fn rotateY(&mut self, angle:f64) {}
    fn rotateZ(&mut self, angle:f64) {}
    fn suface_normal(&self, hit_point:Point3D) -> Vector3D {
        Vector3D::new(0.0, 1.0, 0.1)
    }
    fn get_color(&self) -> Vector3D {
        self.color
    }
}

impl Default for Plane {
    fn default() -> Self {
        Plane { axis: 'X', center: Point3D::default(), direction: Vector3D::default(), color: Vector3D::default() }
    }
}

