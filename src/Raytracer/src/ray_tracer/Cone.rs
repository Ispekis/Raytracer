//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Cone
//

use crate::Math::{Vector3D::Vector3D, Point3D::Point3D};
use crate::Interfaces::Primitives::Primitives;
use crate::RayTracer::Ray::Ray;
use crate::Math::formulas;

#[derive(Copy, Clone)]
pub struct Cone {
    pub radius:f64,
    pub axis:char,
    pub center:Point3D,
    pub direction:Vector3D,
    pub color:Vector3D,
    pub height:f64
}

impl Cone {
    pub fn new_config(radius:f64, axis:char, center:Point3D, color:Vector3D, height:f64) -> Self {
        let mut direction = Vector3D::default();
        if axis == 'X' {
            direction.x = 1.0;
        }
        if axis == 'Y' {
            direction.y = 1.0;
        }
        if axis == 'Z' {
            direction.y = 1.0;
        }
        return Cone {radius, axis, center, direction, color, height};
    }
}

impl Primitives for Cone {
    fn hits(&self, ray:Ray) -> Option<Point3D> {
        let co: Vector3D = ray.origin - self.center;
        let a: f64 = (ray.direction.scal(&self.direction)).powf(2.0) - (self.radius.cos()).powf(2.0);
        let b: f64 = 2.0 * (((ray.direction.scal(&self.direction)) * (co.scal(&self.direction)) - ray.direction.scal(&co) * self.radius.cos().powf(2.0)));
        let c: f64 = (co.scal(&self.direction).powf(2.0)) - co.scal(&co) * self.radius.cos().powf(2.0);
        let dis: f64 = formulas::compute_discriminant(a, b, c);

        if dis < 0.0 {
            return None;
        } else {
            let det = dis.powf(2.0);
            let t1 = (-b - det) / (2.0 * a);
            let t2 = (-b + det) / (2.0 * a);

            let mut t: f64 = t1;
            if t < 0.0 || t2 > 0.0 && t2 < t {
                t = t2;
            }
            if t < 0.0 {
                return None;
            }

            let cp: Vector3D = ray.origin + ray.direction * t - self.center;
            let h: f64 = cp.scal(&self.direction);
            if h < 0.0 || h > self.height {
                return None;
            }
            let res = formulas::resolve_quadratic_eq(dis, a, b);
            let inter_points = formulas::get_inter_point_from_eq(res.unwrap(), ray.origin, ray.direction);
            return Some(formulas::get_closest_point(inter_points, ray.origin));
            // let inter_points = formulas::get_inter_point_from_eq(res.unwrap(), ray.origin, ray.direction);
            // return Some(formulas::get_closest_point(inter_points, ray.origin));
        }
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
        return Vector3D::default();
    }
}

impl Default for Cone {
    fn default() -> Self {
        Cone {
            radius: 0.5,
            axis: 'X',
            center: Point3D::default(),
            direction: Vector3D::default(),
            color: Vector3D::default(),
            height: 10.0
        }
    }
}