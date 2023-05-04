//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// libconfig
//

use std::ffi::c_void;

extern "C" {
    fn config_init() -> *mut c_void;
}