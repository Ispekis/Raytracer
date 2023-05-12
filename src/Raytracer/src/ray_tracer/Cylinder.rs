//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Cylinder
//

use crate::Math::{Point3D::Point3D, formulas, Vector3D::Vector3D};
use crate::RayTracer::Ray::Ray;
use crate::Interfaces::Primitives::Primitives;

#[derive(Copy, Clone)]
pub struct Cylinder {
    pub center:Point3D,
    pub radius:f64,
    pub height:f64,
    pub color:Vector3D,
    pub axis:char,
}

impl Cylinder {
    pub fn new(center:Point3D, radius:f64, height:f64, axis:char) -> Cylinder {
        return Cylinder {center, radius, height, color:Vector3D::default(), axis};
    }

    pub fn new_config(center:Point3D, radius:f64, height: f64, color:Vector3D, axis:char) -> Self {
        Cylinder {center, radius, height, color, axis}
    }

    fn computeA(&self, ray:Ray) -> Option<f64> {
        if (self.axis == 'X') {
            return Some(ray.direction.z.powi(2) + ray.direction.y.powi(2));
        } else if (self.axis == 'Y') {
            return Some(ray.direction.x.powi(2) + ray.direction.z.powi(2));
        } else if (self.axis == 'Z') {
            return Some(ray.direction.x.powi(2) + ray.direction.y.powi(2));
        } else {
            return None;
        }
    }

    fn computeB(&self, ray:Ray) -> Option<f64> {
        if (self.axis == 'X') {
            return Some(2.0 * (ray.direction.z * (ray.origin.z - self.center.z)
            + ray.direction.y *(ray.origin.y - self.center.y)));
        } else if (self.axis == 'Y') {
            return Some(2.0 * (ray.direction.x * (ray.origin.x - self.center.x)
            + ray.direction.z *(ray.origin.z - self.center.z)));
        } else if (self.axis == 'Z') {
            return Some(2.0 * (ray.direction.x * (ray.origin.x - self.center.x)
            + ray.direction.y *(ray.origin.y - self.center.y)));
        } else {
            return None;
        }
    }

    fn computeC(&self, ray:Ray) -> Option<f64> {
        if (self.axis == 'X') {
            return Some((ray.origin.z - self.center.z).powi(2) +
            (ray.origin.y - self.center.y).powi(2) - self.radius.powi(2));
        } else if (self.axis == 'Y') {
            return Some((ray.origin.x - self.center.x).powi(2) +
            (ray.origin.z - self.center.z).powi(2) - self.radius.powi(2));
        } else if (self.axis == 'Z') {
            return Some((ray.origin.x - self.center.x).powi(2) +
            (ray.origin.y - self.center.y).powi(2) - self.radius.powi(2));
        } else {
            return None;
        }
    }

    fn computeR(&self, ray:Ray) -> Option<Vec<f64>> {
        if (self.axis == 'X') {
            return Some(vec![ray.origin.x, ray.direction.x, self.center.x]);
        } else if (self.axis == 'Y') {
            return Some(vec![ray.origin.y, ray.direction.y, self.center.y]);
        } else if (self.axis == 'Z') {
            return Some(vec![ray.origin.z, ray.direction.z, self.center.z]);
        } else {
            return None;
        }
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

impl Primitives for Cylinder {
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

        let a: f64 = direction[0].powi(2) + direction[1].powi(2);

        let b: f64 = 2.0 * (direction[0] * (origin[0] - center[0])
        + direction[1] * (origin[1] - center[1]));

        let c: f64 = (origin[0] - center[0]).powi(2) +
        (origin[1] - center[1]).powi(2) - self.radius.powi(2);

        let delta: f64 = b.powi(2) - 4.0 * a * c;
        if (delta.abs() < 0.001 || delta < 0.0) {
            return None;
        }
        let t1: f64 = (-b - delta.sqrt()) / (2.0 * a);
        let t2: f64 = (-b + delta.sqrt()) / (2.0 * a);
        let t: f64 = t1.min(t2);

        let r = origin[2] + t * direction[2];

        if ( r >= center[2] && r <= center[2] + self.height) {
            return Some(Point3D::default());
        }
        return None;
    }
    fn translate(&mut self, Translate:Vector3D) {
        self.center.x += &Translate.x;
        self.center.x += &Translate.x;
        self.center.y += &Translate.y;
    }
    fn rotateX(&mut self, angle:f64) {}
    fn rotateY(&mut self, angle:f64) {}
    fn rotateZ(&mut self, angle:f64) {}
    fn suface_normal(&self, hit_point:Point3D) -> Vector3D {
        let direction = (hit_point - self.center);
        let norme = (direction.x * direction.x + direction.x * direction.x + direction.y * direction.y).sqrt();
        return Vector3D::new(direction.x / norme, direction.x / norme, direction.y / norme)
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder {
            center: Point3D::default(),
            radius: 0.0,
            height: 0.0,
            color: Vector3D::default(),
            axis: 'Z',
        }
    }
}