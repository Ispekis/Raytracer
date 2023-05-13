//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Cone
//

use crate::math::{vector3d::Vector3D, point3d::Point3D};
use crate::interfaces::primitives::Primitives;
use crate::ray_tracer::ray::Ray;
use crate::math::formulas;

#[derive(Copy, Clone)]
pub struct Cone {
    pub center:Point3D,
    pub radius:f64,
    pub height:f64,
    pub color:Vector3D,
    pub axis:char,
    pub direction:Vector3D
}

impl Cone {
    pub fn new_config(center:Point3D, radius:f64, height: f64, color:Vector3D, axis:char) -> Self {
        let mut direction = Vector3D::default();
        if axis == 'X' {
            direction.x = 1.0;
        }
        if axis == 'Y' {
            direction.y = 1.0;
        }
        if axis == 'Z' {
            direction.y = 1.0;
        }
        Cone {center, radius, height, color, axis, direction}
    }

    fn getOrigin(&self, ray:Ray) -> Option<Vec<f64>> {
        if (self.axis == 'X') {
            return Some(vec![ray.origin.y, ray.origin.z, ray.origin.x]);
        } else if (self.axis == 'Y') {
            return Some(vec![ray.origin.x, ray.origin.z, ray.origin.y]);
        } else if (self.axis == 'Z') {
            return Some(vec![ray.origin.x, ray.origin.y, ray.origin.z]);
        } else {
            return None;
        }
    }

    fn getCenter(&self, ray:Ray) -> Option<Vec<f64>> {
        if (self.axis == 'X') {
            return Some(vec![self.center.y, self.center.z, self.center.x]);
        } else if (self.axis == 'Y') {
            return Some(vec![self.center.x, self.center.z, self.center.y]);
        } else if (self.axis == 'Z') {
            return Some(vec![self.center.x, self.center.y, self.center.z]);
        } else {
            return None;
        }
    }

    fn getDirection(&self, ray:Ray) -> Option<Vec<f64>> {
        if (self.axis == 'X') {
            return Some(vec![ray.direction.y, ray.direction.z, ray.direction.x]);
        } else if (self.axis == 'Y') {
            return Some(vec![ray.direction.x, ray.direction.z, ray.direction.y]);
        } else if (self.axis == 'Z') {
            return Some(vec![ray.direction.x, ray.direction.y, ray.direction.z]);
        } else {
            return None;
        }
    }
}

impl Primitives for Cone {
    fn hits(&self, ray:Ray) -> Option<Point3D> {
        let mut origin: Vec<f64> = Vec::new();
        let mut center: Vec<f64> = Vec::new();
        let mut direction: Vec<f64> = Vec::new();

        match self.getOrigin(ray) {
            Some(result) => origin = result,
            None => return None
        }
        match self.getCenter(ray) {
            Some(result) => center = result,
            None => return None
        }
        match self.getDirection(ray) {
            Some(result) => direction = result,
            None => return None
        }

        let mut a1 = origin[0] + center[0];
        let mut b1 = origin[1] + center[1];
        let mut d1 = self.height - origin[2] + center[2];

        let tan:f64 = (self.radius / self.height) * (self.radius / self.height);

        let mut a2 = direction[0].powf(2.0) + direction[1].powf(2.0) - tan * direction[2].powf(2.0);
        let mut b2 = 2.0 * a1 * direction[0] + (2.0 * b1 * direction[1]) + (2.0 * tan * d1 * direction[2]);
        let mut c = a1.powf(2.0) + b1.powf(2.0) - (tan * d1.powf(2.0));

        c = a1 * a1 + b1 * b1 - (tan * d1.powf(2.0));
        let delta:f64 = b2.powf(2.0) - 4.0 * (a2 * c);
        if delta.abs() < 0.001 {
            return None;
        }
        if delta < 0.0 {
            return None;
        }
        let t1:f64 = (-b2 - delta.sqrt()) / (2.0 * a2);
        let t2:f64 = (-b2 + delta.sqrt()) / (2.0 * a2);
        let mut t:f64;

        if t1 > t2 {
            t = t2;
        } else {
            t = t1;
        }

        let r = origin[2] + t * direction[2];

        if ( r >= center[2] && r <= center[2] + self.height) {
            return Some(Point3D::default());
        }
        return None;
    }
    fn translate(&mut self, translate:Vector3D) {
        self.center.x += &translate.x;
        self.center.y += &translate.y;
        self.center.z += &translate.z;
    }
    fn rotatex(&mut self, _:f64) {}
    fn rotatey(&mut self, _:f64) {}
    fn rotatez(&mut self, _:f64) {}
    fn suface_normal(&self, _:Point3D) -> Vector3D {
        return Vector3D::default();
    }
    fn get_color(&self) -> Vector3D {
        self.color
    }
}

impl Default for Cone {
    fn default() -> Self {
        Cone {
            center: Point3D::default(),
            radius: 0.0,
            height: 0.0,
            color: Vector3D::default(),
            axis: 'Z',
            direction: Vector3D::default()
        }
    }
}