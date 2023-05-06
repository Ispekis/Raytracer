//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Primitives
//

use crate::Math::Vector3D::Vector3D;
use crate::RayTracer::Ray::Ray;

pub trait Primitives {
    fn translate(&mut self, vec:Vector3D);
    fn rotateX(&mut self, angle:f64);
    fn rotateY(&mut self, angle:f64);
    fn rotateZ(&mut self, angle:f64);
    fn hits(&self, ray:Ray) -> bool;
}