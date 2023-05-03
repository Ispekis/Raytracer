//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// rgb
//

#[derive(Copy, Clone)]
pub struct Rgb {
    pub r:u32,
    pub g:u32,
    pub b:u32,
}

impl Rgb {
    // pub fn new_default() -> Rgb {
    //     return Rgb {r:0, g:0, b:0};
    // }

    pub fn new(r:u32, g:u32, b:u32) -> Rgb {
        return Rgb {r, g, b};
    }
}

impl Default for Rgb {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}
