//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// planes
//

use crate::math::{vector3d::Vector3D, point3d::Point3D};
use crate::interfaces::primitives::Primitives;
use crate::ray_tracer::ray::Ray;

use super::material::{
    Solid,
    Mask
};

#[derive(Clone)]
pub struct Plane {
    pub axis:char,
    pub center:Point3D,
    pub direction:Vector3D,
    pub color:Vector3D,
    pub pattern: Box<dyn Mask>
}

impl Plane {
    pub fn new_config(axis:char, position:f64, color:Vector3D, pattern:Box<dyn Mask>) -> Self {
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
        Plane {axis, center:pos, direction, color, pattern}
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
    fn rotatex(&mut self, _:f64) {}
    fn rotatey(&mut self, _:f64) {}
    fn rotatez(&mut self, _:f64) {}
    fn suface_normal(&self, _:Point3D) -> Vector3D {
        Vector3D::new(0.0, 1.0, 0.1)
    }
    fn get_color(&self) -> Vector3D {
        self.color
    }

    fn get_pattern(&self) -> Box<dyn super::material::Mask> {
        self.pattern.clone()
    }
}

impl Default for Plane {
    fn default() -> Self {
        Plane {
            axis: 'X',
            center: Point3D::default(),
            direction: Vector3D::default(),
            color: Vector3D::default() ,
            pattern: Box::new(Solid::default())
        }
    }
}

