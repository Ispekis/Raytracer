//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Rectangle
//

use crate::Math::Point3D::Point3D;
use crate::Math::Vector3D::Vector3D;

pub struct Rectangle3D {
    origin:Point3D,
    bottom_side:Vector3D,
    left_side:Vector3D
}

impl Rectangle3D {
    pub fn pointAt(&self, u:f64, v:f64) -> Point3D {
        return Point3D::default();
    }
}

impl Default for Rectangle3D {
    fn default() -> Self {
        Rectangle3D {
            origin: Point3D::default(),
            bottom_side: Vector3D::default(),
            left_side: Vector3D::default()
        }
    }
}