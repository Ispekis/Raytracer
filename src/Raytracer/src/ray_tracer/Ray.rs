//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// ray
//

use crate::math::Point3D::Point3D;
use crate::math::Vector3D::Vector3D;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin:Point3D,
    pub direction:Vector3D,
}

impl Ray {
    pub fn new(origin:Point3D, direction:Vector3D) -> Ray {
        return Ray {origin, direction};
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray {origin:Point3D::default(), direction:Vector3D::default() }
    }
}