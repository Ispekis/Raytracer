//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Isolid
//

use crate::solid::rgb::Rgb;

pub trait Solid {
    fn getColor(&self) -> Rgb;
    // fn 
}