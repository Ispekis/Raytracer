//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// planes
//

use crate::math::{vector3d::Vector3D, point3d::Point3D};
use crate::interfaces::primitives::Primitives;
use crate::ray_tracer::ray::Ray;

use super::color::Color;
use super::material::{
    Solid,
    Mask
};

#[derive(Clone)]
pub struct Plane {
    pub axis:char,
    pub center:Point3D,
    pub direction:Vector3D,
    pub color:Color,
    pub pattern: Box<dyn Mask>,
    pub reflectiveness: f64
}

impl Plane {
    pub fn new_config(axis:char, position:f64, color:Color, pattern:Box<dyn Mask>) -> Self {
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
        Plane {axis, center:pos, direction: direction.normalize(), color, pattern, reflectiveness:0.0}
    }
}

impl Primitives for Plane {
    fn hits(&self, ray:Ray) -> Option<Point3D>{
        // if ray.origin.y.abs() <= std::f64::EPSILON {
        //     return None;
        // }
        // let mut tmp:Vector3D = ray.direction;
        // let mut tmp1:f64;
        // if self.axis == 'X' {
        //     tmp1 = tmp.x;
        //     tmp.x = tmp.y;
        //     tmp.y = tmp1;
        // }
        // if self.axis == 'Y' {
        //     tmp1 = tmp.y;
        //     tmp.y = tmp.z;
        //     tmp.z = tmp1;
        // }
        // if self.axis == 'Z' {
        //     tmp1 = tmp.z;
        //     tmp.z = tmp.x;
        //     tmp.x = tmp1;
        // }
        let dot = ray.direction.scal(&self.direction);

        // print!("dot = {} {} ", dot, ray.direction);
        if dot.abs() > 1e-6 {
            let t = (self.center - ray.origin).scal(&self.direction) / dot;
            if t >= 0.0 {
                let inter_point = ray.origin + (ray.direction * t);
                return Some(inter_point);
            }
        }
        None
        // let dot = ray.direction.scal(&self.direction);

        // if dot > 1e-6 {
        //     let t = ((self.center - ray.origin).scal(&self.direction)) / dot;
        //     if t >= 0.0 {
        //         let inter_point = ray.origin + (ray.direction * t);
        //         return Some(inter_point);
        //     }
        // }
        // return None;
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
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_pattern(&self) -> Box<dyn super::material::Mask> {
        self.pattern.clone()
    }
    fn get_reflectiveness(&self) -> f64 {
        self.reflectiveness
    }
}

impl Default for Plane {
    fn default() -> Self {
        Plane {
            axis: 'C',
            center: Point3D::default(),
            direction: Vector3D::default(),
            color: Color::default() ,
            pattern: Box::new(Solid::default()),
            reflectiveness: 0.0
        }
    }
}

