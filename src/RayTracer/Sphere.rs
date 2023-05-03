//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// sphere
//

use crate::Math::Point3D::Point3D;
use crate::RayTracer::Ray::Ray;
use crate::Rgb::Rgb;

pub struct Sphere {
    pub center:Point3D,
    pub radius:f64,
    pub color:Rgb
}

impl Sphere {
    pub fn new(center:Point3D, radius:f64, color:Rgb) -> Sphere {
        return Sphere {center, radius, color};
    }

    pub fn hits(&self, ray:Ray) -> bool {
        return false;
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: Point3D::default(),
            radius: 0.0,
            color:Rgb::default()
        }
    }
}