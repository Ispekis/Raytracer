//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// raytracer
//

use libc::FILENAME_MAX;

use crate::Parsing::FileConfig::{ParsingFile, ConfigData};

pub fn run_raytracer(filename: &str) -> u32
{
    println!("USAGE:");
    let config = ParsingFile(filename);
    return 0;
}
