//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Light
//

use crate::math::{
    Point3D::Point3D,
    Vector3D::Vector3D
};

#[derive(Copy, Clone)]
pub struct PointLight {
    pub origin:Point3D,
    pub color:Vector3D,
    pub intensity:f64
}

pub struct Light {
    pub ambient:f64,
    pub diffuse:f64,
    pub specular:f64,
    pub point:Vec<PointLight>,
    pub directional:Vec<Vector3D>
}

impl Light {
    pub fn new_config(ambient:f64, diffuse:f64, specular:f64, point:Vec<PointLight>, directional:Vec<Vector3D>) -> Self {
        Light { ambient, diffuse, specular, point, directional }
    }
}

impl Default for Light {
    fn default() -> Self {
        Light {
            ambient: 0.0,
            diffuse: 0.0,
            specular: 0.0,
            point: Vec::new(),
            directional: Vec::new()
        }
    }
}