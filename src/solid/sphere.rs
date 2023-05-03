//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// spheres
//

use crate::solid::Point3D::Point3D;
use crate::solid::rgb::Rgb;
use crate::solid::Isolid::Solid;
pub struct Sphere {
    pub color:Rgb,
    pub point:Point3D,
    pub radius:u32,
}

impl Sphere {
    pub fn new_default() -> Sphere {
        return Sphere {color:Rgb::default(), point:Point3D::new_default(), radius:20}
    }
    pub fn new(color:Rgb, point:Point3D, radius:u32) -> Sphere{
        return Sphere {color, point, radius}
    }
}

impl Default for Sphere {
    fn default() -> Self {
    }
}
impl Solid for Sphere {
    fn getColor(&self) -> Rgb {
        return self.color;
    }
}