//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// FileConfig
//


use std::ffi::c_void;
// use libc::{Config, ConfigOption};
extern crate librustconfig;

// use librustconfig::{Config, ConfigError};
use crate::RayTracer::Plane::Plane;
use crate::RayTracer::Sphere::Sphere;
use crate::Parsing::Directionnal::Directionnal;
use crate::Parsing::Point::Point;
pub struct Primitives {
    planes: Vec<Plane>,
    sphere: Vec<Sphere>,
}

pub struct Lights {
    ambient: f32,
    diffuse: f32,

    points: Vec<Point>,
    directionnals: Vec<Directionnal>,
}

pub struct ConfigData {
    primitives: Primitives,
    lights: Lights,
}

// pub fn GetConfig(filename: &str) -> *mut c_void {
//     unsafe {
//         let config = config_init();
//     }
// }
// pub fn InitPrimitive()
pub fn ParsingFile(filename: &str) -> u32
{
    // let primitive = 
    // let config = ConfigData { primitives: (), lights: () };
    // return config;
    return 0;
}
