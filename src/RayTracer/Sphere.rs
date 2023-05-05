//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// sphere
//

use crate::Math::{Point3D::Point3D, formulas, Vector3D::Vector3D};
use crate::RayTracer::Ray::Ray;
use crate::Primitives::Primitives;

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
        let dif = ray.origin - self.center;
        // println!("rad = {}, cx = {}, cy = {}, cz = {}", self.radius, self.center.x, self.center.y, self.center.z);
        let a = ray.direction.x.powf(2.0) + ray.direction.y.powf(2.0) + ray.direction.z.powf(2.0);
        let b = 2.0 * dif.scal(&ray.direction);
        let c = dif.scal(&dif) - self.radius.powf(2.0);
        let dis = formulas::compute_discriminant(a, b, c);
        // println!("dis = {}", dis);
        if dis >= 0.0 {
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