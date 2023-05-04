//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// sphere
//

use crate::Math::Point3D::Point3D;
use crate::Math::formulas;
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
        let dif = ray.origin - self.center;
        // println!("rad = {}, cx = {}, cy = {}, cz = {}", self.radius, self.center.x, self.center.y, self.center.z);
        let a = ray.direction.x.powf(2.0) + ray.direction.y.powf(2.0) + ray.direction.z.powf(2.0);
        let b = 2.0 * dif.scal(&ray.direction);
        let c = dif.scal(&dif) - self.radius.powf(2.0);
        let dis = formulas::compute_discriminant(a, b, c);
        // println!("dis = {}", dis);
        if dis >= 0.0 {
            return true;
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