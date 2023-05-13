//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Cone
//

use crate::math::{vector3d::Vector3D, point3d::Point3D};
use crate::interfaces::primitives::Primitives;
use crate::ray_tracer::ray::Ray;
use crate::math::formulas;

#[derive(Copy, Clone)]
pub struct Cone {
    pub center:Point3D,
    pub radius:f64,
    pub height:f64,
    pub color:Vector3D,
    pub axis:char,
    pub direction:Vector3D
}

impl Cone {
    pub fn new_config(center:Point3D, radius:f64, height: f64, color:Vector3D, axis:char) -> Self {
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
        Cone {center, radius, height, color, axis, direction}
    }
}

impl Primitives for Cone {
    fn hits(&self, ray:Ray) -> Option<Point3D> {
        let mut a1 = 0.0;
        let mut b1 = 0.0;
        let mut d1 = 0.0;

        let tan:f64 = (self.radius / self.height) * (self.radius / self.height);

        let mut a2 = 0.0;
        let mut b2 = 0.0;
        let mut c = 0.0;
        if (self.axis == 'X') {
            a1 = ray.origin.z - self.center.z;
            b1 = ray.origin.y - self.center.y;
            d1 = self.height - ray.origin.x + self.center.x;
            a2 = (ray.direction.z).powi(2) + (ray.direction.y).powi(2) - (tan * ray.direction.x);
            b2 = (2.0 * a1 * ray.direction.z) + (2.0 * b1 * ray.direction.y) + (2.0 * (tan * d1 * ray.direction.x));
        }
        else if (self.axis == 'Y') {
            a1 = ray.origin.x - self.center.x;
            b1 = ray.origin.z - self.center.z;
            d1 = self.height - ray.origin.y + self.center.y;
            a2 = (ray.direction.x).powi(2) + (ray.direction.z).powi(2) - tan * ray.direction.y;
            b2 = (2.0 * a1 * ray.direction.x) + (2.0 * b1 * ray.direction.z) + (2.0 * (tan * d1 * ray.direction.y));
        }
        else {
            a1 = ray.origin.y - self.center.y;
            b1 = ray.origin.x - self.center.x;
            d1 = self.height - ray.origin.z + self.center.z;
            a2 = (ray.direction.y).powi(2) + (ray.direction.x).powi(2) - tan * ray.direction.z;
            b2 = (2.0 * a1 * ray.direction.y) + (2.0 * b1 * ray.direction.x) + (2.0 * (tan * d1 * ray.direction.z));
        }
        c = a1 * a1 + b1 * b1 - (tan * d1 * d1);
        let delta:f64 = b2.powf(2.0) - 4.0 * (a2 * c);
        if delta.abs() < 0.001 {
            return None;
        }
        if delta < 0.0 {
            return None;
        }
        let t1:f64 = (-b2 - delta.sqrt()) / (2.0 * a2);
        let t2:f64 = (-b2 + delta.sqrt()) / (2.0 * a2);
        let mut t:f64;

        if t1 > t2 {
            t = t2;
        } else {
            t = t1;
        }

        let mut r1 = 0.0;
        let mut r2 = 0.0;
        let mut r3 = 0.0;
        if (self.axis == 'X') {
           r1 = ray.origin.x;
           r2 = ray.direction.x;
           r3 = self.center.x;
        }
        else if (self.axis == 'Y') {
           r1 = ray.origin.y;
           r2 = ray.direction.y;
           r3 = self.center.y;
        }
        else {
           r1 = ray.origin.z;
           r2 = ray.direction.z;
           r3 = self.center.z;
        }

        let r:f64 = r1 + t * r2;

        if r >= r3 && r <= r3 + self.height {
            return Some(Point3D::default());
        } else {
            return None;
        }
        // let tmpRay = ray.direction;
        // let mut tmp = self;
        // let definex = 'x';
        // let mut a = 0.0;
        // let mut b = 0.0;
        // let mut c = 0.0;
        // if (self.axis == 'X') {
        //     a = (ray.direction.z).powi(2) + (ray.direction.y).powi(2);
        //     b = 2.0 * (ray.direction.z * (ray.origin.z - self.center.z) + ray.direction.y *(ray.origin.y - self.center.y));
        //     c = (ray.origin.z - self.center.z).powi(2) + (ray.origin.y - self.center.y).powi(2) - self.radius.powi(2);
        // }
        // else if (self.axis == 'Y') {
        //     a = (ray.direction.x).powi(2) + (ray.direction.z).powi(2);
        //     b = 2.0 * (ray.direction.x * (ray.origin.x - self.center.x) + ray.direction.z *(ray.origin.z - self.center.z));
        //     c = (ray.origin.x - self.center.x).powi(2) + (ray.origin.z - self.center.z).powi(2) - self.radius.powi(2);
        // }
        // else {
        //     a = (ray.direction.x).powi(2) + (ray.direction.y).powi(2);
        //     b = 2.0 * (ray.direction.x * (ray.origin.x - self.center.x) + ray.direction.y *(ray.origin.y - self.center.y));
        //     c = (ray.origin.x - self.center.x).powi(2) + (ray.origin.y - self.center.y).powi(2) - self.radius.powi(2);
        // }
        // let delta = b*b - 4.0*a*c;

        // if (delta.abs() < 0.001) {
        //     return None;
        // }
        // if (delta < 0.0) {
        //     return None;
        // }

        // let t1 = (-b - delta.sqrt()) / (2.0 * a);
        // let t2 = (-b + delta.sqrt()) / (2.0 * a);
        // let mut t = 0.0;

        // if (t1 > t2) {
        //     t = t2;
        // } else {
        //     t = t1;
        // }

        // let mut a1 = 0.0;
        // let mut a2 = 0.0;
        // let mut a3 = 0.0;

        // if (self.axis == 'X') {
        //     a1 = ray.origin.x;
        //     a2 = ray.direction.x;
        //     a3 = self.center.x;
        // }
        // else if (self.axis == 'Y') {
        //     a1 = ray.origin.y;
        //     a2 = ray.direction.y;
        //     a3 = self.center.y;
        // }
        // else {
        //     a1 = ray.origin.z;
        //     a2 = ray.direction.z;
        //     a3 = self.center.z;
        // }

        // let r = a1 + t * a2;

        // if ( r >= a3 && r <= a3 + self.height) {
        //     return Some(Point3D::default());
        // }
        // let dif = ray.origin - self.center;
        // let a = ray.direction.scal(&ray.direction);
        // let b = 2.0 * ray.direction.scal(&dif);
        // let c = dif.scal(&dif) - self.radius.powf(2.0);
        // let des = formulas::compute_discriminant(a, b, c);
        // let res = formulas::resolve_quadratic_eq(des, a, b);
        // if let Some(v) = res {
        //     let inter_points = formulas::get_inter_point_from_eq(v, ray.origin, ray.direction);
        //     return Some(formulas::get_closest_point(inter_points, ray.origin));
        // }
        return None;
    }
    fn translate(&mut self, translate:Vector3D) {
        self.center.x += &translate.x;
        self.center.y += &translate.y;
        self.center.z += &translate.z;
    }
    fn rotatex(&mut self, _:f64) {}
    fn rotatey(&mut self, _:f64) {}
    fn rotatez(&mut self, _:f64) {}
    fn suface_normal(&self, _:Point3D) -> Vector3D {
        return Vector3D::default();
    }
    fn get_color(&self) -> Vector3D {
        self.color
    }
}

impl Default for Cone {
    fn default() -> Self {
        Cone {
            center: Point3D::default(),
            radius: 0.0,
            height: 0.0,
            color: Vector3D::default(),
            axis: 'Z',
            direction: Vector3D::default()
        }
    }
}