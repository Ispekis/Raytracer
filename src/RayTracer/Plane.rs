//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// planes
//

use crate::Math::Vector3D::Vector3D;

#[derive(Copy, Clone)]
pub struct Plane {
    pub axis:char,
    pub position:f64,
    pub color:Vector3D
}

impl Plane {
    pub fn new_config(axis:char, position:f64, color:Vector3D) -> Self {
        Plane {axis, position, color}
    }
}

impl Default for Plane {
    fn default() -> Self {
        Plane { axis: 'X', position: 0.0, color: Vector3D::default() }
    }
}

