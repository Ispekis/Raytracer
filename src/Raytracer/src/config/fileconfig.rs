//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// FileConfig
//

use serde_json::{Value};
use crate::interfaces::primitives::Primitives;
use crate::math::{
    point3d::Point3D,
    vector3d::Vector3D
};
use crate::ray_tracer::{
    camera::Camera,
    sphere::Sphere,
    plane::Plane,
    cone::Cone,
    light::{
        Light,
        PointLight
    },
    cylinder::Cylinder,
    material
};
use crate::tools;

pub struct Primitivest {
    pub spheres:Vec<Sphere>,
    pub planes:Vec<Plane>,
    pub cylinders:Vec<Cylinder>
    pub cones:Vec<Cone>
}

pub struct SceneData {
    pub camera:Camera,
    pub primitives:Primitivest,
    pub lights:Light
}

fn convert_string_to_json_obj(str: String) -> std::result::Result<Value, Box<dyn std::error::Error>> {
    let obj = serde_json::from_str(&str)?;
    Ok(obj)
}

fn config_cam(data:&Value) -> std::result::Result<Camera, Box<dyn std::error::Error>> {
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
    let spheres_len = data["primitives"]["spheres"]
    .as_array()
    .map_or(0, |arr| arr.len());
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

        // Set the color
        let mut pattern: Box<dyn material::Mask> = Box::new(material::Solid::new(color));
        if !data["primitives"]["spheres"][i]["pattern"].is_null() {
            let pattern_str = data["primitives"]["spheres"][i]["pattern"].to_string().parse::<String>()?;
            pattern = material::get_material_pattern(pattern_str.as_str());
        }
        pattern.set_color(color);

        // Set the reflectiveness
        let mut reflectiveness:f64 = 0.0;
        if !data["primitives"]["spheres"][i]["material"]["reflectiveness"].is_null() {
            reflectiveness = data["primitives"]["spheres"][i]["material"]["reflectiveness"].to_string().parse::<f64>()?;
        }
        let mut new = Sphere::new_config(position, radius, color, pattern, reflectiveness);

        if !data["primitives"]["spheres"][i]["translation"].is_null() {
            let translation = Vector3D::new(
                data["primitives"]["spheres"][i]["translation"]["x"].to_string().parse::<f64>()?,
                data["primitives"]["spheres"][i]["translation"]["y"].to_string().parse::<f64>()?,
                data["primitives"]["spheres"][i]["translation"]["z"].to_string().parse::<f64>()?);
            new.translate(translation);
        }
        if !data["primitives"]["spheres"][i]["rotation"].is_null() {
            let rotation = Vector3D::new(
                data["primitives"]["spheres"][i]["rotation"]["x"].to_string().parse::<f64>()?,
                data["primitives"]["spheres"][i]["rotation"]["y"].to_string().parse::<f64>()?,
                data["primitives"]["spheres"][i]["rotation"]["z"].to_string().parse::<f64>()?);
                new.rotatex(rotation.x);
                new.rotatey(rotation.y);
                new.rotatez(rotation.z);
        }
        spheres.push(new);
    }

    Ok(spheres)
}

fn config_planes(data:&Value) -> std::result::Result<Vec<Plane>, Box<dyn std::error::Error>> {
    let mut planes: Vec<Plane> = Vec::new();

    let planes_len = data["primitives"]["planes"]
    .as_array()
    .map_or(0, |arr| arr.len());

    for i in 0..planes_len {
        let axis_str = data["primitives"]["planes"][i]["axis"].to_string().parse::<String>()?;
        let axis = axis_str[1..2].chars().next().unwrap();
        let position = data["primitives"]["planes"][i]["position"].to_string().parse::<f64>()?;
        let color = Vector3D::new(
            data["primitives"]["planes"][i]["color"]["r"].to_string().parse::<f64>()?,
            data["primitives"]["planes"][i]["color"]["g"].to_string().parse::<f64>()?,
            data["primitives"]["planes"][i]["color"]["b"].to_string().parse::<f64>()?);

        let mut pattern: Box<dyn material::Mask> = Box::new(material::Solid::new(color));
        if !data["primitives"]["planes"][i]["pattern"].is_null() {
            let pattern_str = data["primitives"]["planes"][i]["pattern"].to_string().parse::<String>()?;
            pattern = material::get_material_pattern(pattern_str.as_str());
        }
        pattern.set_color(color);
        let mut new = Plane::new_config(axis, position, color, pattern);

        if !data["primitives"]["planes"][i]["translation"].is_null() {
            let translation = Vector3D::new(
                data["primitives"]["planes"][i]["translation"]["x"].to_string().parse::<f64>()?,
                data["primitives"]["planes"][i]["translation"]["y"].to_string().parse::<f64>()?,
                data["primitives"]["planes"][i]["translation"]["z"].to_string().parse::<f64>()?);
            new.translate(translation);
        }
        if !data["primitives"]["planes"][i]["rotation"].is_null() {
            let rotation = Vector3D::new(
                data["primitives"]["planes"][i]["rotation"]["x"].to_string().parse::<f64>()?,
                data["primitives"]["planes"][i]["rotation"]["y"].to_string().parse::<f64>()?,
                data["primitives"]["planes"][i]["rotation"]["z"].to_string().parse::<f64>()?);
                new.rotatex(rotation.x);
                new.rotatey(rotation.y);
                new.rotatez(rotation.z);
        }

        planes.push(new);
    }

    Ok(planes)
}

fn config_cylinders(data:&Value) -> std::result::Result<Vec<Cylinder>, Box<dyn std::error::Error>> {
    let mut cylinders: Vec<Cylinder> = Vec::new();

    let cylinders_len = data["primitives"]["cylinders"]
    .as_array()
    .map_or(0, |arr| arr.len());

    for i in 0..cylinders_len {
        let position = Point3D::new(
            data["primitives"]["cylinders"][i]["x"].to_string().parse::<f64>()?,
            data["primitives"]["cylinders"][i]["y"].to_string().parse::<f64>()?,
            data["primitives"]["cylinders"][i]["z"].to_string().parse::<f64>()?);
        let radius = data["primitives"]["cylinders"][i]["r"].to_string().parse::<f64>()?;
        let axis_str = data["primitives"]["cylinders"][i]["axis"].to_string().parse::<String>()?;
        let axis = axis_str[1..2].chars().next().unwrap();
        let height = data["primitives"]["cylinders"][i]["h"].to_string().parse::<f64>()?;
        let color = Vector3D::new(
            data["primitives"]["cylinders"][i]["color"]["r"].to_string().parse::<f64>()?,
            data["primitives"]["cylinders"][i]["color"]["g"].to_string().parse::<f64>()?,
            data["primitives"]["cylinders"][i]["color"]["b"].to_string().parse::<f64>()?);

        let mut pattern: Box<dyn material::Mask> = Box::new(material::Solid::new(color));
        if !data["primitives"]["cylinders"][i]["pattern"].is_null() {
            let pattern_str = data["primitives"]["cylinders"][i]["pattern"].to_string().parse::<String>()?;
            pattern = material::get_material_pattern(pattern_str.as_str());
        }
        pattern.set_color(color);
        cylinders.push(Cylinder::new_config(position, radius, height, color, axis, pattern));
    }

    Ok(cylinders)
}

fn config_cones(data:&Value) -> std::result::Result<Vec<Cone>, Box<dyn std::error::Error>> {
    let mut cones: Vec<Cone> = Vec::new();

    let cones_len =  data["primitives"]["cones"]
    .as_array()
    .ok_or("Not an array")?.len();

    for i in 0..cones_len {
        let position = Point3D::new(
            data["primitives"]["cones"][i]["x"].to_string().parse::<f64>()?,
            data["primitives"]["cones"][i]["y"].to_string().parse::<f64>()?,
            data["primitives"]["cones"][i]["z"].to_string().parse::<f64>()?);
        let radius = data["primitives"]["cones"][i]["r"].to_string().parse::<f64>()?;
        let axis_str = data["primitives"]["cones"][i]["axis"].to_string().parse::<String>()?;
        let axis = axis_str[1..2].chars().next().unwrap();
        let height = data["primitives"]["cones"][i]["h"].to_string().parse::<f64>()?;
        let color = Vector3D::new(
            data["primitives"]["cones"][i]["color"]["r"].to_string().parse::<f64>()?,
            data["primitives"]["cones"][i]["color"]["g"].to_string().parse::<f64>()?,
            data["primitives"]["cones"][i]["color"]["b"].to_string().parse::<f64>()?);
        let mut pattern: Box<dyn material::Mask> = Box::new(material::Solid::new(color));
        if !data["primitives"]["cones"][i]["pattern"].is_null() {
            let pattern_str = data["primitives"]["cones"][i]["pattern"].to_string().parse::<String>()?;
            pattern = material::get_material_pattern(pattern_str.as_str());
        }
        pattern.set_color(color);
        let mut new = Cone::new_config(position, radius, height, color, axis, pattern);
        cones.push(new);
    }
    Ok(cones)
}

fn config_primitives(data:&Value) -> std::result::Result<Primitivest, Box<dyn std::error::Error>> {
    let spheres = config_spheres(data)?;

    let planes = config_planes(data)?;

    let cylinders = config_cylinders(data)?;

    let cones = config_cones(data)?;

    let cylinders = config_cylinders(data)?;

    Ok(Primitivest {spheres, planes, cones, cylinders})
}

fn config_lights(data:&Value) -> std::result::Result<Light, Box<dyn std::error::Error>> {
    let ambient = data["lights"]["ambient"].to_string().parse::<f64>()?;
    let diffuse = data["lights"]["diffuse"].to_string().parse::<f64>()?;
    let specular = data["lights"]["specular"].to_string().parse::<f64>()?;
    let points_len =  data["lights"]["point"]
    .as_array()
    .ok_or("Not an array")?.len();
    let directionals_len =  data["lights"]["directional"]
    .as_array()
    .ok_or("Not an array")?.len();
    let mut points: Vec<PointLight> = Vec::new();
    let mut directionals: Vec<Vector3D> = Vec::new();

    for i in 0..points_len {
        let point = Point3D::new(
            data["lights"]["point"][i]["x"].to_string().parse::<f64>()?,
            data["lights"]["point"][i]["y"].to_string().parse::<f64>()?,
            data["lights"]["point"][i]["z"].to_string().parse::<f64>()?);

        let mut color = Vector3D::new(255.0, 255.0, 255.0);
        if !data["lights"]["point"][i]["color"].is_null() {
            color = Vector3D::new(
                data["lights"]["point"][i]["color"]["x"].to_string().parse::<f64>()?,
                data["lights"]["point"][i]["color"]["y"].to_string().parse::<f64>()?,
                data["lights"]["point"][i]["color"]["z"].to_string().parse::<f64>()?);
        }
        let intensity;
        if data["lights"]["point"][i]["intensity"].is_null() {
            intensity = 1.0
        } else {
            intensity = data["lights"]["point"][i]["intensity"].to_string().parse::<f64>()?;
        }
        points.push(PointLight { origin: point, color, intensity });
    }

    for i in 0..directionals_len {
        let directional = Vector3D::new(
            data["lights"]["directional"][i]["x"].to_string().parse::<f64>()?,
            data["lights"]["directional"][i]["y"].to_string().parse::<f64>()?,
            data["lights"]["directional"][i]["z"].to_string().parse::<f64>()?);
        directionals.push(directional);
    }
    Ok(Light::new_config(ambient, diffuse, specular, points, directionals))
}

impl SceneData {
    pub fn new(filepath: &str) -> std::result::Result<SceneData, Box<dyn std::error::Error>> {
        // Convert string to json
        let data = convert_string_to_json_obj(tools::read_file(&filepath)?)?;

        // Get camera's configs
        let camera = config_cam(&data)?;

        // Get primitives's configs
        let primitives = config_primitives(&data)?;

        // Get lights's configs
        let lights = config_lights(&data)?;

        Ok(SceneData { camera, primitives, lights })
    }
}
