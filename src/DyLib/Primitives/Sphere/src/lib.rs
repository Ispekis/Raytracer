//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// lib
//

#[path = "../../../../Raytracer/src/math"]
mod math {
    pub mod Point3D;
    pub mod Vector3D;
    pub mod formulas;
}

#[path = "../../../../Raytracer/src/interfaces"]
mod interfaces {
    pub mod Primitives;
}

#[path = "../../../../Raytracer/src/ray_tracer"]
mod ray_tracer {
    pub mod Ray;
}

mod Sphere;

pub fn entryPoint() -> Sphere::Sphere {
    return Sphere::Sphere::default();
}