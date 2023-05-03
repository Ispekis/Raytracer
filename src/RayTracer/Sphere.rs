//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// sphere
//

use crate::Math::Point3D::Point3D;
use crate::RayTracer::ray::ray;

pub struct Sphere {
    pub point:Point3D,
    pub radius:f64
}

impl Sphere {
    pub fn new(point:Point3D, radius:f64) -> Sphere {
        return Sphere {point, radius};
    }

    pub fn hits(&self, ray:ray) -> bool {
        return false;
    }
}