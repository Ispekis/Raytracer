//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Directionnal
//



pub struct Directionnal {
    x: u32,
    y: u32,
    z: u32,
}

impl Directionnal {
    pub fn new(x:u32, y:u32, z:u32) -> Directionnal {
        return Directionnal {x, y, z}
    }
}

impl Default for Directionnal {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}