use libc::POLLRDBAND;
use solid::plane;

mod error_handler;
mod usage;
mod raytracer;
mod Math {
    pub mod Point3D;
    pub mod Vector3D;
}

mod RayTracer {
    pub mod ray;
    pub mod Sphere;
    pub mod Camera;
}

mod Rectangle3D;

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 && &args[1] == "--help" {
        usage::display_usage(&args[0]);
        return std::process::ExitCode::SUCCESS;
    }
    if error_handler::error_handler(&args) == 1 {
        return std::process::ExitCode::from(84);
    }
    raytracer::run_raytracer();

    let vec1 = Math::Vector3D::Vector3D::new(2.0, 4.0, 0.0);
    let vec2 = Math::Vector3D::Vector3D::new(2.0, 4.0, 0.0);
    let p = Math::Point3D::Point3D::default();

    let ray = RayTracer::ray::ray::new((p), vec1);

    // println!("{}", Vector3D::length(&vec1));
    println!("{}", ray.point.x + ray.point.x);
    return std::process::ExitCode::SUCCESS;
}