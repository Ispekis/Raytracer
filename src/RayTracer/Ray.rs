//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// ray
//

use crate::Math::Point3D::Point3D;
use crate::Math::Vector3D::Vector3D;

#[derive(Copy, Clone)]
pub struct Ray {
    pub point:Point3D,
    pub vector:Vector3D,
}

impl Ray {
    pub fn new_default() -> Ray {
        return Ray {point:Point3D::default(), vector:Vector3D::default() }
    }

    pub fn new(point:Point3D, vector:Vector3D) -> Ray {
        return Ray {point, vector};
    }
}