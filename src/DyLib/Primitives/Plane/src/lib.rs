//
// EPITECH PROJECT, 2023
// ù
// File description:
// lib
//

mod Math {
    pub mod Point3D;
    pub mod Vector3D;
    pub mod formulas;
}

mod Interfaces {
    pub mod Primitives;
}

mod RayTracer {
    pub mod Plane;
    pub mod Ray;
}

pub fn entryPoint() -> RayTracer::Plane::Plane {
    return RayTracer::Plane::Plane::default();
}
