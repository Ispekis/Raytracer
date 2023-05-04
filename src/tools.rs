//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// tools
//

use std::io::Read;

pub fn read_file(filepath:&str) -> String {
    let mut file = std::fs::File::open(&filepath)
    .expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading");
    return data;

}