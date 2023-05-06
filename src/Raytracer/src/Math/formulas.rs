//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// formulas
//

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
