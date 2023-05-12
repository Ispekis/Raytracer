//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Primitives
//

use crate::math::Point3D::Point3D;
use crate::math::Vector3D::Vector3D;
use crate::ray_tracer::Ray::Ray;

pub trait Primitives {
    fn translate(&mut self, vec:Vector3D);
    fn rotatex(&mut self, angle:f64);
    fn rotatey(&mut self, angle:f64);
    fn rotatez(&mut self, angle:f64);
    fn hits(&self, ray:Ray) -> Option<Point3D>;
    fn suface_normal(&self, hit_point:Point3D) -> Vector3D;
    fn get_color(&self) -> Vector3D;
}
