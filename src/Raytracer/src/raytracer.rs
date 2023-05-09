//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// raytracer
//

use crate::Config::FileConfig;
use crate::Interfaces::Primitives::Primitives;
use crate::RayTracer;
use crate::Math;

fn write_color(color:Math::Vector3D::Vector3D) {
    println!("{} {} {}", color.x as u32, color.y as u32, color.z as u32);
}

fn draw_primitives(u:f64, v:f64, scene:&mut FileConfig::SceneData) {
    let ray = scene.camera.ray(u, v);

    for i in 0..scene.primitives.spheres.len() {
        if scene.primitives.spheres[i].hits(ray) {
            write_color(scene.primitives.spheres[i].color);
            return;
        }
    }
    for i in 0..scene.primitives.planes.len() {
        if scene.primitives.planes[i].hits(ray) {
            write_color(scene.primitives.planes[i].color);
            return;
        }
    }
    write_color(Math::Vector3D::Vector3D::new(0.0, 0.0, 0.0));
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
