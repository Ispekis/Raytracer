//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Point3D
//

<<<<<<< HEAD:src/solid/Point3D.rs
use crate::solid::Vector3D::Vector3D;
=======
use crate::Math::Vector3D::Vector3D;

>>>>>>> bd758ec (3 : feat : moving correcting struct to namespace):src/Math/Point3D.rs
pub struct Point3D {
    pub x:f64,
    pub y:f64,
    pub z:f64
}

impl Point3D {
    pub fn new_default() -> Point3D {
        return Point3D {x:0.0, y:0.0, z:0.0}
    }

    pub fn new(x:f64, y:f64, z:f64) -> Point3D {
        return Point3D {x, y, z};
    }
}

impl std::ops::Add<Vector3D> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        return Point3D {x:self.x + rhs.x , y:self.y + rhs.y, z:self.z + rhs.z};
    }
}

impl std::ops::Sub<Vector3D> for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        return Point3D {x:self.x - rhs.x , y:self.y - rhs.y, z:self.z - rhs.z};
    }
}

impl std::ops::Mul<Vector3D> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        return Point3D {x:self.x * rhs.x , y:self.y * rhs.y, z:self.z * rhs.z};
    }
}

impl std::ops::Div<Vector3D> for Point3D {
    type Output = Point3D;

    fn div(self, rhs: Vector3D) -> Self::Output {
        return Point3D {x:self.x / rhs.x , y:self.y / rhs.y, z:self.z / rhs.z};
    }
}