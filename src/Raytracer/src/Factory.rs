//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Factory
//

use crate::RayTracer::Sphere::Sphere;
use crate::RayTracer::Plane::Plane;
use crate::Interfaces::Primitives::Primitives;

struct Factory {
}

impl Factory {
    fn createSphere(&self) -> Box<dyn Primitives> {
        Box::new(Sphere::default())
    }

    fn createPlane(&self) -> Box<dyn Primitives> {
        Box::new(Plane::default())
    }

    pub fn createPrimitive(&self, name:&str) -> Option<Box<dyn Primitives>> {
        if name == "sphere" {
            return Some(self.createSphere());
        }
        if name == "plane" {
            return Some(self.createPlane());
        }
        return None;
    }
}

impl Default for Factory {
    fn default() -> Self {
        Factory {  }
    }
}