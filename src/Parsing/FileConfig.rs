//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// FileConfig
//

use serde_json::{Result, Value};
use crate::Math::{
    Point3D::Point3D,
    Vector3D::Vector3D
};
use crate::RayTracer::{
    Camera::Camera,
    Sphere::Sphere,
    Plane::Plane
};
use crate::tools;

pub struct Primitives {
    pub spheres:Vec<Sphere>,
    // pub planes:Vec<Plane>
}

pub struct SceneData {
    pub camera:Camera,
    pub primitives:Primitives
}

fn convert_string_to_json_obj(str:String) -> Option<Value> {
    let obj: std::result::Result<Value, serde_json::Error> = serde_json::from_str(&str);

    match obj {
        Ok(v) => return Some(v),
        Err(_) => return None,
    }
    // println!("{}", obj["camera"]["resolution"]["width"]);
}

fn config_cam(data:&Value) -> std::result::Result<(Camera), Box<dyn std::error::Error>> {
    let width = data["camera"]["resolution"]["width"].to_string().parse::<u32>()?;

    let height = data["camera"]["resolution"]["height"].to_string().parse::<u32>()?;

    let position = Point3D::new(
        data["camera"]["position"]["x"].to_string().parse::<f64>()?,
        data["camera"]["position"]["y"].to_string().parse::<f64>()?,
        data["camera"]["position"]["z"].to_string().parse::<f64>()?);

    let rotation = Vector3D::new(
        data["camera"]["rotation"]["x"].to_string().parse::<f64>()?,
        data["camera"]["rotation"]["x"].to_string().parse::<f64>()?,
        data["camera"]["rotation"]["x"].to_string().parse::<f64>()?);

    let fov = data["camera"]["fieldOfView"].to_string().parse::<f64>()?;

    Ok(Camera::new_config(width, height, position, rotation, fov))
}

fn config_spheres(data:&Value) -> std::result::Result<Vec<Sphere>, Box<dyn std::error::Error>> {
    let spheres_len =  data["primitives"]["spheres"]
    .as_array()
    .ok_or("Not an array")?.len();
    let mut spheres: Vec<Sphere> = Vec::new();

    for i in 0..spheres_len {
        let position = Point3D::new(
            data["primitives"]["spheres"][i]["x"].to_string().parse::<f64>()?,
            data["primitives"]["spheres"][i]["y"].to_string().parse::<f64>()?,
            data["primitives"]["spheres"][i]["z"].to_string().parse::<f64>()?);

        let radius = data["primitives"]["spheres"][i]["r"].to_string().parse::<f64>()?;

        let color = Vector3D::new(
            data["primitives"]["spheres"][i]["color"]["r"].to_string().parse::<f64>()?,
            data["primitives"]["spheres"][i]["color"]["g"].to_string().parse::<f64>()?,
            data["primitives"]["spheres"][i]["color"]["b"].to_string().parse::<f64>()?);

        spheres.push(Sphere::new_config(position, radius, color));
    }

    Ok((spheres))
}

// fn config_planes(data:&Value) -> std::result::Result<Primitives, Box<dyn std::error::Error>> {

// }

fn config_primitives(data:&Value) -> std::result::Result<Primitives, Box<dyn std::error::Error>> {

    let spheres = config_spheres(data)?;

    // let panes = 

    Ok(Primitives {spheres})
}

impl SceneData {
    pub fn new(filepath:&str) -> std::result::Result<SceneData, Box<dyn std::error::Error>> {
        let obj = convert_string_to_json_obj(tools::read_file(&filepath));

        if obj == None {
            println!("Error on parsing config file");
        }
        let data = obj.unwrap();

        let cam = config_cam(&data)?;
        let prim = config_primitives(&data)?;

        Ok(SceneData {camera: cam, primitives: prim })
    }
}

// impl Default for SceneData {
// }