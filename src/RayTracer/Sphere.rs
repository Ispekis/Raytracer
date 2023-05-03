//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// sphere
//

use crate::Math::Point3D::Point3D;
use crate::RayTracer::Ray::Ray;
use crate::Rgb::Rgb;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center:Point3D,
    pub radius:f64,
    // pub color:Rgb
}

impl Sphere {
    pub fn new(center:Point3D, radius:f64) -> Sphere {
        return Sphere {center, radius};
    }

    pub fn hits(&self, ray:Ray) -> bool {
        // println!("rad = {}, cx = {}, cy = {}, cz = {}", self.radius, self.center.x, self.center.y, self.center.z);
        let a = ray.vector.x.powf(2.0) + ray.vector.y.powf(2.0) + ray.vector.z.powf(2.0);
        let b = 2.0 * ((ray.point.x * ray.vector.x) + (ray.point.y * ray.vector.y) + (ray.point.z * ray.vector.z));
        let c = (ray.point.x.powf(2.0) + ray.point.y.powf(2.0) + ray.point.z.powf(2.0)) - self.radius;
        let dis = b.powf(2.0) - 4.0 * a * c;
        // println!("dis = {}", dis);
        if (dis >= 0.0) {
            return  true;
        }
        return false;
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: Point3D::default(),
            radius: 0.0,
        }
    }
}