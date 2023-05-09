//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Vector3D
//

#[derive(Copy, Clone)]
pub struct Vector3D {
    pub x:f64,
    pub y:f64,
    pub z:f64,
}

impl Vector3D {
    pub fn new(x:f64, y:f64, z:f64) -> Vector3D {
        return Vector3D {x, y, z};
    }

    pub fn scal(&self, other: &Self) -> f64 {
        return (self.x * other.x) + (self.y * other.y) + (self.z * other.z);
    }

    pub fn scalother(other2: Vector3D, other1: Vector3D) -> f64 {
        return (other1.x * other2.x) + (other1.y * other2.y) + (other1.z * other2.z);
    }

    pub fn length(&self) -> f64 {
        let res: f64 = (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();
        return res;
    }

    pub fn cross(&self, other:Self) -> Self {
        let new_x = (self.y * other.z - other.y - self.z);
        let new_y = -(self.x * other.z - other.x * self.z);
        let new_z = (self.x * other.y - other.x - self.y);
        return Vector3D::new(new_x, new_y, new_z);
    }
}

impl Default for Vector3D {
    fn default() -> Self {
        Vector3D { x: (0.0), y: (0.0), z: (0.0) }
    }
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

impl std::ops::Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        return Vector3D {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs};
    }
}

impl std::ops::Mul<u32> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: u32) -> Self::Output {
        return Vector3D {x: self.x * (rhs as f64), y: self.y * (rhs as f64), z: self.z * (rhs as f64)};
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

impl std::fmt::Display for Vector3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector3D(x={}, y={}, z={})", self.x, self.y, self.z)
    }
}