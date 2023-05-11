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
    fn hits(&self, ray:Ray) -> bool{
        // let oc = ray.origin - self.center;
        // let a: f64 = ray.direction.scal(&ray.direction) - (ray.direction.scal(&oc).powf(2.0));
        // let b: f64 = 2.0 * (ray.direction.scal(&oc) - oc.scal(&ray.direction) * (oc.scal(&oc) - self.radius.powf(2.0).sqrt()));
        // let c: f64 = oc.scal(&oc) - self.radius.powf(2.0) - (oc.scal(&ray.direction)).powf(2.0);

        // let dis = b.powf(2.0) - 4.0 * a * c;
        // if dis < 0.0 {
        //     return false;
        // }
        // let t1 = (-b - dis.sqrt()) / (2.0 * a);
        // let t2 = (-b + dis.sqrt()) / (2.0 * a);

        // if t1 >= 0.0 && t2 >= 0.0 {
        //     let t = t1.min(t2);
        // } else {
        //     let t = t1.max(t2);
        // }
        // return true;

        let a = ray.direction.x.powi(2) + ray.direction.y.powi(2);
        let b = 2.0 * (ray.direction.x * (ray.origin.x - self.center.x) + ray.direction.y * (ray.origin.y - self.center.y));
        let c = (ray.origin.x - self.center.x).powi(2) + (ray.origin.y - self.center.y).powi(2) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            // return None;
            return false;
        }

        let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

        let mut t_min = f64::INFINITY;
        let mut t_max = f64::NEG_INFINITY;

        if t0 >= 0.0 && t0 <= self.height {
            t_min = t_min.min(t0);
            t_max = t_max.max(t0);
        }

        if t1 >= 0.0 && t1 <= self.height {
            t_min = t_min.min(t1);
            t_max = t_max.max(t1);
        }

        if t_min == f64::INFINITY || t_max == f64::NEG_INFINITY {
            // return None;
            return false;
        }

        // Some(t_min)
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