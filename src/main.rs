use libc::POLLRDBAND;

mod error_handler;
mod usage;
mod raytracer;
mod Math {
    pub mod Point3D;
    pub mod Vector3D;
}

pub mod ISolid;
pub mod Rgb;

mod RayTracer {
    pub mod Ray;
    pub mod Sphere;
    pub mod Camera;
    pub mod Plane;
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
    // let vec2 = Math::Vector3D::Vector3D::new(2.0, 4.0, 0.0);
    let p = Math::Point3D::Point3D::new(1.0, 2.0, 3.0);
    let sphere = RayTracer::Sphere::Sphere::new(p, 5.0, Rgb::Rgb::default());
    let ray = RayTracer::Ray::Ray::new(p, vec1);

    sphere.hits(ray);

    // println!("{}", Vector3D::length(&vec1));
    println!("{}", ray.point.x + ray.point.x);
    return std::process::ExitCode::SUCCESS;
}