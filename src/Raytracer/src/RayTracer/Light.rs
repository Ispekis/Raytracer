//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Light
//

use crate::Math::{
    Point3D::Point3D,
    Vector3D::Vector3D
};

pub struct Light {
    ambient:f64,
    diffuse:f64,
    point:Vec<Point3D>,
    directional:Vec<Vector3D>
}

impl Light {
    pub fn new_config(ambient:f64, diffuse:f64, point:Vec<Point3D>, directional:Vec<Vector3D>) -> Self {
        Light { ambient, diffuse, point, directional }
    }
}

impl Default for Light {
    fn default() -> Self {
        Light {
            ambient: 0.0,
            diffuse: 0.0,
            point: Vec::new(),
            directional: Vec::new()
        }
    }
}