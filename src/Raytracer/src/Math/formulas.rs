//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// formulas
//

use crate::Math::{Point3D::Point3D, Vector3D::Vector3D};

pub fn compute_discriminant(a:f64, b:f64, c:f64) -> f64 {
    return b.powf(2.0) - 4.0 * a * c;
}

pub fn resolve_quadratic_eq(delta:f64, a:f64, b:f64) -> Option<Vec<f64>>{
    let mut res: Vec<f64> = Vec::new();
    if (delta < 0.0) {
        return None;
    }
    if (delta == 0.0) {
        res.push(-b / 2.0 * a );
        return Some(res);
    }

    res.push((-b + delta) / (2.0 * a));
    res.push((-b - delta) / (2.0 * a));

    return Some(res);
}

pub fn get_inter_from_eq(eqs:Vec<f64>, point:Point3D, direction:Vector3D) -> Vec<Point3D>{
    let mut res: Vec<Point3D> = Vec::new();
    if eqs.len() == 1 {
        res.push(Point3D::new(point.x + eqs[0] * direction.x, point.y + eqs[0] * direction.y, point.z + eqs[0] * direction.z));
        return res;
    }
    if eqs.len() == 2 {
        res.push(Point3D::new(point.x + eqs[1] * direction.x, point.y + eqs[1] * direction.y, point.z + eqs[1] * direction.z));
        res.push(Point3D::new(point.x + eqs[1] * direction.x, point.y + eqs[1] * direction.y, point.z + eqs[1] * direction.z));
        return res;
    }
    return res;
}
