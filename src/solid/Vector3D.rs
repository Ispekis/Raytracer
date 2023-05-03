//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Vector3D
//

pub struct Vector3D {
    pub x:f64,
    pub y:f64,
    pub z:f64,
}

impl Vector3D {
    pub fn new_default() -> Vector3D {
        return Vector3D { x: 0.0, y: 0.0, z: 0.0};
    }

    pub fn new(x:f64, y:f64, z:f64) -> Vector3D {
        return Vector3D {x, y, z};
    }
}

pub fn length(vector : &Vector3D) -> f64 {
    let res: f64 = (vector.x.powf(2.0) + vector.y.powf(2.0) + vector.z.powf(2.0)).sqrt();
    return res;
}

impl std::ops::Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        return Vector3D {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z};
    }
}

impl std::ops::AddAssign<Vector3D> for Vector3D {
    fn add_assign(&mut self, rhs: Vector3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        return Vector3D {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z};
    }
}

impl std::ops::SubAssign<Vector3D> for Vector3D {
    fn sub_assign(&mut self, rhs: Vector3D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        return Vector3D {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z};
    }
}

impl std::ops::MulAssign<Vector3D> for Vector3D {
    fn mul_assign(&mut self, rhs: Vector3D) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl std::ops::Div<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: Vector3D) -> Self::Output {
        return Vector3D {x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z};
    }
}

impl std::ops::DivAssign<Vector3D> for Vector3D {
    fn div_assign(&mut self, rhs: Vector3D) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
