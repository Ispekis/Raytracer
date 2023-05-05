//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// planes
//

use crate::Math::{Vector3D::Vector3D, Point3D::Point3D};

#[derive(Copy, Clone)]
pub struct Plane {
    pub axis:char,
    pub position:f64,
    // pub position:Point3D,
    pub color:Vector3D
}

impl Plane {
    pub fn new_config(axis:char, position:f64, color:Vector3D) -> Self {
        Plane {axis, position, color}
    }
    pub fn translate(&mut self, Translate:Vector3D) {
        if self.axis == 'X' {
            self.position += Translate.x;
        }
        if self.axis == 'Y' {
            self.position += Translate.y;
        }
        if self.axis == 'Z' {
            self.position += Translate.z;
        }
    }
    pub fn rotateX(&mut self, degres:f64) {
    }
    pub fn rotateY(&mut self, degres:f64) {
    }
    pub fn rotateZ(&mut self, degres:f64) {
    }
    
}

impl Default for Plane {
    fn default() -> Self {
        Plane { axis: 'X', position: 0.0, color: Vector3D::default() }
    }
}

