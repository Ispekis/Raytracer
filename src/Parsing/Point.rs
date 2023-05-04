//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Point
//



pub struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl Point {
    pub fn new(x:u32, y:u32, z:u32) -> Point {
        return Point {x, y, z}
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}