//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// sphere
//

use crate::math::{point3d::Point3D, formulas, vector3d::Vector3D};
use crate::ray_tracer::ray::Ray;
use crate::interfaces::primitives::Primitives;
use crate::ray_tracer::material::{Mask, Solid};

// #[derive(Clone)]
pub struct Sphere {
    pub center:Point3D,
    pub radius:f64,
    pub color:Vector3D,
    pub pattern:Box<dyn Mask>
}

impl Sphere {
    // pub fn new(center:Point3D, radius:f64) -> Sphere {
    //     return Sphere {center, radius, color:Vector3D::default()};
    // }

    pub fn new_config(center:Point3D, radius:f64, color:Vector3D, pattern:Box<dyn Mask>) -> Self {
        Sphere {center, radius, color, pattern}
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
    fn translate(&mut self, translate:Vector3D) {
        self.center.x += &translate.x;
        self.center.y += &translate.y;
        self.center.z += &translate.z;
    }
    fn rotatex(&mut self, _:f64) {}
    fn rotatey(&mut self, _:f64) {}
    fn rotatez(&mut self, _:f64) {}
    fn suface_normal(&self, hit_point:Point3D) -> Vector3D {
        (hit_point - self.center).normalize()
    }
    fn get_color(&self) -> Vector3D {
        self.color
    }

    fn get_pattern(&self) -> Box<dyn Mask> {
        self.pattern.clone()
    }
}

impl Clone for Sphere {
    fn clone(&self) -> Self {
        Sphere {
            center: self.center.clone(),
            radius: self.radius,
            color: self.color.clone(),
            pattern: self.pattern.clone()
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: Point3D::default(),
            radius: 0.0,
            color: Vector3D::default(),
            pattern: Box::new(Solid::default())
        }
    }
}