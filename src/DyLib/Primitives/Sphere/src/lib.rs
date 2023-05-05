//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// lib
//

mod sphere;
mod Point3D;
mod Vector3D;
mod Ray;
mod formulas;

use crate::sphere::Sphere;

pub fn entryPoint() -> sphere::Sphere {
    return sphere::Sphere::default();
}