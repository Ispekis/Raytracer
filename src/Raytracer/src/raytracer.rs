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
    let mut r = color.x * light.ambient + (color_light.x * coeff);
    let mut g = color.y * light.ambient + (color_light.y * coeff);
    let mut b = color.z * light.ambient + (color_light.z * coeff);

    write_flat_color(Math::Vector3D::Vector3D::new(r, g, b));
}

fn draw_primitives(u:f64, v:f64, scene:&mut FileConfig::SceneData) {
    let ray = scene.camera.ray(u, v);

    for i in 0..scene.primitives.spheres.len() {
        let hit_point = scene.primitives.spheres[i].hits(ray);
        if (hit_point != None) {
            let normal = (hit_point.unwrap() - scene.primitives.spheres[i].center).normalize();
            let light_direction = ((hit_point.unwrap() - scene.lights.point[0].origin)).normalize();
            let mut d = normal.scal(&(light_direction * -1.0));
            if (d < 0.0) {
                d = 0.0;
            }
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
