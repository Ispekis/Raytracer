//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Cylinder
//

use crate::Math::{Point3D::Point3D, formulas, Vector3D::Vector3D};
use crate::RayTracer::Ray::Ray;
use crate::Interfaces::Primitives::Primitives;

#[derive(Copy, Clone)]
pub struct Cylinder {
    pub center:Point3D,
    pub radius:f64,
    pub height:f64,
    pub color:Vector3D,
}

impl Cylinder {
    pub fn new(center:Point3D, radius:f64, height:f64) -> Cylinder {
        return Cylinder {center, radius, height, color:Vector3D::default()};
    }

    pub fn new_config(center:Point3D, radius:f64, height: f64, color:Vector3D) -> Self {
        Cylinder {center, radius, height, color}
    }
}

impl Primitives for Cylinder {
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
        // let oc = ray.origin - self.center;
        // let a: f64 = ray.direction.scal(&ray.direction) - (ray.direction.scal(&oc).powf(2.0));
        // let b: f64 = 2.0 * (ray.direction.scal(&oc) - oc.scal(&ray.direction) * (oc.scal(&oc) - self.radius.powf(2.0).sqrt()));
        // let c: f64 = oc.scal(&oc) - self.radius.powf(2.0) - (oc.scal(&ray.direction)).powf(2.0);

        // let dis = b.powf(2.0) - 4.0 * a * c;
        // if dis < 0.0 {
        //     return false;
        // }
        // let t1 = (-b - dis.sqrt()) / (2.0 * a);
        // let t2 = (-b + dis.sqrt()) / (2.0 * a);

        // if t1 >= 0.0 && t2 >= 0.0 {
        //     let t = t1.min(t2);
        // } else {
        //     let t = t1.max(t2);
        // }
        // return true;

        // let a = ray.direction.x.powi(2) + ray.direction.y.powi(2);
        // let b = 2.0 * (ray.direction.x * (ray.origin.x - self.center.x) + ray.direction.y * (ray.origin.y - self.center.y));
        // let c = (ray.origin.x - self.center.x).powi(2) + (ray.origin.y - self.center.y).powi(2) - self.radius.powi(2);
        // let discriminant = b.powi(2) - 4.0 * a * c;

        // if discriminant < 0.0 {
        //     return None;
        // }

        // let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
        // let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

        // let mut t_min = f64::INFINITY;
        // let mut t_max = f64::NEG_INFINITY;

        // if t0 >= 0.0 && t0 <= self.height {
        //     t_min = t_min.min(t0);
        //     t_max = t_max.max(t0);
        // }

        // if t1 >= 0.0 && t1 <= self.height {
        //     t_min = t_min.min(t1);
        //     t_max = t_max.max(t1);
        // }

        // if t_min == f64::INFINITY || t_max == f64::NEG_INFINITY {
        //     return None;
        // }

        // Some(t_min)
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
        let direction = (hit_point - self.center);
        let norme = (direction.x * direction.x + direction.y * direction.y + direction.z * direction.z).sqrt();
        return Vector3D::new(direction.x / norme, direction.y / norme, direction.z / norme)
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: Point3D::default(),
            radius: 0.0,
            height: 0.0,
            color: Vector3D::default()
        }
    }
}