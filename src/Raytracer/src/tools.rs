//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// tools
//

use std::io::Read;

pub fn read_file(filepath:&str) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(&filepath)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}