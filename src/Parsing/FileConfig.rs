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
    Plane::Plane,
    Light::Light
};
use crate::tools;

pub struct Primitives {
    pub spheres:Vec<Sphere>,
    pub planes:Vec<Plane>
}

pub struct SceneData {
    pub camera:Camera,
    pub primitives:Primitives,
    pub lights:Light
}

fn convert_string_to_json_obj(str: String) -> std::result::Result<Value, Box<dyn std::error::Error>> {
    let obj = serde_json::from_str(&str)?;
    Ok(obj)
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

fn config_planes(data:&Value) -> std::result::Result<Vec<Plane>, Box<dyn std::error::Error>> {
    let mut planes: Vec<Plane> = Vec::new();

    let planes_len =  data["primitives"]["planes"]
    .as_array()
    .ok_or("Not an array")?.len();

    for i in 0..planes_len {
        let axis_str = data["primitives"]["planes"][i]["axis"].to_string().parse::<String>()?;
        let axis = axis_str.chars().next().unwrap();
        let position = data["primitives"]["planes"][i]["position"].to_string().parse::<f64>()?;
        let color = Vector3D::new(
            data["primitives"]["planes"][i]["color"]["r"].to_string().parse::<f64>()?,
            data["primitives"]["planes"][i]["color"]["g"].to_string().parse::<f64>()?,
            data["primitives"]["planes"][i]["color"]["b"].to_string().parse::<f64>()?);

        planes.push(Plane::new_config(axis, position, color));
    }

    Ok(planes)
}

fn config_primitives(data:&Value) -> std::result::Result<Primitives, Box<dyn std::error::Error>> {
    
    let spheres = config_spheres(data)?;
    
    let planes = config_planes(data)?;

    Ok(Primitives {spheres, planes})
}

fn config_lights(data:&Value) -> std::result::Result<Light, Box<dyn std::error::Error>> {
    let ambient = data["lights"]["ambient"].to_string().parse::<f64>()?;
    let diffuse = data["lights"]["diffuse"].to_string().parse::<f64>()?;
    let points_len =  data["lights"]["point"]
    .as_array()
    .ok_or("Not an array")?.len();
    let directionals_len =  data["lights"]["directional"]
    .as_array()
    .ok_or("Not an array")?.len();
    let mut points: Vec<Point3D> = Vec::new();
    let mut directionals: Vec<Vector3D> = Vec::new();

    for i in 0..points_len {
        let point = Point3D::new(
            data["lights"]["point"][i]["x"].to_string().parse::<f64>()?,
            data["lights"]["point"][i]["y"].to_string().parse::<f64>()?,
            data["lights"]["point"][i]["z"].to_string().parse::<f64>()?);
        points.push(point);
    }

    for i in 0..directionals_len {
        let directional = Vector3D::new(
            data["lights"]["directional"][i]["x"].to_string().parse::<f64>()?,
            data["lights"]["directional"][i]["y"].to_string().parse::<f64>()?,
            data["lights"]["directional"][i]["z"].to_string().parse::<f64>()?);
        directionals.push(directional);
    }
    Ok(Light::new_config(ambient, diffuse, points, directionals))
}

impl SceneData {
    pub fn new(filepath:&str) -> std::result::Result<SceneData, Box<dyn std::error::Error>> {
        // Convert string to json
        let data = convert_string_to_json_obj(tools::read_file(&filepath)?)?;
        
        // Get camera's configs
        let camera = config_cam(&data)?;
        
        // Get primitives's configs
        let primitives = config_primitives(&data)?;
        
        // Get lights's configs
        let lights = config_lights(&data)?;

        Ok(SceneData {camera, primitives, lights })
    }
}