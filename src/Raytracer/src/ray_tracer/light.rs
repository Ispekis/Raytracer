//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Light
//

use crate::math::{
    point3d::Point3D,
    vector3d::Vector3D
};

#[derive(Copy, Clone)]
pub struct PointLight {
    pub origin:Point3D,
    pub color:Point3D,
    pub intensity:f64
}

pub struct DirectionnalLight {
    pub origin:Point3D,
    pub color:Point3D,
    pub intensity:f64,
    pub rotation:Point3D,
}

pub struct Light {
    pub ambient:f64,
    pub diffuse:f64,
    pub specular:f64,
    pub point:Vec<PointLight>,
    pub directional:Vec<DirectionnalLight>
}

impl Light {
    pub fn new_config(ambient:f64, diffuse:f64, specular:f64, point:Vec<PointLight>, directional:Vec<DirectionnalLight>) -> Self {
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

