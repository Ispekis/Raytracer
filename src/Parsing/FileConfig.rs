//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// FileConfig
//

use serde_json::{Result, Value};
use crate::RayTracer::Camera::Camera;
use crate::RayTracer::Sphere::Sphere;
use crate::tools;

struct primitives {
    spheres:Vec<Sphere>
}

pub struct SceneData {
    camera:Camera,
    primitives:primitives
}

fn convert_string_to_json_obj(str:String) -> Option<Value> {
    let obj: std::result::Result<Value, serde_json::Error> = serde_json::from_str(&str);

    match obj {
        Ok(v) => return Some(v),
        Err(_) => return None,
    }
    // println!("{}", obj["camera"]["resolution"]["width"]);
}

impl SceneData {
    pub fn new(filepath:&str) -> Self{
        let obj = convert_string_to_json_obj(tools::read_file(&filepath));
        println!("{}", obj.unwrap()["camera"]["resolution"]["width"]);
        // SceneData { camera: (), primitives: () }
    }
}

// impl Default for SceneData {
// }