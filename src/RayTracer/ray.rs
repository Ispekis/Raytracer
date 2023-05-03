//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// ray
//

use crate::Math::Point3D::Point3D;
use crate::Math::Vector3D::Vector3D;

pub struct ray {
    pub point:Point3D,
    pub vector:Vector3D,
}

impl ray {
    pub fn new_default() -> ray {
        return ray {point:Point3D::new_default(), vector:Vector3D::new_default() }
    }

    pub fn new(point:Point3D, vector:Vector3D) -> ray {
        return ray {point, vector};
    }
}