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
        let b = 2.0 * dif.scal(&ray.direction);
        let c = dif.scal(&dif) - self.radius.powf(2.0);
        let dis = formulas::compute_discriminant(a, b, c);
        let res = formulas::resolve_quadratic_eq(dis, a, b);
        if (res == None) {
            return None;
        } else {
            let inter_points = formulas::get_inter_point_from_eq(res.unwrap(), ray.origin, ray.direction);
            return Some(formulas::get_closest_point(inter_points, ray.origin));
        }
    }
    fn hits_get_t(&self, ray:Ray) -> Option<f64> {
        let dif = ray.origin - self.center;
        let a = ray.direction.scal(&ray.direction);
        let b = 2.0 * ray.direction.scal(&dif);
        let c = dif.scal(&dif) - self.radius.powf(2.0);
        let dis = formulas::compute_discriminant(a, b, c);
        let res = formulas::resolve_quadratic_eq(dis, a, b);
        if let Some(v) = res {
            let tmp = v.clone();
            let inter_points: Vec<Point3D> = formulas::get_inter_point_from_eq(v, ray.origin, ray.direction);
            let closest_hit_point_index = formulas::get_closest_point_index(inter_points, ray.origin);
            return Some(tmp[closest_hit_point_index]);
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