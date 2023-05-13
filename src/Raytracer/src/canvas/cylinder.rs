//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Cylinder
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
pub struct Cylinder {
    pub center:Point3D,
    pub radius:f64,
    pub height:f64,
    pub color:Color,
    pub axis:char,
    pub pattern:Box<dyn Mask>,
    pub reflectiveness:f64
}

impl Cylinder {
    pub fn new_config(center:Point3D, radius:f64, height: f64, color:Color, axis:char, pattern:Box<dyn Mask>) -> Self {
        Cylinder {center, radius, height, color, axis, pattern, reflectiveness: 0.0}
    }

    fn get_origin(&self, ray:Ray) -> Option<Vec<f64>> {
        if self.axis == 'X' {
            return Some(vec![ray.origin.y, ray.origin.z, ray.origin.x]);
        } else if self.axis == 'Y' {
            return Some(vec![ray.origin.x, ray.origin.z, ray.origin.y]);
        } else if self.axis == 'Z' {
            return Some(vec![ray.origin.x, ray.origin.y, ray.origin.z]);
        } else {
            return None;
        }
    }

    fn get_center(&self, _ray:Ray) -> Option<Vec<f64>> {
        if self.axis == 'X' {
            return Some(vec![self.center.y, self.center.z, self.center.x]);
        } else if self.axis == 'Y' {
            return Some(vec![self.center.x, self.center.z, self.center.y]);
        } else if self.axis == 'Z' {
            return Some(vec![self.center.x, self.center.y, self.center.z]);
        } else {
            return None;
        }
    }

    fn get_direction(&self, ray:Ray) -> Option<Vec<f64>> {
        if self.axis == 'X' {
            return Some(vec![ray.direction.y, ray.direction.z, ray.direction.x]);
        } else if self.axis == 'Y' {
            return Some(vec![ray.direction.x, ray.direction.z, ray.direction.y]);
        } else if self.axis == 'Z' {
            return Some(vec![ray.direction.x, ray.direction.y, ray.direction.z]);
        } else {
            return None;
        }
    }
}

impl Primitives for Cylinder {
    fn hits(&self, ray:Ray) -> Option<Point3D> {
        let origin: Vec<f64>;
        let center: Vec<f64>;
        let direction: Vec<f64>;

        match self.get_origin(ray) {
            Some(result) => origin = result,
            None => return None
        }
        match self.get_center(ray) {
            Some(result) => center = result,
            None => return None
        }
        match self.get_direction(ray) {
            Some(result) => direction = result,
            None => return None
        }

        let a: f64 = direction[0].powi(2) + direction[1].powi(2);

        let b: f64 = 2.0 * (direction[0] * (origin[0] - center[0])
        + direction[1] * (origin[1] - center[1]));

        let c: f64 = (origin[0] - center[0]).powi(2) +
        (origin[1] - center[1]).powi(2) - self.radius.powi(2);

        let delta: f64 = b.powi(2) - 4.0 * a * c;
        if delta.abs() < 0.001 || delta < 0.0 {
            return None;
        }
        let t1: f64 = (-b - delta.sqrt()) / (2.0 * a);
        let t2: f64 = (-b + delta.sqrt()) / (2.0 * a);
        let t: f64 = t1.min(t2);

        let r = origin[2] + t * direction[2];

        if r >= center[2] && r <= center[2] + self.height {
            return Some(Point3D::default());
        }
        return None;
    }
    fn translate(&mut self, vec:Vector3D) {
        self.center.x += &vec.x;
        self.center.x += &vec.x;
        self.center.y += &vec.y;
    }
    fn rotatex(&mut self, _angle:f64) {}
    fn rotatey(&mut self, _angle:f64) {}
    fn rotatez(&mut self, _angle:f64) {}
    fn suface_normal(&self, hit_point:Point3D) -> Vector3D {
        let direction = hit_point - self.center;
        let norme = (direction.x * direction.x + direction.x * direction.x + direction.y * direction.y).sqrt();
        return Vector3D::new(direction.x / norme, direction.x / norme, direction.y / norme)
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

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder {
            center: Point3D::default(),
            radius: 0.0,
            height: 0.0,
            color: Color::default(),
            axis: 'Z',
            pattern: Box::new(Solid::default()),
            reflectiveness: 0.0
        }
    }
}