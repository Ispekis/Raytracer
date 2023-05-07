//
// EPITECH PROJECT, 2023
// ù
// File description:
// lib
//

mod plane;
mod Point3D;
mod Vector3D;
mod Ray;

// use crate::Point3D::Point3D;
// use  crate::Vector3D::Vector3D;
use crate::plane::Plane;

pub fn entryPoint() -> plane::Plane {
    return plane::Plane::default();
}