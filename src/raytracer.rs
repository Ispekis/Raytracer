//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// raytracer
//

use crate::Config::FileConfig;
use crate::RayTracer;
use crate::Math;

fn write_color(color:Math::Vector3D::Vector3D) {
    println!("{} {} {}", color.x as u32, color.y as u32, color.z as u32);
}

pub fn run_raytracer(scene: FileConfig::SceneData) -> u32
{
    let cam = RayTracer::Camera::Camera::default();
    let s = RayTracer::Sphere::Sphere::new(Math::Point3D::Point3D::new(0.0, 0.0, 10.0), 9.0);

    let width = 1000;
    let height = 1000;

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for y in 0..height {
        for x in 0..width {
            let u = x as f64 / (width as f64 - 1.0);
            let v = y as f64 / (height as f64 - 1.0);
            let ray = cam.ray(u, v);
            // println!("ray {} {} {}", ray.direction.x, ray.direction.y, ray.direction.z);
            if s.hits(ray) {
                // println!("hit at u {} and v {}", u, v);
                write_color(Math::Vector3D::Vector3D::new(255.0, 0.0, 0.0));
            } else {
                write_color(Math::Vector3D::Vector3D::new(0.0, 0.0, 255.0));
                // println!("no");
            }
        }
    }
    return 0;
}
