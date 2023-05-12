//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// sphere
//

use crate::Math::{Point3D::Point3D, formulas, Vector3D::Vector3D};
use crate::RayTracer::Ray::Ray;
use crate::Interfaces::Primitives::Primitives;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center:Point3D,
    pub radius:f64,
    pub color:Vector3D,
}

impl Sphere {
    pub fn new(center:Point3D, radius:f64) -> Sphere {
        return Sphere {center, radius, color:Vector3D::default()};
    }

    pub fn new_config(center:Point3D, radius:f64, color:Vector3D) -> Self {
        Sphere {center, radius, color}
    }
}

impl Primitives for Sphere {
    fn hits(&self, ray:Ray) -> Option<Point3D> {
        let dif = ray.origin - self.center;
        let a = ray.direction.scal(&ray.direction);
        let b = 2.0 * ray.direction.scal(&dif);
        let c = dif.scal(&dif) - self.radius.powf(2.0);
        let des = formulas::compute_discriminant(a, b, c);
        let res = formulas::resolve_quadratic_eq(des, a, b);
        if let Some(v) = res {
            let inter_points = formulas::get_inter_point_from_eq(v, ray.origin, ray.direction);
            return Some(formulas::get_closest_point(inter_points, ray.origin));
        }
        None
    }
    fn translate(&mut self, Translate:Vector3D) {
        self.center.x += &Translate.x;
        self.center.y += &Translate.y;
        self.center.z += &Translate.z;
    }
    fn rotateX(&mut self, angle:f64) {}
    fn rotateY(&mut self, angle:f64) {}
    fn rotateZ(&mut self, angle:f64) {}
    fn suface_normal(&self, hit_point:Point3D) -> Vector3D {
        (hit_point - self.center).normalize()
    }
    fn get_color(&self) -> Vector3D {
        self.color
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: Point3D::default(),
            radius: 0.0,
            color: Vector3D::default()
        }
    }
}