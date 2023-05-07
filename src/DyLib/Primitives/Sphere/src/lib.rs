//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// lib
//

#[path = "../../../../Raytracer/src/Math"]
mod Math {
    pub mod Point3D;
    pub mod Vector3D;
    pub mod formulas;
}

#[path = "../../../../Raytracer/src/Interfaces"]
mod Interfaces {
    pub mod Primitives;
}

#[path = "../../../../Raytracer/src/RayTracer"]
mod RayTracer {
    pub mod Ray;
}

mod Sphere;

pub fn entryPoint() -> Sphere::Sphere {
    return Sphere::Sphere::default();
}