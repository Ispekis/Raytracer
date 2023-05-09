//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// sphere
//

use crate::Math::{Point3D::Point3D, formulas, Vector3D::Vector3D};
use crate::RayTracer::Ray::Ray;
use crate::Interfaces::Primitives::Primitives;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center:Point3D,
    pub radius:f64,
    pub color:Vector3D,
}

impl Sphere {
    pub fn new(center:Point3D, radius:f64) -> Sphere {
        return Sphere {center, radius, color:Vector3D::default()};
    }

    pub fn new_config(center:Point3D, radius:f64, color:Vector3D) -> Self {
        Sphere {center, radius, color}
    }
}

impl Primitives for Sphere {
    fn hits(&self, ray:Ray) -> bool{
        let dist:Vector3D = ray.origin - self.center;

        let a = ray.direction.scal(&ray.direction);
        let b = 2.0 * ray.direction.scal(&dist);
        let c = dist.scal(&dist) - self.radius * self.radius;
        let dis = b.powf(2.0) - 4.0 * a * c;
        // t1 = (-b - np.sqrt(discriminant)) / (2*a)
// t2 = (-b + np.sqrt(discriminant)) / (2*a)
        // let B:Vector3D = dist * ray.direction;
        // let a = (B.x * B.x) + (B.y * B.y)  + (B.z * B.z) ;
        // let b = (dist.x * dist.x) + (dist.y * dist.y)  + (dist.z * dist.z);
        // let dis = a - b + self.radius * self.radius;
    
        // let dif = ray.origin - self.center;
        // let a = ray.direction.scal(&ray.direction);
        // let b = 2.0 * dif.scal(&ray.direction);
        // let c = dif.scal(&dif) - self.radius.powf(2.0);
        // let dis = formulas::compute_discriminant(a, b, c);
        if dis > 0.0 {
            // let t1:f64 = (-b - f64::sqrt(dis)) / (2.0*a);
            // let t2:f64 = (-b + f64::sqrt(dis)) / (2.0*a);
            // if (t1 < 0.0 && t2 < 0.0) {
            //     return false;
            // }
            return true;
        }
        return false;
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

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: Point3D::default(),
            radius: 0.0,
            color: Vector3D::default()
        }
    }
}