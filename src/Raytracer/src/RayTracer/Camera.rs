//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Camera
//

use crate::Math::Point3D::Point3D;
use crate::Math::Vector3D::Vector3D;
use crate::RayTracer::Rectangle3D::Rectangle3D;
use crate::RayTracer::Ray::Ray;

pub struct Camera {
    origin:Point3D,
    screen:Rectangle3D,
    pub width:u32,
    pub height:u32,
    pub rotation:Vector3D,
    pub fov:f64
}

impl Camera {
    pub fn new_config(width:u32, height:u32, position:Point3D, rotation:Vector3D, fov:f64) -> Self{
        Camera {
            origin: position,
            screen: Rectangle3D::new(Point3D::new(-500.0, -500.0, 2.0), Vector3D::new(width as f64, 0.0, 0.0), Vector3D::new(0.0, height as f64, 0.0)),
            width,
            height,
            rotation,
            fov
        }
    }

    pub fn ray(&self, u:f64, v:f64) -> Ray {
        let point = self.screen.pointAt(u, v);
        Ray { origin:self.origin, direction: (point - self.origin) }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            origin:Point3D::default(),
            screen:Rectangle3D::default(),
            width:0,
            height:0,
            rotation:Vector3D::default(),
            fov:0.0
        }
    }
}
