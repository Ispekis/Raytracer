//
// EPITECH PROJECT, 2023
// ù
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

mod Plane;

pub fn entryPoint() -> Plane::Plane {
    return Plane::Plane::default();
}
