use libc::POLLRDBAND;

mod error_handler;
mod usage;
mod raytracer;
mod Math {
    pub mod Point3D;
    pub mod Vector3D;
    pub mod formulas;
}

extern crate librustconfig;

use librustconfig::config::{Config, ConfigError};
extern crate libc;

pub mod ISolid;
pub mod Rgb;

mod RayTracer {
    pub mod Ray;
    pub mod Sphere;
    pub mod Camera;
    pub mod Plane;
}

mod Parsing {
    pub mod FileConfig;
    pub mod Point;
    pub mod Directionnal;
    pub mod libconfig;
}
mod Rectangle3D;

fn write_color(color:Math::Vector3D::Vector3D) {
    println!("{} {} {}", color.x as u32, color.y as u32, color.z as u32);
}

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 && &args[1] == "--help" {
        usage::display_usage(&args[0]);
        return std::process::ExitCode::SUCCESS;
    }
    if error_handler::error_handler(&args) == 1 {
        return std::process::ExitCode::from(84);
    }
    // raytracer::run_raytracer();

    // let vec1 = Math::Vector3D::Vector3D::new(2.0, 4.0, 0.0);
    // // let vec2 = Math::Vector3D::Vector3D::new(2.0, 4.0, 0.0);
    // let p = Math::Point3D::Point3D::new(1.0, 2.0, 3.0);
    // let sphere = RayTracer::Sphere::Sphere::new(p, 5.0);
    // let ray = RayTracer::Ray::Ray::new(p, vec1);

    // sphere.hits(ray);

    // // println!("{}", Vector3D::length(&vec1));
    // println!("{}", ray.point.x + ray.point.x);

    let cam = RayTracer::Camera::Camera::default();
    let s = RayTracer::Sphere::Sphere::new(Math::Point3D::Point3D::new(0.0, 0.0, -1.0), 0.5);

    let width = 100;
    let height = 50;

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

    return std::process::ExitCode::SUCCESS;
}