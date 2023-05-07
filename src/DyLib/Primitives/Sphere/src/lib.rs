//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
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
    pub mod Sphere;
    pub mod Ray;
}

pub fn entryPoint() -> RayTracer::Sphere::Sphere {
    return RayTracer::Sphere::Sphere::default();
}