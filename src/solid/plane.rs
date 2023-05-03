//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// planes
//

// mod rgb;
// mod solid {
//     mod position;
// }

use crate::solid::rgb::Rgb;
// use crate::solid::plane;
// use crate::position::Position;


pub struct Plane {
    pub color:Rgb,
    pub axis:String,
}

impl Plane {
    pub fn new_default() -> Plane {
        return Plane {color:Rgb::default(), axis:String::new()}
    }
    pub fn new(r:u32, g:u32, b:u32, axis:String) -> Plane {
        return Plane {color:Rgb::new(r, g, b), axis}
    }
}

