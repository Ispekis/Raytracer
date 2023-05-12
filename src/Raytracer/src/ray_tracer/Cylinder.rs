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
        // if (tmp.axis == 'X') {
            // let tmp = tmp.center.y;
            // &tmp.center.x = &tmp.center.y;
// 
        // }
        let definex = 'x';


        let a = (ray.direction.y).powi(2) + (ray.direction.z).powi(2);
        let b = 2.0 * (ray.direction.y * (ray.origin.y - self.center.y) + ray.direction.z *(ray.origin.z - self.center.z));
        let c = (ray.origin.y - self.center.y).powi(2) + (ray.origin.z - self.center.z).powi(2) - self.radius.powi(2);
        // let v = formulas::suface_normal_vector(ray.direction);
        // let a: f64 = v.x.powi(2) + v.y.powi(2);
        // let b = 2.0 * (ray.origin.x * v.x + v.y * ray.origin.y);
        // // let b: f64 = 2.0 * (v.x * (ray.origin.x - self.center.x) + v.y * (ray.origin.y - self.center.y));
        // let c: f64 = (ray.origin.x).powi(2) + (ray.origin.y).powi(2) - self.radius.powi(2);

        let delta = b*b - 4.0*a*c;
        // let disc = b.powi(2) - 4.0 * a * c;

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

        let r = ray.origin.x + t * ray.direction.x;

        if ( r >= self.center.x && r <= self.center.x + self.height) {
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
        // let y = ray.origin.y + t * ray.direction.y;
        // let z = ray.origin.z + t * ray.direction.z;
        // if y >= self.center.y && y <= self.center.y + self.height {
        //     return Some(Point3D::default());
        // } else {
        //     return None;
        // }
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
            color: Vector3D::default(),
            axis: 'Z',
        }
    }
}