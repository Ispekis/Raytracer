//
// EPITECH PROJECT, 2023
// ù
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

mod Plane;

pub fn entryPoint() -> Plane::Plane {
    return Plane::Plane::default();
}
