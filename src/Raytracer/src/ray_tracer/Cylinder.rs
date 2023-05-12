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
}

impl Primitives for Cylinder {
    fn hits(&self, ray:Ray) -> Option<Point3D> {

        let tmpRay = ray.direction;
        let mut tmp = self;
        let definex = 'x';
        // let mut ;
        let mut a = 0.0;
        let mut b = 0.0;
        let mut c = 0.0;
        if (self.axis == 'X') {
            a = (ray.direction.z).powi(2) + (ray.direction.y).powi(2);
            b = 2.0 * (ray.direction.z * (ray.origin.z - self.center.z) + ray.direction.y *(ray.origin.y - self.center.y));
            c = (ray.origin.z - self.center.z).powi(2) + (ray.origin.y - self.center.y).powi(2) - self.radius.powi(2);
        }
        else if (self.axis == 'Y') {
            a = (ray.direction.x).powi(2) + (ray.direction.z).powi(2);
            b = 2.0 * (ray.direction.x * (ray.origin.x - self.center.x) + ray.direction.z *(ray.origin.z - self.center.z));
            c = (ray.origin.x - self.center.x).powi(2) + (ray.origin.z - self.center.z).powi(2) - self.radius.powi(2);
        }
        else {
            a = (ray.direction.x).powi(2) + (ray.direction.y).powi(2);
            b = 2.0 * (ray.direction.x * (ray.origin.x - self.center.x) + ray.direction.y *(ray.origin.y - self.center.y));
            c = (ray.origin.x - self.center.x).powi(2) + (ray.origin.y - self.center.y).powi(2) - self.radius.powi(2);
        }
        let delta = b*b - 4.0*a*c;
        if (delta.abs() < 0.001) {
            return None;
        }
        if (delta < 0.0) {
            return None;
        }
        let t1 = (-b - delta.sqrt()) / (2.0 * a);
        let t2 = (-b + delta.sqrt()) / (2.0 * a);
        let mut t = 0.0;

        if (t1 > t2) {
            t = t2;
        } else {
            t = t1;
        }

        let mut a1 = 0.0;
        let mut a2 = 0.0;
        let mut a3 = 0.0;
        if (self.axis == 'X') {
            a1 = ray.origin.x;
            a2 = ray.direction.x;
            a3 = self.center.x;
            
        }
        else if (self.axis == 'Y') {
            a1 = ray.origin.y;
            a2 = ray.direction.y;
            a3 = self.center.y;
        }
        else {
            a1 = ray.origin.z;
            a2 = ray.direction.z;
            a3 = self.center.z;
        }
        let r = a1 + t * a2;

        if ( r >= a3 && r <= a3 + self.height) {
            return Some(Point3D::default());
        }
        return None;
        // if disc < 0.0 {
        //     return None;
        // }

        // let t1 = (-b - disc.sqrt()) / (2.0 * a);
        // let t2 = (-b + disc.sqrt()) / (2.0 * a);
        // let mut t = t1.max(t2);

        // if t < 0.0 {
        //     return None;
        // }

        // let x = ray.origin.x + t * ray.direction.x;
        // let y = ray.origin.x + t * ray.direction.x;
        // let z = ray.origin.y + t * ray.direction.y;
        // if y >= self.center.x && y <= self.center.x + self.height {
        //     return Some(Point3D::default());
        // } else {
        //     return None;
        // }
        // if z.abs() <= 1.0 {
        //     return Some(Point3D::default());
        // }

        // if y < self.center.x || y > self.center.x + self.height {
        //     t = (-b + disc.sqrt()) / (2.0 * a);
        //     if t < 0.0 {
        //         return None;
        //     }
        //     let y = ray.origin.x + t * ray.direction.x;
        //     if y < self.center.x || y > self.center.x + self.height {
        //         return None;
        //     }
        // }

        // let mut hit_point = Point3D::new(0.0, 0.0, 0.0);

        // hit_point.x = ray.origin.x + t * ray.direction.x;
        // hit_point.x = ray.origin.x + t * ray.direction.x;
        // hit_point.y = ray.origin.y + t * ray.direction.y;

        // Some(hit_point)
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