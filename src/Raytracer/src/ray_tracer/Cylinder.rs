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
}

impl Cylinder {
    pub fn new(center:Point3D, radius:f64, height:f64) -> Cylinder {
        return Cylinder {center, radius, height, color:Vector3D::default()};
    }

    pub fn new_config(center:Point3D, radius:f64, height: f64, color:Vector3D) -> Self {
        Cylinder {center, radius, height, color}
    }
}

impl Primitives for Cylinder {
    fn hits(&self, ray:Ray) -> Option<Point3D> {
        let a: f64 = ray.direction.x.powi(2) + ray.direction.y.powi(2);
        let b: f64 = 2.0 * (ray.direction.x * (ray.origin.x - self.center.x) + ray.direction.y * (ray.origin.y - self.center.y));
        let c: f64 = (ray.origin.x - self.center.x).powi(2) + (ray.origin.y - self.center.y).powi(2) - self.radius.powi(2);

        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            return None;
        }

        let t1 = (-b - disc.sqrt()) / (2.0 * a);
        let t2 = (-b + disc.sqrt()) / (2.0 * a);
        let mut t = t1.max(t2);

        if t < 0.0 {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y >= self.center.y && y <= self.center.y + self.height {
            return Some(Point3D::default());
        } else {
            return None;
        }
        // if z.abs() <= 1.0 {
        //     return Some(Point3D::default());
        // }

        // if y < self.center.y || y > self.center.y + self.height {
        //     t = (-b + disc.sqrt()) / (2.0 * a);
        //     if t < 0.0 {
        //         return None;
        //     }
        //     let y = ray.origin.y + t * ray.direction.y;
        //     if y < self.center.y || y > self.center.y + self.height {
        //         return None;
        //     }
        // }

        // let mut hit_point = Point3D::new(0.0, 0.0, 0.0);

        // hit_point.x = ray.origin.x + t * ray.direction.x;
        // hit_point.y = ray.origin.y + t * ray.direction.y;
        // hit_point.z = ray.origin.z + t * ray.direction.z;

        // Some(hit_point)
    }
    fn translate(&mut self, Translate:Vector3D) {
        self.center.x += &Translate.x;
        self.center.y += &Translate.y;
        self.center.z += &Translate.z;
    }
    fn rotateX(&mut self, angle:f64) {}
    fn rotateY(&mut self, angle:f64) {}
    fn rotateZ(&mut self, angle:f64) {}
    fn suface_normal(&self, hit_point:Point3D) -> Vector3D {
        let direction = (hit_point - self.center);
        let norme = (direction.x * direction.x + direction.y * direction.y + direction.z * direction.z).sqrt();
        return Vector3D::new(direction.x / norme, direction.y / norme, direction.z / norme)
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder {
            center: Point3D::default(),
            radius: 0.0,
            height: 0.0,
            color: Vector3D::default()
        }
    }
}