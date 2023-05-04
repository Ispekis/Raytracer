//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Camera
//

use crate::Math::Point3D::Point3D;
use crate::Math::Vector3D::Vector3D;
use crate::Rectangle3D::Rectangle3D;
use crate::RayTracer::Ray::Ray;

pub struct Camera {
    origin:Point3D,
    screen:Rectangle3D
}

impl Camera {
    pub fn new(&self, origin:Point3D, screen:Rectangle3D) -> Self {
        Camera {origin, screen}
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
            // screen:Rectangle3D::default(),
            screen:Rectangle3D::new(Point3D::new(-0.96, -0.54, 1.0), Vector3D::new(1.92, 0.0, 0.0), Vector3D::new(0.0, 1.08, 0.0))
        }
    }
}
