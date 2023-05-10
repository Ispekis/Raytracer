//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// raytracer
//

use crate::Config;
use crate::Config::FileConfig;
use crate::Interfaces::Primitives::Primitives;
use crate::RayTracer::{
    Light::Light
};
use crate::Math::{self, formulas, Vector3D};

fn write_flat_color(color:Math::Vector3D::Vector3D) {
    println!("{} {} {}", color.x as u32, color.y as u32, color.z as u32);
}

fn write_color(color:Math::Vector3D::Vector3D, light:&mut Light, coeff:f64) {
    let color_light = light.point[0].color * light.diffuse;
    let mut r = color.x + (color_light.x * coeff);
    if (r > 255.0) {
        r = 255.0;
    }
    let mut g = color.y + (color_light.y *  coeff);
    if (g > 255.0) {
        g = 255.0;
    }
    let mut b = color.z + (color_light.z * coeff);
    if (b > 255.0) {
        b = 255.0;
    }
    write_flat_color(Math::Vector3D::Vector3D::new(r, g, b));
}

fn draw_primitives(u:f64, v:f64, scene:&mut FileConfig::SceneData) {
    let ray = scene.camera.ray(u, v);

    // let t:Vec<f64>; 
    for i in 0..scene.primitives.spheres.len() {
        let hit_point = scene.primitives.spheres[i].hits(ray);
        if (hit_point != None) {
            let normal = (hit_point.unwrap() - scene.primitives.spheres[i].center).normalize();
            let light_direction = ((hit_point.unwrap() - scene.lights.point[0].origin)).normalize();
            let d = normal.scal(&(light_direction * -1.0));
            write_color(scene.primitives.spheres[i].color, &mut scene.lights, d);
            return;
        }
    }
    for i in 0..scene.primitives.planes.len() {
        let hit_point = scene.primitives.planes[i].hits(ray);
        if hit_point != None{
            write_flat_color(scene.primitives.planes[i].color);
            return;
        }
    }
    write_flat_color(Math::Vector3D::Vector3D::new(0.0, 0.0, 0.0));
}

pub fn run_raytracer(scene:&mut FileConfig::SceneData) -> u32
{
    let width = scene.camera.width;
    let height = scene.camera.height;

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for y in 0..height {
        for x in 0..width {
            let u = x as f64 / (width as f64 - 1.0);
            let v = y as f64 / (height as f64 - 1.0);
            draw_primitives(u, v, scene);
        }
    }
    return 0;
}
